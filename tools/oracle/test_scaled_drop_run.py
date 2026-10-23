"""Receipt validation tests using generated packages, never oracle outputs."""
from pathlib import Path
import copy
import json
import sys
import tempfile
import unittest
import zipfile
sys.path.insert(0, str(Path.cwd()/'tools/oracle'))
import accept_report as a
import scaled_drop_run as r


class NativeRun(unittest.TestCase):
    def setUp(self):
        root = a.ROOT/'target/oracle-tests'
        root.mkdir(parents=True, exist_ok=True)
        tmp = tempfile.TemporaryDirectory(dir=root)
        self.addCleanup(tmp.cleanup)
        self.root = Path(tmp.name)
        self.inputs = self.root/'inputs'
        self.out = self.root/'out'
        self.inputs.mkdir()
        self.out.mkdir()
        self.script = self.root/'script.ps1'
        self.common = self.root/'common.ps1'
        self.script.write_text('# synthetic capture identity\n')
        self.common.write_text('# synthetic helper identity\n')
        (self.out/'capture.ps1').write_bytes(self.script.read_bytes())
        (self.out/'common.ps1').write_bytes(self.common.read_bytes())
        self.run = dict(run_id='a'*32,names=[r.BLANK,*r.CASES],completed=True,
                        started_utc='2026-10-23T01:00:00Z',finished_utc='2026-10-23T01:01:00Z',
                        visio_version='test',visio_build='test',user='test',machine='test',
                        script_sha256=a.digest(self.script),common_sha256=a.digest(self.common))
        self.rows = []
        for name in self.run['names']:
            directory = self.out/name
            directory.mkdir()
            self.package(self.inputs/(name+'.vsdx'),1614)
            (directory/'input.vsdx').write_bytes((self.inputs/(name+'.vsdx')).read_bytes())
            self.package(directory/'saved.vsdx',1350)
            for file in ['observed.sheet.tsv','connects.tsv']:
                (directory/file).write_text('synthetic receipt\n')
            self.rows.append(dict(name=name,run_id=self.run['run_id'],opened=True,saved=True,error=None,
                input_sha256=a.digest(self.inputs/(name+'.vsdx')),
                script_sha256=self.run['script_sha256'],common_sha256=self.run['common_sha256'],
                artifacts=[dict(file=f,sha256=a.digest(directory/f)) for f in ['saved.vsdx','observed.sheet.tsv','connects.tsv']]))
        self.flush()

    def package(self,path,height,shape=False):
        with zipfile.ZipFile(path,'w') as z:
            content = '<Shapes><Shape ID="1" Type="Shape"/></Shapes>' if shape else '<Shapes/>'
            z.writestr('visio/pages/page1.xml','<PageContents xmlns="http://schemas.microsoft.com/office/visio/2012/main">'+content+'</PageContents>')
            z.writestr('visio/windows.xml',f'<Windows xmlns="http://schemas.microsoft.com/office/visio/2012/main" ClientHeight="{height}"/>')

    def flush(self):
        (self.out/'run.json').write_text(json.dumps(self.run))
        (self.out/'status.json').write_text(json.dumps(self.rows))

    def verify(self):
        return r.verify_run(self.inputs,self.out,self.script,self.common)

    def test_complete_same_run_and_exact_names(self):
        self.assertEqual(len(self.verify()[1]),14)
        original = copy.deepcopy(self.rows)
        for changed in [original[:-1],original+[original[-1]],original[:-1]+[original[-2]]]:
            self.rows = changed
            self.flush()
            with self.assertRaises(ValueError): self.verify()
        self.rows = copy.deepcopy(original)
        self.rows[0]['run_id'] = 'b'*32
        self.flush()
        with self.assertRaises(ValueError): self.verify()

    def test_stale_or_missing_control_and_changed_artifacts_fail(self):
        for path in [self.out/r.BLANK/'saved.vsdx',self.out/r.BLANK/'input.vsdx',
                     self.inputs/(r.BLANK+'.vsdx'),self.script,self.out/'common.ps1',
                     self.out/r.CASES[0]/'saved.vsdx']:
            original = path.read_bytes()
            path.write_bytes(original+b'changed')
            with self.assertRaises(ValueError): self.verify()
            path.unlink()
            with self.assertRaises(FileNotFoundError): self.verify()
            path.write_bytes(original)
        self.verify()

    def test_hostile_or_incomplete_receipt_fails(self):
        original = copy.deepcopy(self.rows)
        for key,value in [('artifacts',[dict(file='../outside',sha256='x')]),('opened',False),('opened','false'),('saved',1),('saved',False),('error','native failure')]:
            self.rows = copy.deepcopy(original)
            self.rows[0][key] = value
            self.flush()
            with self.assertRaises(ValueError): self.verify()
        (self.out/'status.json').write_bytes(b'['*10000+b']'*10000)
        with self.assertRaises(ValueError): self.verify()

    def test_incomplete_run_or_nonblank_control_is_rejected(self):
        for value in [False, 'false', 1]:
            self.run['completed'] = value
            self.flush()
            with self.assertRaises(ValueError): self.verify()
        self.run['completed'] = True
        path = self.inputs/(r.BLANK+'.vsdx')
        self.package(path,1614,shape=True)
        (self.out/r.BLANK/'input.vsdx').write_bytes(path.read_bytes())
        self.rows[0]['input_sha256'] = a.digest(path)
        self.flush()
        with self.assertRaisesRegex(ValueError,'contains page shapes'): self.verify()

    def test_only_window_control_is_replaced(self):
        window=(str(('visio/windows.xml/Windows:0/Attribute:ClientHeight',0)),'V','old','old')
        document=(str(('visio/document.xml/DocumentSheet/Cell:DocLangID',0)),'V','en','fr')
        result,fresh=r.window_control([window,document],self.out,set())
        self.assertEqual(len(fresh),2)  # Exact attribute and opaque part hash.
        attribute = next(d for d in fresh if 'Attribute:ClientHeight' in d[0])
        self.assertEqual(attribute[2:],('1614','1350'))
        self.assertIn(document,result)
        self.assertNotIn(window,result)
        self.assertEqual(result,[document,*fresh])


if __name__ == '__main__': unittest.main()
