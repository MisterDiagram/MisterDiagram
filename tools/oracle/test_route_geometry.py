"""Boundary tests for the named router-owned geometry acceptance rule."""
import copy
from pathlib import Path
import sys
import tempfile
import unittest
import zipfile
sys.path.insert(0, str(Path(__file__).resolve().parent))
import accept_report as report
import route_geometry as route


class RouterGeometry(unittest.TestCase):
    def setUp(self):
        root = report.ROOT/'target/oracle-tests'
        root.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=root)
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)

    def package(self, name, obj='2', fixed='0', value='1', extra='', section='0', kind='LineTo', formula='', inherited=False):
        flags = f'<Cell N="ObjType" V="{obj}"/><Cell N="ConFixedCode" V="{fixed}"/>'
        endpoints = ''.join(f'<Cell N="{n}" V="0"/>' for n in ('BeginX','BeginY','EndX','EndY'))
        attrs = ' Master="7"' if inherited else ''
        geometry = f'<Section N="Geometry" IX="{section}"><Row IX="1" T="{kind}"><Cell N="X" V="{value}"{formula}/><Cell N="Y" V="0"/></Row></Section>'
        xml = f'<PageContents xmlns="{route.NS}"><Shapes><Shape ID="1" Type="Shape"{attrs}>{"" if inherited else flags+endpoints}{geometry}{extra}</Shape></Shapes></PageContents>'
        path = self.root/name
        with zipfile.ZipFile(path,'w') as z:
            z.writestr('visio/pages/page1.xml',xml)
            if inherited:
                z.writestr('visio/masters/masters.xml', f'<Masters xmlns="{route.NS}" xmlns:r="{route.REL}"><Master ID="7"><Rel r:id="r7"/></Master></Masters>')
                z.writestr('visio/masters/_rels/masters.xml.rels', f'<Relationships xmlns="{route.PKG}"><Relationship Id="r7" Type="http://schemas.microsoft.com/visio/2010/relationships/master" Target="unrelated-name.xml"/></Relationships>')
                z.writestr('visio/masters/unrelated-name.xml',f'<MasterContents xmlns="{route.NS}"><Shapes><Shape ID="42" Type="Shape">{flags}{endpoints}</Shape></Shapes></MasterContents>')
        return path

    def compare(self, before, after):
        _, a = report.package_cells(before)
        _, b = report.package_cells(after)
        return route.judge(before,after,a,b,report.cell_diffs(a,b,set()))

    def test_numeric_route_bound_is_absolute_and_counted(self):
        a=self.package('a.vsdx')
        b=self.package('b.vsdx',value='1.0000005')
        remaining, judged=self.compare(a,b)
        self.assertEqual(remaining,[])
        self.assertEqual(judged['cells_judged'],2)
        self.assertEqual(judged['changed_within_bound'],1)
        b=self.package('b.vsdx',value='1.000002')
        remaining, judged=self.compare(a,b)
        self.assertTrue(remaining,'over-bound route must remain a repair')
        self.assertEqual(judged['outside_bound'],1)
        self.assertEqual(route.number('NaN'),None)
        self.assertEqual(route.number('inf'),None)
        a=self.package('a.vsdx',value='1000000')
        b=self.package('b.vsdx',value='1000000.0001')
        self.assertTrue(self.compare(a,b)[0], 'relative tolerance is forbidden')

    def test_non_connector_fixed_and_other_sections_stay_exact(self):
        for options in ({'obj':'1'},{'fixed':'2'},{'section':'1'},{'kind':'Ellipse'},{'kind':'RelCubBezTo'}):
            with self.subTest(options=options):
                a=self.package('a.vsdx',**options)
                b=self.package('b.vsdx',value='1.0000000001',**options)
                remaining, judged=self.compare(a,b)
                self.assertTrue(remaining,'non-owned geometry must remain an exact repair')
                self.assertEqual(judged['cells_judged'],0)

    def test_inherited_flags_follow_relationship_not_filename(self):
        a=self.package('a.vsdx',inherited=True)
        b=self.package('b.vsdx',inherited=True,value='1.0000000001')
        self.assertEqual(self.compare(a,b)[0],[])
        b=self.package('b.vsdx',inherited=True,value='1.0000000001',obj='1')
        self.assertTrue(self.compare(a,b)[0])
        self.assertEqual(self.compare(a,b)[1]['cells_judged'],0)

    def test_formulas_units_structure_and_nonroute_cells_are_exact(self):
        a=self.package('a.vsdx')
        for options in ({'formula':' F="1"'},{'formula':' U="IN"'},{'formula':' E="#VALUE!"'},
                        {'extra':'<Cell N="Width" V="1.0000000001"/>'},
                        {'kind':'MoveTo'},{'section':'1'},
                        {'extra':'<Section N="Geometry" IX="0"/>'}):
            with self.subTest(options=options):
                b=self.package('b.vsdx',value='1.0000000001',**options)
                self.assertTrue(self.compare(a,b)[0])
        b=self.package('b.vsdx',formula=' F="1"',value='1.0000000001')
        self.assertTrue(any(d[1]=='F' for d in self.compare(a,b)[0]))

    def test_exact_neighbors_missing_cells_and_invalid_numbers(self):
        a=self.package('a.vsdx',value='0')
        b=self.package('b.vsdx',value='0.000001')
        self.assertEqual(self.compare(a,b)[0],[])
        self.assertTrue(report.cell_diffs(report.package_cells(a)[1],report.package_cells(b)[1],set()))
        for value in ('NaN','inf','1e9999','Themed',''):
            b=self.package('b.vsdx',value=value)
            self.assertTrue(self.compare(a,b)[0])
        _, old=report.package_cells(a)
        row='visio/pages/page1.xml/Shape:1/Section:Geometry:0/Row::1'
        x=(row+'/Cell:X',0)
        for change in ('missing-cell','missing-row','missing-section','row-deleted','formula','unit','error'):
            new=copy.deepcopy(old)
            new[x]['V']='0.0000001'
            if change=='missing-cell': del new[x]
            elif change=='missing-row': del new[(row+'/Present',0)]
            elif change=='missing-section': del new[(row.rsplit('/Row:',1)[0]+'/Present',0)]
            elif change=='row-deleted': new[(row+'/Attribute:Del',0)]={'V':'1','F':None,'U':None,'E':None,'kind':'attribute'}
            else: new[x][{'formula':'F','unit':'U','error':'E'}[change]]='1'
            remaining, _=route.judge(a,a,old,new,report.cell_diffs(old,new,set()))
            self.assertTrue(remaining,change)
        for suffix in ('/Cell:PinX','/Cell:Width','/Cell:BeginX','/Section:Control:/Row:TextPosition:/Cell:Y','/Section:Connection:/Row::0/Cell:X','/Section:Geometry:0/Cell:NoShow'):
            key=('visio/pages/page1.xml/Shape:1'+suffix,0)
            old[key]={'V':'0','F':None,'U':None,'E':None}
            new=copy.deepcopy(old);new[key]['V']='0.0000001'
            self.assertTrue(route.judge(a,a,old,new,report.cell_diffs(old,new,set()))[0])

    def test_ownership_overrides_and_unknown_master_fail_closed(self):
        a=self.package('a.vsdx',inherited=True)
        for extra in ('<Cell N="ConFixedCode" V="2"/>','<Cell N="ObjType" V="2" F="1"/>'):
            b=self.package('b.vsdx',inherited=True,value='1.0000001',extra=extra)
            self.assertEqual(self.compare(a,b)[1]['cells_judged'],0)
        source=self.package('source.vsdx',inherited=True)
        with zipfile.ZipFile(source) as z: parts={n:z.read(n) for n in z.namelist()}
        relname='visio/masters/_rels/masters.xml.rels'
        for replace in (b'Target="../unrelated-name.xml"',b'Target="unrelated-name.xml" TargetMode="External"',b'Target="missing.xml"'):
            b=self.root/'bad-master.vsdx'
            with zipfile.ZipFile(b,'w') as z:
                for n,v in parts.items():z.writestr(n,v.replace(b'Target="unrelated-name.xml"',replace) if n==relname else v)
            self.assertEqual(route.owned_shapes(b),set())

    def test_document_defaults_precede_style_zero_and_selected_styles(self):
        source=self.package('source.vsdx')
        with zipfile.ZipFile(source) as z: parts={n:z.read(n) for n in z.namelist()}
        for document_flag, default_flag, expected in (('2','0',False),('0','2',True),(None,'2',False)):
            path=self.root/'defaults.vsdx'
            page=parts['visio/pages/page1.xml'].replace(b'<Cell N="ConFixedCode" V="0"/>',b'').replace(b'Type="Shape"',b'Type="Shape" LineStyle="7" FillStyle="7" TextStyle="7"')
            doccell='' if document_flag is None else f'<Cell N="ConFixedCode" V="{document_flag}"/>'
            document=f'<VisioDocument xmlns="{route.NS}"><DocumentSheet>{doccell}</DocumentSheet><StyleSheets><StyleSheet ID="0"><Cell N="ConFixedCode" V="{default_flag}"/></StyleSheet><StyleSheet ID="7"><Cell N="ConFixedCode" V="0"/></StyleSheet></StyleSheets></VisioDocument>'
            with zipfile.ZipFile(path,'w') as z:
                z.writestr('visio/pages/page1.xml',page);z.writestr('visio/document.xml',document)
            self.assertEqual(bool(route.owned_shapes(path)),expected,'ownership uses document before style0, never selected styles')

    def test_casefold_duplicates_and_foreign_geometry_stay_exact(self):
        a=self.package('a.vsdx',extra='<Cell N="objtype" V="1"/>')
        b=self.package('b.vsdx',extra='<Cell N="objtype" V="1"/>',value='1.0000001')
        self.assertEqual(self.compare(a,b)[1]['cells_judged'],0)
        for label,value in (('foreign-a','1'),('foreign-b','1.0000001')):
            source=self.package(label+'-source.vsdx',value=value)
            with zipfile.ZipFile(source) as z: data=z.read('visio/pages/page1.xml')
            data=data.replace(b'<Section N="Geometry"',b'<Section xmlns="urn:extension" N="Geometry"')
            with zipfile.ZipFile(self.root/(label+'.vsdx'),'w') as z:z.writestr('visio/pages/page1.xml',data)
        remaining,judged=self.compare(self.root/'foreign-a.vsdx',self.root/'foreign-b.vsdx')
        self.assertTrue(remaining,'foreign geometry is not router owned')
        self.assertEqual(judged['cells_judged'],0)

    def test_page_wide_numeric_shape_id_collisions_are_ambiguous(self):
        for extra in ('<Shape ID="01" Type="Shape"/>','<Shape ID="2" Type="Group"><Shapes><Shape ID="1" Type="Shape"/></Shapes></Shape>'):
            source=self.package('source.vsdx')
            with zipfile.ZipFile(source) as z: data=z.read('visio/pages/page1.xml')
            data=data.replace(b'</Shapes></PageContents>',extra.encode()+b'</Shapes></PageContents>')
            path=self.root/'duplicate.vsdx'
            with zipfile.ZipFile(path,'w') as z:z.writestr('visio/pages/page1.xml',data)
            self.assertEqual(route.owned_shapes(path),set(),'page-wide numeric IDs must be unique')

    def test_numeric_geometry_aliases_and_master_ambiguity_fail_closed(self):
        for edit in ('section','row','master-id','master-chain'):
            paths=[]
            for label,value in (('a','1'),('b','1.0000001')):
                source=self.package(label+'-source.vsdx',value=value,inherited=True)
                with zipfile.ZipFile(source) as z: parts={n:z.read(n) for n in z.namelist()}
                if edit=='section': parts['visio/pages/page1.xml']=parts['visio/pages/page1.xml'].replace(b'</Shape>',b'<Section N="Geometry" IX="00"/></Shape>')
                elif edit=='row': parts['visio/pages/page1.xml']=parts['visio/pages/page1.xml'].replace(b'</Section>',b'<Row IX="01" T="LineTo"/></Section>')
                elif edit=='master-id': parts['visio/masters/unrelated-name.xml']=parts['visio/masters/unrelated-name.xml'].replace(b'</Shape>',b'<Shapes><Shape ID="42" Type="Shape"/></Shapes></Shape>')
                else: parts['visio/masters/unrelated-name.xml']=parts['visio/masters/unrelated-name.xml'].replace(b'ID="42"',b'ID="42" Master="99"')
                path=self.root/(label+'.vsdx');paths.append(path)
                with zipfile.ZipFile(path,'w') as z:
                    for n,v in parts.items():z.writestr(n,v)
            remaining,judged=self.compare(*paths)
            self.assertTrue(remaining,edit)
            self.assertEqual(judged['cells_judged'],0,edit)

    def test_hostile_and_ambiguous_ownership_fails_closed(self):
        for extra in ('<Cell N="ObjType" V="2"/>','<Cell N="ObjType" V="1" F="2"/>'):
            a=self.package('a.vsdx',extra=extra)
            b=self.package('b.vsdx',extra=extra,value='1.0000000001')
            self.assertEqual(self.compare(a,b)[1]['cells_judged'],0)
        a=self.package('a.vsdx',obj='NaN')
        self.assertEqual(route.owned_shapes(a),set())
        path=self.root/'bad.vsdx'
        with zipfile.ZipFile(path,'w') as z:
            z.writestr('visio/pages/page1.xml','<!DOCTYPE x [<!ENTITY e "x">]><x/>')
        with self.assertRaisesRegex(ValueError,'unsafe XML'):
            route.owned_shapes(path)
        with zipfile.ZipFile(path,'w') as z:
            z.writestr('visio/pages/page1.xml','<x>'*130+'</x>'*130)
        with self.assertRaisesRegex(ValueError,'tree budget'):
            route.owned_shapes(path)


if __name__ == '__main__':
    unittest.main()
