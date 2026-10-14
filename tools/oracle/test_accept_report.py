"""Host-only acceptance parser and comparison regressions. Never edits oracle evidence."""
import sys
from pathlib import Path
import tempfile
import unittest
import zipfile
sys.path.insert(0, str(Path(__file__).resolve().parent))
import accept_report as report

class Acceptance(unittest.TestCase):
    def setUp(self):
        root = report.ROOT/'target/oracle-tests'
        root.mkdir(parents=True, exist_ok=True)
        self.tmp = tempfile.TemporaryDirectory(dir=root)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)

    def package(self, xml):
        path = self.root/'test.vsdx'
        with zipfile.ZipFile(path, 'w') as z:
            z.writestr('visio/document.xml', xml)
        return path

    def test_connects_are_required_and_bounded(self):
        p = self.root/'connects.tsv'
        header = 'from_sheet\tfrom_cell\tto_sheet\tto_cell\tfrom_part\tto_part\n'
        p.write_text(header+'3\tBeginX\t1\tPinX\t9\t3\n3\tEndX\t2\tPinX\t12\t3\n')
        self.assertEqual(len(report.read_connects(p)), 2)
        for bad in ['wrong header', header+'-1\tBeginX\t1\tPinX\t9\t3\n', header+'x'*(8*1024*1024)]:
            p.write_text(bad)
            with self.assertRaises(ValueError): report.read_connects(p)

    def test_hostile_json_is_bounded(self):
        path=self.root/'bad.json'
        for payload in [b'x'*(4*1024*1024+1), b'['*10000+b']'*10000, b'\xff']:
            path.write_bytes(payload)
            with self.assertRaises(ValueError): report.load_json(path)

    def test_serialized_cache_change_survives_formula_equality(self):
        _, a = report.package_cells(self.package('<DocumentSheet><Cell N="X" V="1" F="2"/></DocumentSheet>'))
        _, b = report.package_cells(self.package('<DocumentSheet><Cell N="X" V="2" F="2"/></DocumentSheet>'))
        self.assertEqual(len(report.cell_diffs(a,b,set())),1)

    def test_duplicates_missing_and_volatile_are_explicit(self):
        _, a = report.package_cells(self.package('<DocumentSheet><Cell N="X" V="1"/><Cell N="X" V="2"/><Cell N="Y" V="3" F="NOW()"/></DocumentSheet>'))
        self.assertEqual(len(a),4)
        self.assertEqual(len(report.cell_diffs(a,{}, {'NOW'})),3)
        self.assertEqual(len(report.cell_diffs({},a, {'NOW'})),3)

    def test_opaque_solution_payload_has_part_and_digest_records(self):
        path = self.package('<DocumentSheet><Cell N="X" V="1"/></DocumentSheet>')
        with zipfile.ZipFile(path,'a') as z:
            z.writestr('visio/solutions/solution1.xml','<vendor:opaque/>')
        parts,cells = report.package_cells(path)
        self.assertIn('visio/solutions/solution1.xml',parts)
        self.assertEqual(len(cells),3)
        self.assertIn(('visio/solutions/solution1.xml/Opaque',0),cells)

    def test_opc_metadata_byte_hashes_remain_in_acceptance(self):
        a=report.metadata_records(b'<Types X="1"/>','[Content_Types].xml')
        b=report.metadata_records(b'<Types X="2"/>','[Content_Types].xml')
        self.assertTrue(report.cell_diffs(a,b,set()))
        p=self.package('<VisioDocument/>')
        with zipfile.ZipFile(p,'a') as z:z.writestr('[Content_Types].xml','<Types/>')
        _,cells=report.package_cells(p)
        self.assertIn(('[Content_Types].xml/Opaque',0),cells)

    def test_lost_connectivity_is_visible(self):
        _, a = report.package_cells(self.package('<PageContents><Connects><Connect FromSheet="3" FromCell="BeginX" ToSheet="1" ToCell="PinX"/></Connects></PageContents>'))
        _, b = report.package_cells(self.package('<PageContents/>'))
        self.assertEqual(len(report.cell_diffs(a,b,set())),1)

    def test_hostile_xml_and_duplicate_parts_rejected(self):
        for xml in ['<!DOCTYPE x [<!ENTITY x "boom">]><x>&x;</x>', '<x>'*130+'</x>'*130]:
            with self.assertRaises(ValueError):
                report.package_cells(self.package(xml))
        path=self.package('<x/>')
        import warnings
        with warnings.catch_warnings():
            warnings.simplefilter('ignore')
            with zipfile.ZipFile(path,'a') as z:
                z.writestr('visio/document.xml','<x/>')
        with self.assertRaises(ValueError):
            report.package_cells(path)

    def test_acceptance_tolerates_one_but_rejects_two_pixel_mismatches(self):
        from PIL import Image
        a,b=self.root/'a.png',self.root/'b.png'
        Image.new('RGB',(4,4),'white').save(a)
        Image.new('RGB',(5,4),'white').save(b)
        result=report.pixels(a,b)
        self.assertEqual(result['mad'],0)
        self.assertEqual(result['pixels'],16)
        Image.new('RGB',(6,4),'white').save(b)
        self.assertEqual(report.pixels(a,b)['error'],'hard size mismatch')


    def test_cache_formatting_is_a_literal_difference(self):
        _,a=report.package_cells(self.package('<DocumentSheet><Cell N="X" V="1" F="2"/></DocumentSheet>'))
        _,b=report.package_cells(self.package('<DocumentSheet><Cell N="X" V="1.0" F="2"/></DocumentSheet>'))
        self.assertEqual(len(report.cell_diffs(a,b,set())),1)

    def test_attributes_text_icons_and_opaque_bytes_are_compared(self):
        before='<PageContents><Shapes><Shape ID="1" Type="Shape"><Section N="Geometry" IX="0"><Row IX="1" T="MoveTo" Del="0"/></Section><Text>Hello</Text><Icon>abc</Icon></Shape></Shapes></PageContents>'
        after=before.replace('Type="Shape"','Type="Group"').replace('T="MoveTo"','T="LineTo"').replace('Del="0"','Del="1"').replace('Hello','Bye').replace('abc','def')
        _,a=report.package_cells(self.package(before))
        _,b=report.package_cells(self.package(after))
        self.assertEqual(len(report.cell_diffs(a,b,set())),5)
        a_path=self.package('<DocumentSheet/>')
        with zipfile.ZipFile(a_path,'a') as z: z.writestr('visio/media/image1.bmp',b'one')
        _,a=report.package_cells(a_path)
        b_path=self.package('<DocumentSheet/>')
        with zipfile.ZipFile(b_path,'a') as z: z.writestr('visio/media/image1.bmp',b'two')
        _,b=report.package_cells(b_path)
        self.assertEqual(len(report.cell_diffs(a,b,set())),1)

    def test_raw_inh_uses_local_value_and_preserves_unit_context(self):
        def check(name, value, unit, expected, unit_code):
            row=['1','page:0:Page-1/Sheet.1','1','1','0',name,'Width/2',expected,unit_code]
            observed={tuple(row[:6]):row}
            computed_row=list(row);computed_row[7]='1.5'
            computed={tuple(row[:6]):computed_row}
            raw=[('page-shape',0,1,name,value,'Inh',unit,None,True)]
            counts,failures=report.source_com_compare(raw,computed,observed,set(),['Calibri'])
            self.assertEqual(counts['eligible'],1)
            self.assertEqual(failures,[])
        check('PinX','4','IN','4','65')
        check('PinX','4','DL','4','70')
        check('Char.LangID','en',None,'9','32')
        check('Char.Font','0',None,'0','32')

    def test_unmapped_source_and_unknown_language_do_not_pass_silently(self):
        raw=[('page-shape',0,1,'PinX','4','Inh',None,None,True)]
        counts,failures=report.source_com_compare(raw,{}, {},set(),[])
        self.assertEqual(counts['unmapped_local'],1)
        self.assertTrue(failures)
        row=['1','page:0:Page-1/Sheet.1','3','0','0','Char.LangID','0','0','32']
        rows={tuple(row[:6]):row}
        raw=[('page-shape',0,1,'Char.LangID','unknown-language',None,None,None,True)]
        self.assertTrue(report.source_com_compare(raw,rows,rows,set(),[])[1])

    def test_single_layer_membership_uses_its_numeric_result(self):
        raw=[('page-shape',0,1,'LayerMember','1',None,None,None,True)]
        row=['1','page:0:Page/Sheet.1','1','6','2','LayerMember','1','1','231']
        rows={tuple(row[:6]):row}
        self.assertEqual(report.source_com_compare(raw,rows,rows,set(),[])[1],[])

    def test_raw_source_missing_and_empty_attributes_remain_distinct(self):
        path=self.root/'serialized.tsv'
        header='scope\towner\tshape\tname\tV\tF\tU\tE\tpresent\tactive\n'
        path.write_text(header+'page-shape\t0\t1\tComment\t\t\t\t\t1\t1\n')
        row=report.read_serialized(path)[0]
        self.assertEqual(row[4:8],('',None,None,None))
        path.write_text(header+'page-shape\t0\t1\tWidth\t999.000\tInh\tIN\t\t7\t1\n')
        self.assertEqual(report.read_serialized(path)[0][4:8],('999.000','Inh','IN',None))


    def test_style_xml_only_cells_are_disclosed_before_metadata_lookup(self):
        raw=[('style',0,0,'FillGradientStops.GradientStopColor','#ffffff',None,None,None,True),('style',0,0,'LayerMember','0',None,None,None,True)]
        counts,failures=report.source_com_compare(raw,{}, {},set(),[])
        self.assertEqual(counts['non_COM_local'],2)
        self.assertEqual(failures,[])

    def test_phantom_pagesheet_unit_is_diagnostic_only(self):
        want=['0','pagesheet:0:Page/ThePage','1','1','2','Width','','0','32']
        got=list(want);got[7]='1';got[8]='73'
        k=tuple(want[:6]);a={k:got};b={k:want}
        failures,count=report.computed_diagnostic(a,b,{}, {},set())
        self.assertEqual(count,1)
        self.assertEqual(len(failures),1) # The differing value is still reported.
        real={('visio/pages/pages.xml/Page:0/PageSheet/Cell:Width',0):{'V':'0'}}
        failures,count=report.computed_diagnostic(a,b,real,{},set())
        self.assertEqual(count,0)
        self.assertEqual(len(failures),2)


    def test_serialized_source_cumulative_byte_limit(self):
        path=self.root/'large-serialized.tsv'
        header='scope\towner\tshape\tname\tV\tF\tU\tE\tpresent\tactive\n'
        row='page-shape\t0\t1\tComment\t'+('x'*700000)+'\t\t\t\t1\t1\n'
        with path.open('w') as f:
            f.write(header)
            for _ in range(100): f.write(row)
        with self.assertRaisesRegex(ValueError,'serialized source budget'):
            report.read_serialized(path)


    def test_empty_objects_and_anonymous_row_membership_are_visible(self):
        _,a=report.package_cells(self.package('<PageContents><Shapes><Shape ID="1"/></Shapes></PageContents>'))
        _,b=report.package_cells(self.package('<PageContents><Shapes><Shape ID="2"/></Shapes></PageContents>'))
        self.assertTrue(report.cell_diffs(a,b,set()))
        a_xml='<DocumentSheet><Section N="User"><Row><Cell N="X" V="1"/></Row><Row><Cell N="Y" V="2"/></Row></Section></DocumentSheet>'
        b_xml='<DocumentSheet><Section N="User"><Row><Cell N="X" V="1"/><Cell N="Y" V="2"/></Row><Row/></Section></DocumentSheet>'
        _,a=report.package_cells(self.package(a_xml));_,b=report.package_cells(self.package(b_xml))
        self.assertTrue(report.cell_diffs(a,b,set()))


    def test_pagesheet_identity_uses_zero_shape_id_for_any_page(self):
        row=['0','pagesheet:7:Background/ThePage','1','10','0','PageWidth','10','10','70']
        computed=list(row);computed[0]='7'
        raw=[('page',7,0,'PageWidth','10',None,None,None,True)]
        counts,failures=report.source_com_compare(raw,{tuple(computed[:6]):computed},{tuple(row[:6]):row},set(),[])
        self.assertEqual(counts['eligible'],1)
        self.assertEqual(failures,[])

    def test_unobserved_style_cells_are_disclosed_without_numeric_eligibility(self):
        raw=[('style',0,0,'HelpTopic','0',None,None,None,True)]
        counts,failures=report.source_com_compare(raw,{}, {},set(),[])
        self.assertEqual(counts['unobserved_style_local'],1)
        self.assertEqual(counts['eligible'],0)
        self.assertEqual(failures,[])
        row=['0','style/No style','1','0','0','HelpTopic','','0','32']
        mapped={tuple(row[:6]):row}
        self.assertTrue(report.source_com_compare(raw,mapped,{},set(),[])[1])
        self.assertTrue(report.source_com_compare(raw,{},mapped,set(),[])[1])


    def test_nonempty_export_retains_human_one_pixel_rule(self):
        from PIL import Image
        a=self.root/'a.png';b=self.root/'b.png'
        Image.new('RGB',(97,97),'white').save(a)
        Image.new('RGB',(98,97),'white').save(b)
        self.assertEqual(report.pixels(a,b)['mad'],0)
        self.assertTrue(report.pixels(a,b)['one_pixel'])
        Image.new('RGB',(99,97),'white').save(b)
        self.assertIn('error',report.pixels(a,b))
        Image.new('RGB',(97,97),'white').save(b)
        self.assertEqual(report.pixels(a,b)['mad'],0)

    def test_empty_export_requires_blank_480_by_288_ours(self):
        from PIL import Image
        a=self.root/'a.png';b=self.root/'b.png'
        Image.new('RGB',(480,288),'white').save(a)
        Image.new('RGB',(480,288),'white').save(b)
        self.assertTrue(report.empty_pixels(a,b)['empty'])
        ink=Image.new('RGB',(480,288),'white');ink.putpixel((1,1),(0,0,0));ink.save(a)
        with self.assertRaises(ValueError): report.empty_pixels(a,b)


    def test_scratch_sizes_and_connector_gap_are_explicit(self):
        from PIL import Image
        result={'sizes':[(193,97),(193,97)]}
        report.scratch_render_assertion('scratch__dropped-master','page-0',result,None)
        with self.assertRaises(ValueError): report.scratch_render_assertion('scratch__dropped-master','page-0',{'sizes':[(145,97),(145,97)]},None)
        p=self.root/'route.png';im=Image.new('RGB',(481,97),'white');im.save(p)
        with self.assertRaises(ValueError): report.scratch_render_assertion('scratch__dynamic-glue','page-0',{'sizes':[(481,97),(481,97)]},p)
        im.putpixel((240,48),(180,180,180));im.save(p)
        report.scratch_render_assertion('scratch__dynamic-glue','page-0',{'sizes':[(481,97),(481,97)]},p)


    def test_allowed_rewrites_do_not_hide_other_repairs(self):
        def cell(value,kind): return {'V':value,'F':None,'U':None,'E':None,'kind':kind}
        keys=[('master/Attribute:MasterType',0),('master/Attribute:UniqueID',0),('master/Cell:LayerMember',0)]
        a={keys[0]:cell('0','master_type'),keys[1]:cell('{ABCDEF12-1234-ABCD-1111-1234567890AB}','master_uniqueid')}
        b={keys[0]:cell('541','master_type'),keys[1]:cell('{ABCDEF12-1234-ABCD-8E40-00608CF305B2}','master_uniqueid'),keys[2]:cell('0','cell')}
        parts={'added':['docProps/thumbnail.emf','unexpected.xml'],'removed':[]}
        diffs,left,allowed=report.allowed_rewrites(a,b,report.cell_diffs(a,b,set()),parts)
        self.assertEqual(len(diffs),1)
        self.assertEqual(left['added'],['unexpected.xml'])
        self.assertEqual({k:len(v) for k,v in allowed.items()},{'thumbnail_added':1,'master_type':1,'master_uniqueid':1})
        b[keys[1]]=cell('{00000000-1234-ABCD-8E40-00608CF305B2}','master_uniqueid')
        self.assertEqual(len(report.allowed_rewrites(a,b,report.cell_diffs(a,b,set()),parts)[0]),2)

    def test_com_floor_populations_are_pinned_by_sha256(self):
        import hashlib,tomllib
        floors=tomllib.loads((report.ROOT/'crates/sheet/tests/floors.toml').read_text())
        pins=floors['m3_21_cohorts']
        members={}
        for name in ['old_packages','combined_packages','old_dumps','combined_dumps']:
            data=(report.ROOT/pins[name+'_path']).read_bytes()
            self.assertEqual(hashlib.sha256(data).hexdigest(),pins[name+'_sha256'])
            lines=data.decode().splitlines()
            self.assertEqual(len(lines),pins[name+'_count'])
            self.assertEqual(len(lines),len(set(lines)))
            self.assertTrue(all(not Path(v).is_absolute() and '..' not in Path(v).parts for v in lines))
            members[name]=set(lines)
        self.assertLess(members['old_packages'],members['combined_packages'])
        self.assertLess(members['old_dumps'],members['combined_dumps'])


    def test_shipped_render_disclosure_keeps_other_checks_mandatory(self):
        for name in ['shipped-stencil','shipped-template']:
            self.assertFalse(report.content_failed(name,0,[],[{'error':'renderer gap'}]))
            self.assertTrue(report.content_failed(name,1,[],[]))
            self.assertTrue(report.content_failed(name,0,['COM mismatch'],[]))
        for name in ['generated','from-scratch','blank','shipped-unknown']:
            self.assertTrue(report.content_failed(name,0,[],[{'error':'renderer gap'}]))
            self.assertTrue(report.render_required(name))

    def test_baseline_direction_probe_and_scratch(self):
        # Real comparison identities: probe window state, scratch style defaults.
        for shared, defect in [('pages/Page:0/Attribute:ViewCenterX','pages/Shape:1/Cell:Width'),
                               ('visio/document.xml/StyleSheet:0/Cell:X','pages/Shape:1/Cell:FillForegnd')]:
            ours=[(shared,'V','1','2'),(defect,'V','0','1')]
            base=[(shared,'V','3','4'),('baseline-only','added',None,'x')]
            remaining, removed=report.subtract_baseline(ours,base)
            self.assertEqual(remaining,[ours[1]])
            self.assertEqual(removed,[ours[0]])
            self.assertEqual(report.subtract_baseline(ours,[])[0],ours)
            self.assertEqual(report.subtract_baseline(ours,[(defect,'F','0','1')])[0],ours)

    def test_scratch_baseline_is_document_scoped(self):
        self.assertTrue(report.document_level_difference((str(('visio/document.xml/StyleSheet:0/Cell:X',0)),'added')))
        self.assertFalse(report.document_level_difference((str(('visio/masters/master1.xml/Shape:5/Cell:X',0)),'added')))

    def test_palette_font_order_and_metadata_fields_are_visible(self):
        _,a=report.package_cells(self.package('<VisioDocument><Colors><ColorEntry IX="24" RGB="#ffffff"/></Colors><FaceNames><FaceName NameU="Carlito"/><FaceName NameU="Cousine"/></FaceNames></VisioDocument>'))
        _,b=report.package_cells(self.package('<VisioDocument><Colors><ColorEntry IX="24" RGB="#000000"/></Colors><FaceNames><FaceName NameU="Cousine"/><FaceName NameU="Carlito"/></FaceNames></VisioDocument>'))
        self.assertEqual(len(report.cell_diffs(a,b,set())),3)
        _,a=report.package_cells(self.package('<VisioDocument xmlns="http://schemas.microsoft.com/office/visio/2012/main" xmlns:x="urn:extension"><FaceNames><FaceName NameU="Carlito"/><x:Payload/><FaceName NameU="Cousine"/></FaceNames></VisioDocument>'))
        _,b=report.package_cells(self.package('<VisioDocument xmlns="http://schemas.microsoft.com/office/visio/2012/main"><FaceNames><FaceName NameU="Carlito"/><FaceName NameU="Cousine"/></FaceNames></VisioDocument>'))
        self.assertEqual(report.cell_diffs(a,b,set()),[])
        a=report.metadata_records(b'<Windows><Window ViewScale="1" Width="4"/></Windows>','visio/windows.xml')
        b=report.metadata_records(b'<Windows><Window ViewScale="2" Width="9"/></Windows>','visio/windows.xml')
        baseline=report.metadata_records(b'<Windows><Window ViewScale="2" Width="4"/></Windows>','visio/windows.xml')
        remaining,_=report.subtract_baseline(report.cell_diffs(a,b,set()),report.cell_diffs(a,baseline,set()))
        self.assertEqual(len(remaining),1)
        self.assertIn('Width',remaining[0][0])
        self.assertTrue(report.document_level_difference(('part:visio/windows.xml','added')))
        self.assertFalse(report.document_level_difference((str(('visio/pages/page1.xml/Cell:docProps/x',0)),'V')))

    def test_baseline_com_keeps_new_value_and_unit_failures(self):
        key=('page',0,1,'Width')
        ours=[(key,'value local V=1 actual=2'),(key,'units expected=73 actual=70')]
        self.assertEqual(report.subtract_com_baseline(ours,[(key,'value local V=3 actual=4')]),[ours[1]])

if __name__ == '__main__':
    unittest.main()
