$out = '\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\make-corpus'
New-Item -ItemType Directory -Force $out | Out-Null
$log = Join-Path $out 'probe-log.txt'
"START $(Get-Date -Format s)" | Out-File $log
$app = $null
try {
    . (Join-Path $PSScriptRoot 'common.ps1')
    $app = New-Visio
    $office = Office-Folder
    $script:Made = 0; $script:Failed = 0; $script:Resumed = 0
    $resumeRoot = Join-Path $PSScriptRoot 'resume-make-corpus'
    $resumeByName = @{}
    $manifest = Join-Path $resumeRoot 'manifest.json'
    if (Test-Path $manifest) {
        if ((Get-Item $manifest).Length -gt 1048576) { throw 'checkpoint manifest too large' }
        $resume = Get-Content $manifest -Raw | ConvertFrom-Json
        if ($resume.common_sha256 -ne (Get-FileHash (Join-Path $PSScriptRoot 'common.ps1') -Algorithm SHA256).Hash) { throw 'checkpoint common.ps1 differs' }
        if ($resume.resume_recipe_sha256 -ne (Get-FileHash $PSCommandPath -Algorithm SHA256).Hash) { throw 'checkpoint recipe differs' }
        foreach ($entry in $resume.drawings.PSObject.Properties) { $resumeByName[$entry.Name] = $entry.Value }
        Log "CHECKPOINT source=$($resume.source) drawings=$($resumeByName.Count)"
    }
    function Resume-Drawing([string]$name) {
        if (-not $resumeByName.ContainsKey($name)) { return $false }
        $extensions = @('vsdx','png','svg','pdf','sheet.tsv','connects.tsv')
        foreach ($ext in $extensions) {
            $source = Join-Path $resumeRoot "data/$name.$ext"
            if ((Get-FileHash $source -Algorithm SHA256).Hash -ne $resumeByName[$name].$ext) { throw "checkpoint checksum mismatch: $name.$ext" }
        }
        foreach ($ext in $extensions) {
            $target = Join-Path $out "$name.$ext"
            New-Item -ItemType Directory -Force (Split-Path $target) | Out-Null
            Copy-Item -LiteralPath (Join-Path $resumeRoot "data/$name.$ext") -Destination $target
        }
        $script:Made++; $script:Resumed++
        Log "DRAWING RESUMED $name"
        return $true
    }
    function Save-Drawing($doc, [string]$name) {
        $base = Join-Path $out $name
        New-Item -ItemType Directory -Force (Split-Path $base) | Out-Null
        $doc.SaveAs("$base.vsdx")
        Export-Png $app $doc.Pages.Item(1) "$base.png"
        $doc.Pages.Item(1).Export("$base.svg")
        $doc.ExportAsFixedFormat(1, "$base.pdf", 1, 0)
        Dump-Document $doc "$base.sheet.tsv"
        Dump-Connects $doc "$base.connects.tsv"
        $script:Made++
        Log "DRAWING OK $name"
    }
    function Drawing([string]$name, [scriptblock]$build) {
        $doc = $null
        Begin-File $name
        try {
            if (Resume-Drawing $name) { return $true }
            $doc = $app.Documents.Add('')
            $page = $doc.Pages.Item(1)
            & $build $doc $page | Out-Null
            Save-Drawing $doc $name
            return $true
        } catch { $script:Failed++; Log "DRAWING FAIL $name $($_.Exception.Message) $($_.ScriptStackTrace)"; return $false }
        finally { if ($doc) { try { Close-Document $doc } catch { Log "CLOSE FAIL $name $_" } }; End-File }
    }
    function Rect($page) { $page.DrawRectangle(1, 3, 5, 6) }
    function Line-And-Rect($page, [string]$cell, [string]$value) {
        $line = $page.DrawLine(1, 7, 6, 7); Cell $line $cell $value
        $rect = Rect $page; Cell $rect $cell $value
    }
    function Dynamic($page, $a, $b) {
        $c = $page.Drop($app.ConnectorToolDataObject, 0, 0)
        $c.CellsU('BeginX').GlueTo($a.CellsU('PinX'))
        $c.CellsU('EndX').GlueTo($b.CellsU('PinX'))
        $c
    }
    function Pair($page) {
        $a = $page.DrawRectangle(1, 6, 2.5, 7.5)
        $b = $page.DrawRectangle(5, 6, 6.5, 7.5)
        $a.Text = 'A'; $b.Text = 'B'
        @($a, $b)
    }
    function Gradient($page, [int]$direction) {
        # fillgradientenabled-cell-gradient-properties-section.md, fillgradientdir-cell-gradient-properties-section.md
        # gradient-stop-row-fill-gradient-section.md; visSectionFillGradientStops = 249.
        $s = Rect $page
        Cell $s 'FillGradientEnabled' 'TRUE'
        Cell $s 'FillGradientDir' "$direction"
        Cell $s 'FillGradientAngle' '0 deg'
        if (-not $s.SectionExists(249, 0)) { $s.AddSection(249) | Out-Null }
        while ($s.RowCount(249) -lt 2) { $s.AddRow(249, -1, 0) | Out-Null }
        for ($i = 0; $i -lt $s.RowCount(249); $i++) {
            for ($j = 0; $j -lt $s.RowsCellCount(249, $i); $j++) {
                $c = $s.CellsSRC(249, $i, $j)
                if ($c.Name -match 'ColorTrans') { $c.FormulaU = '0%' }
                elseif ($c.Name -match 'Color') { $c.FormulaU = if ($i -eq 0) { 'RGB(0,32,96)' } else { 'RGB(255,255,192)' } }
                elseif ($c.Name -match 'Position') { $c.FormulaU = if ($i -eq 0) { '0%' } else { '100%' } }
            }
        }
    }
    # Validate the explicit obstacle recipe before the larger sweeps.
    Drawing 'connector/obstacle' { param($doc,$page) $pair=Pair $page; $o=$page.DrawRectangle(3,5,4,8); $o.Text='Obstacle'; Cell $o 'ObjType' '1'; Cell $o 'ShapePermeableX' 'FALSE'; Cell $o 'ShapePermeableY' 'FALSE'; Dynamic $page $pair[0] $pair[1] | Out-Null; Route-Connectors $page } | Out-Null
    # Shape Data section (visSectionProp=243); AddNamedRow with visTagDefault=0.
    # https://learn.microsoft.com/en-us/office/client-developer/visio/shape-data-section
    # https://learn.microsoft.com/en-us/office/client-developer/visio/value-cell-shape-data-section
    # https://learn.microsoft.com/en-us/office/vba/api/visio.shape.addnamedrow
    Drawing 'data/property-rows' {
        param($doc,$page)
        $pair=Pair $page; $a=$pair[0]
        if (-not $a.SectionExists(243,0)) { $a.AddSection(243) | Out-Null }
        $properties=@(
            @{Name='Name'; Type='0'; Format='""'; Value='"Switch A"'},
            @{Name='DomainID'; Type='2'; Format='""'; Value='42'},
            @{Name='Fabric'; Type='1'; Format='"A;B"'; Value='"A"'},
            @{Name='Installed'; Type='5'; Format='"yyyy-MM-dd"'; Value='45000'}
        )
        foreach ($p in $properties) {
            $a.AddNamedRow(243,$p.Name,0) | Out-Null
            Cell $a "Prop.$($p.Name).Label" ('"'+$p.Name+'"')
            Cell $a "Prop.$($p.Name).Prompt" '""'
            Cell $a "Prop.$($p.Name).Type" $p.Type
            Cell $a "Prop.$($p.Name).Format" $p.Format
            Cell $a "Prop.$($p.Name).Value" $p.Value
            Cell $a "Prop.$($p.Name).SortKey" '""'
            Cell $a "Prop.$($p.Name).Invisible" 'FALSE'
        }
    } | Out-Null
    # https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop
    # https://learn.microsoft.com/en-us/office/vba/api/visio.shape.text
    # https://learn.microsoft.com/en-us/office/client-developer/visio/horzalign-cell-paragraph-section
    Drawing 'master/from-shape' {
        param($doc,$page)
        $s=$page.DrawRectangle(1,6,2.5,6.75)
        $s.Text="Switch`r`nOriginal"
        Cell $s 'Para.HorzAlign' '1'
        $master=$doc.Masters.Drop($s,0,0); $master.NameU='Switch'; $master.Name='Switch'
        $s.Delete()
        $page.Drop($master,2,6) | Out-Null
        $page.Drop($master,5,6) | Out-Null
        $label=$page.DrawRectangle(1,8,4,8.6); $label.Text='Single run'
        Cell $label 'LinePattern' '0'; Cell $label 'FillPattern' '0'
    } | Out-Null
    # Exact synthetic geometry, text, glue, fill and layer from the original probe.ps1.
    Drawing 'basic/0001-probe' {
        param($doc,$page)
        $a = $page.DrawRectangle(1,8,3,9.5); $a.Text = 'Switch A'
        $b = $page.DrawRectangle(5,8,7,9.5); $b.Text = 'Switch B'
        $c = $page.DrawOval(3,4,5,5.5); $c.Text = 'Storage'
        $conn1 = Dynamic $page $a $b
        $conn2 = Dynamic $page $b $c; $conn2.Text = '32G'
        Cell $b 'FillForegnd' 'RGB(204,0,0)'
        $page.Layers.Add('Fabric') | Out-Null
    } | Out-Null
    # fillpattern-cell-fill-format-section.md, fillforegndtrans-cell-fill-format-section.md
    foreach ($n in 0..40) {
        Drawing ('fill/pattern-{0:D2}' -f $n) {
            param($doc,$page)
            $s = Rect $page
            Cell $s 'FillPattern' "$n"; Cell $s 'FillForegnd' 'RGB(0,32,96)'; Cell $s 'FillBkgnd' 'RGB(255,255,192)'
        } | Out-Null
    }
    Drawing 'fill/gradient-linear' { param($doc,$page) Gradient $page 0 } | Out-Null
    Drawing 'fill/gradient-radial' { param($doc,$page) Gradient $page 1 } | Out-Null
    Drawing 'fill/transparency-50' { param($doc,$page) $s=Rect $page; Cell $s 'FillForegnd' 'RGB(0,32,96)'; Cell $s 'FillForegndTrans' '50%' } | Out-Null
    # linepattern/lineweight/linecap/rounding/compoundtype-cell-line-format-section.md
    foreach ($n in 0..23) { Drawing ('line/pattern-{0:D2}' -f $n) { param($doc,$page) Line-And-Rect $page 'LinePattern' "$n" } | Out-Null }
    foreach ($weight in @('0.25','1','3','6')) { Drawing "line/weight-$weight" { param($doc,$page) Line-And-Rect $page 'LineWeight' "$weight pt" } | Out-Null }
    foreach ($cap in 0..2) { Drawing "line/cap-$cap" { param($doc,$page) Line-And-Rect $page 'LineWeight' '6 pt'; foreach($s in $page.Shapes) { Cell $s 'LineCap' "$cap" } } | Out-Null }
    Drawing 'line/rounding-0.25' { param($doc,$page) $s=Rect $page; Cell $s 'Rounding' '0.25 in' } | Out-Null
    foreach ($type in 0..4) { Drawing "line/compound-$type" { param($doc,$page) Line-And-Rect $page 'CompoundType' "$type"; foreach($s in $page.Shapes) { Cell $s 'LineWeight' '6 pt' } } | Out-Null }
    # beginarrow/endarrow/beginarrowsize/endarrowsize-cell-line-format-section.md
    foreach ($end in @('begin','end')) {
        $arrowCell = if ($end -eq 'begin') { 'BeginArrow' } else { 'EndArrow' }
        $rejected = $false
        foreach ($n in 0..45) {
            $script:ArrowRejected = $false
            $ok = Drawing ('arrow/{0}-{1:D2}' -f $end,$n) {
                param($doc,$page)
                $y = 8
                foreach ($size in @(1,2,4)) {
                    $s = $page.DrawLine(1,$y,6,$y)
                    try { Cell $s $arrowCell "$n"; Cell $s ($arrowCell+'Size') "$size" }
                    catch { $script:ArrowRejected = $true; throw }
                    Cell $s 'LineWeight' '2 pt'
                    $y -= 2
                }
            }
            if ($script:ArrowRejected) { Log "ARROW FIRST_REJECTED end=$end index=$n"; $rejected=$true; break }
        }
        if (-not $rejected) { Log "ARROW FIRST_REJECTED end=$end none_in_0_to_45" }
    }
    # horzalign-cell-paragraph-section.md, verticalalign-cell-text-block-format-section.md
    foreach ($h in 0..2) { foreach ($v in 0..2) {
        Drawing "text/align-$h-$v" { param($doc,$page) $s=Rect $page; $s.Text="Alignment`r`nSecond line"; Cell $s 'Para.HorzAlign' "$h"; Cell $s 'VerticalAlign' "$v" } | Out-Null
    } }
    Drawing 'text/margins' { param($doc,$page) $s=Rect $page; $s.Text='Margins are intentionally unequal.'; Cell $s 'LeftMargin' '0.4 in'; Cell $s 'RightMargin' '0.2 in'; Cell $s 'TopMargin' '0.3 in'; Cell $s 'BottomMargin' '0.1 in' } | Out-Null
    Drawing 'text/rotated-30' { param($doc,$page) $s=Rect $page; $s.Text='Thirty degrees'; Cell $s 'TxtAngle' '30 deg' } | Out-Null
    Drawing 'text/bullets' { param($doc,$page) $s=Rect $page; $s.Text="Alpha`r`nBeta`r`nGamma"; Cell $s 'Para.Bullet' '1' } | Out-Null
    Drawing 'text/tabs' {
        param($doc,$page)
        $s=Rect $page; $s.Text="Name`tValue`r`nAlpha`t100`r`nBeta`t200"
        # position-cell-tabs-section.md. Default tab spacing provides repeatable tab stops.
        Cell $s 'DefaultTabStop' '1 in'
    } | Out-Null
    Drawing 'text/fields' {
        param($doc,$page)
        $s=Rect $page; $s.Text='Date: X  Page: Y'
        # Characters.AddField, VisFieldCategories, VisFieldCodes, VisFieldFormats.
        $chars=$s.Characters; $chars.Begin=6; $chars.End=7; $chars.AddField(1,2,20)
        $chars=$s.Characters; $chars.Begin=$chars.CharCount-1; $chars.End=$chars.CharCount; $chars.AddField(5,3,0)
    } | Out-Null
    Drawing 'text/sizes' {
        param($doc,$page)
        $y=8
        foreach($size in @(8,12,18,36)) { $s=$page.DrawRectangle(1,$y-1,6,$y); $s.Text="$size point text"; Cell $s 'Char.Size' "$size pt"; $y-=1.5 }
    } | Out-Null
    Drawing 'text/multiline-wrap' { param($doc,$page) $s=$page.DrawRectangle(1,3,3,7); $s.Text="A long paragraph should wrap across several lines within a narrow text block.`r`nAn explicit second paragraph follows."; Cell $s 'Char.Size' '18 pt' } | Out-Null
    Drawing 'group/two-rects-rotated' {
        param($doc,$page)
        $a=$page.DrawRectangle(1,4,3,6); $b=$page.DrawRectangle(3,4,5,6); $a.Text='flipped'; $b.Text='normal'
        Cell $a 'FlipX' '1'
        $sel=$page.CreateSelection(0,0,0); $sel.Select($a,2); $sel.Select($b,2)
        $g=$sel.Group(); Cell $g 'Angle' '30 deg'
    } | Out-Null
    Drawing 'master/basic-rect' {
        param($doc,$page)
        $path=Get-ChildItem (Join-Path $office 'Visio Content\1033') -Filter 'BASIC_U.vssx' -Recurse | Select-Object -First 1
        if (-not $path) { throw 'BASIC_U.vssx not found' }
        $stencil=$app.Documents.OpenEx($path.FullName,2)
        try { $a=$page.Drop($stencil.Masters.ItemU('Rectangle'),2,6); $b=$page.Drop($stencil.Masters.ItemU('Rounded Rectangle'),5,6); Cell $a 'Width' '3 in'; Cell $a 'Height' '2 in' }
        finally { Close-Document $stencil }
    } | Out-Null
    Drawing 'connector/static-glue' {
        param($doc,$page)
        $pair=Pair $page; $a=$pair[0]; $b=$pair[1]
        foreach($s in @($a,$b)) { if(-not $s.SectionExists(7,0)) { $s.AddSection(7) | Out-Null }; if($s.RowCount(7)-eq 0) { $s.AddRow(7,0,153) | Out-Null }; Cell $s 'Connections.X1' 'Width*0.5'; Cell $s 'Connections.Y1' 'Height*0.5' }
        $c=$page.Drop($app.ConnectorToolDataObject,0,0); $c.CellsU('BeginX').GlueTo($a.CellsU('Connections.X1')); $c.CellsU('EndX').GlueTo($b.CellsU('Connections.X1'))
    } | Out-Null
    Drawing 'connector/dynamic-glue' { param($doc,$page) $pair=Pair $page; Dynamic $page $pair[0] $pair[1] | Out-Null } | Out-Null
    Drawing 'connector/line-jumps' {
        param($doc,$page)
        $a=$page.DrawRectangle(0.5,4.5,1.5,5.5); $b=$page.DrawRectangle(6.5,4.5,7.5,5.5)
        $c=$page.DrawRectangle(3.5,1.5,4.5,2.5); $d=$page.DrawRectangle(3.5,7.5,4.5,8.5)
        Dynamic $page $a $b | Out-Null; Dynamic $page $c $d | Out-Null
        Cell $page.PageSheet 'LineJumpStyle' '1'; Cell $page.PageSheet 'LineJumpCode' '1'; Route-Connectors $page
    } | Out-Null
    # Save two snapshots of the same document/shape IDs, with a controlled endpoint move.
    $doc=$null
    Begin-File 'connector/move-before-and-after'
    try {
        if ($resumeByName.ContainsKey('connector/move-before') -and $resumeByName.ContainsKey('connector/move-after')) {
            Resume-Drawing 'connector/move-before' | Out-Null
            Resume-Drawing 'connector/move-after' | Out-Null
        } else {
        $doc=$app.Documents.Add(''); $page=$doc.Pages.Item(1); $pair=Pair $page
        Dynamic $page $pair[0] $pair[1] | Out-Null
        Save-Drawing $doc 'connector/move-before'
        $b=$pair[1]; Cell $b 'PinX' (($b.CellsU('PinX').ResultIU+2).ToString('R',$inv)+' in'); Cell $b 'PinY' (($b.CellsU('PinY').ResultIU-1).ToString('R',$inv)+' in')
        Save-Drawing $doc 'connector/move-after'
        }
    } catch { $script:Failed++; Log "DRAWING FAIL connector/move-before-and-after $($_.Exception.Message)" }
    finally { if($doc) { Close-Document $doc }; End-File }
    Drawing 'page/background' {
        param($doc,$page)
        $bg=$doc.Pages.Add(); $bg.Name='Background'; $bg.Background=-1
        $s=$bg.DrawRectangle(0.5,0.5,7,9); $s.Text='Background'; Cell $s 'FillForegnd' 'RGB(230,240,255)'
        $page.BackPage=$bg.Name; $s=$page.DrawOval(2,3,5,6); $s.Text='Foreground'
    } | Out-Null
    Drawing 'page/layers' {
        param($doc,$page)
        $hidden=$page.Layers.Add('Invisible'); $locked=$page.Layers.Add('Locked')
        $a=$page.DrawRectangle(1,4,3,6); $a.Text='Hidden'; $hidden.Add($a,0)
        $b=$page.DrawRectangle(4,4,6,6); $b.Text='Locked'; $locked.Add($b,0)
        $hidden.CellsC(4).FormulaU='0'; $locked.CellsC(7).FormulaU='1'
    } | Out-Null
    Drawing 'page/metric-scale' {
        param($doc,$page)
        Cell $page.PageSheet 'PageScale' '1 cm'; Cell $page.PageSheet 'DrawingScale' '1 m'
        # DrawingScale formula units also select metric drawing units.
        $s=$page.DrawRectangle(1,1,3,2); Cell $s 'Width' '2 m'; $s.Text='2 m'
    } | Out-Null
    foreach($ext in @('png','jpg','emf')) {
        Drawing "image/$ext" {
            param($doc,$page)
            $path=Join-Path $PSScriptRoot "assets\sample.$ext"
            $s=$page.Import($path); Cell $s 'PinX' '4 in'; Cell $s 'PinY' '5 in'; Cell $s 'Width' '3 in'; Cell $s 'Height' '2 in'
        } | Out-Null
    }
    # P4b route sweep. These recipes observe Visio; no route coordinates are supplied.
    # pinx-cell-shape-transform-section.md, piny-cell-shape-transform-section.md
    foreach ($offset in @('right-above','right-below','above','below','left')) {
        Drawing "route/offset-$offset" {
            param($doc,$page)
            $pair=Pair $page
            switch ($offset) {
                'right-above' { Cell $pair[1] 'PinY' '9 in' }
                'right-below' { Cell $pair[1] 'PinY' '3 in' }
                'above' { Cell $pair[1] 'PinX' '1.75 in'; Cell $pair[1] 'PinY' '9.75 in' }
                'below' { Cell $pair[1] 'PinX' '1.75 in'; Cell $pair[1] 'PinY' '3.75 in' }
                'left' { Cell $pair[0] 'PinX' '6.75 in'; Cell $pair[1] 'PinX' '1.75 in' }
            }
            Dynamic $page $pair[0] $pair[1] | Out-Null
            Route-Connectors $page
        } | Out-Null
    }
    function Route-Obstacle($page, [double]$x0, [double]$y0, [double]$x1, [double]$y1, [int]$kind=1, [bool]$permeable=$false) {
        $o=$page.DrawRectangle($x0,$y0,$x1,$y1)
        # objtype-cell-miscellaneous-section.md
        Cell $o 'ObjType' "$kind"
        # shapepermeablex-cell-shape-layout-section.md, shapepermeabley-cell-shape-layout-section.md
        $value=if ($permeable) { 'TRUE' } else { 'FALSE' }
        Cell $o 'ShapePermeableX' $value; Cell $o 'ShapePermeableY' $value
        $o
    }
    foreach ($obstacle in @('below-centre','stacked','touching','permeable','objtype-zero')) {
        Drawing "route/obstacle-$obstacle" {
            param($doc,$page)
            $pair=Pair $page
            switch ($obstacle) {
                'below-centre' { Route-Obstacle $page 3 4.5 4 7 | Out-Null }
                'stacked' { Route-Obstacle $page 3 4 4 6.25 | Out-Null; Route-Obstacle $page 3 7.25 4 9 | Out-Null }
                'touching' { Route-Obstacle $page 3 5 4 6.75 | Out-Null }
                'permeable' { Route-Obstacle $page 3 5 4 8 1 $true | Out-Null }
                'objtype-zero' { Route-Obstacle $page 3 5 4 8 0 | Out-Null }
            }
            Dynamic $page $pair[0] $pair[1] | Out-Null
            Route-Connectors $page
        } | Out-Null
    }
    foreach ($style in @(1,2,16)) {
        Drawing "route/style-$style" {
            param($doc,$page)
            $pair=Pair $page
            # piny-cell-shape-transform-section.md, routestyle-cell-page-layout-section.md
            Cell $pair[1] 'PinY' '9 in'; Cell $page.PageSheet 'RouteStyle' "$style"
            Dynamic $page $pair[0] $pair[1] | Out-Null
            Route-Connectors $page
        } | Out-Null
    }
    Drawing 'route/style-override' {
        param($doc,$page)
        $pair=Pair $page
        # piny-cell-shape-transform-section.md, routestyle-cell-page-layout-section.md
        Cell $pair[1] 'PinY' '9 in'; Cell $page.PageSheet 'RouteStyle' '1'
        $c=Dynamic $page $pair[0] $pair[1]
        # shaperoutestyle-cell-shape-layout-section.md
        Cell $c 'ShapeRouteStyle' '2'
        Route-Connectors $page
    } | Out-Null
    function Route-Crossings($page, [int]$count=1) {
        $a=$page.DrawRectangle(0.5,4.5,1.5,5.5); $b=$page.DrawRectangle(6.5,4.5,7.5,5.5)
        $ends=@()
        $xs=if ($count -eq 3) { @(2,4,6) } else { @(4) }
        foreach ($x in $xs) {
            $c=$page.DrawRectangle($x-0.5,1.5,$x+0.5,2.5)
            $d=$page.DrawRectangle($x-0.5,7.5,$x+0.5,8.5)
            $ends+=,@($c,$d)
        }
        Dynamic $page $a $b | Out-Null
        foreach ($pair in $ends) { Dynamic $page $pair[0] $pair[1] | Out-Null }
    }
    foreach ($jump in @('gap','square','two-sides','code-2','factor-x','three-crossings')) {
        Drawing "route/jump-$jump" {
            param($doc,$page)
            $count=if ($jump -eq 'three-crossings') { 3 } else { 1 }
            Route-Crossings $page $count
            $style=switch ($jump) { 'gap' {2} 'square' {3} 'two-sides' {4} default {1} }
            $code=if ($jump -eq 'code-2') { 2 } else { 1 }
            # linejumpstyle-cell-page-layout-section.md, linejumpcode-cell-page-layout-section.md
            # Numeric code 2 is requested; the reference labels it vertical, not horizontal.
            Cell $page.PageSheet 'LineJumpStyle' "$style"; Cell $page.PageSheet 'LineJumpCode' "$code"
            # linejumpfactorx-cell-page-layout-section.md
            if ($jump -eq 'factor-x') { Cell $page.PageSheet 'LineJumpFactorX' '1.0' }
            Route-Connectors $page
        } | Out-Null
    }
    foreach ($preference in @('line-to-node','block-size','avoid-page-breaks')) {
        Drawing "route/preference-$preference" {
            param($doc,$page)
            $pair=Pair $page
            # plowcode-cell-page-layout-section.md: keep the authored obstacle positions.
            Cell $page.PageSheet 'PlowCode' '0'
            switch ($preference) {
                'line-to-node' {
                    Route-Obstacle $page 3 5 4 8 | Out-Null
                    # linetonodex-cell-page-layout-section.md, linetonodey-cell-page-layout-section.md
                    Cell $page.PageSheet 'LineToNodeX' '0.25 in'; Cell $page.PageSheet 'LineToNodeY' '0.25 in'
                }
                'block-size' {
                    Route-Obstacle $page 3 5 4 8 | Out-Null
                    # blocksizex-cell-page-layout-section.md, blocksizey-cell-page-layout-section.md
                    Cell $page.PageSheet 'BlockSizeX' '0.5 in'; Cell $page.PageSheet 'BlockSizeY' '0.5 in'
                }
                'avoid-page-breaks' {
                    # pagewidth-cell-page-properties-section.md, pageheight-cell-page-properties-section.md
                    Cell $page.PageSheet 'PageWidth' '17 in'; Cell $page.PageSheet 'PageHeight' '11 in'
                    # paperkind-cell-print-properties-section.md, printpageorientation-cell-print-properties-section.md
                    Cell $page.PageSheet 'PaperKind' '1'; Cell $page.PageSheet 'PrintPageOrientation' '1'
                    # pinx-cell-shape-transform-section.md, piny-cell-shape-transform-section.md
                    Cell $pair[0] 'PinX' '6 in'; Cell $pair[1] 'PinX' '10 in'; Cell $pair[1] 'PinY' '9 in'
                    # avoidpagebreaks-cell-page-layout-section.md
                    Cell $page.PageSheet 'AvoidPageBreaks' 'TRUE'
                }
            }
            Dynamic $page $pair[0] $pair[1] | Out-Null
            Route-Connectors $page
        } | Out-Null
    }
    # https://learn.microsoft.com/en-us/office/vba/api/visio.page.settheme
    # https://learn.microsoft.com/en-us/office/vba/api/visio.page.setthemevariant
    # A refused COM call is logged by Drawing; the route sweep is still retained.
    Drawing 'theme/builtin-1' {
        param($doc,$page)
        Pair $page | Out-Null
        $page.SetTheme(1)
    } | Out-Null
    Drawing 'theme/builtin-1-variant' {
        param($doc,$page)
        Pair $page | Out-Null
        $page.SetTheme(1)
        $page.SetThemeVariant(1,1,2)
    } | Out-Null

    Log "SUMMARY made=$script:Made resumed=$script:Resumed failed=$script:Failed cells=$script:CellCount cell_errors=$script:CellErrors primary_cells=$($script:CellCount-$script:SupplementCells) supplemental_cells=$script:SupplementCells"
} catch { "FAIL $($_.Exception.Message) $($_.ScriptStackTrace)" | Out-File $log -Append
} finally {
    if ($app) { try { $app.Quit() } catch { "QUIT FAIL $_" | Out-File $log -Append } }
    'DONE' | Out-File (Join-Path $out 'DONE')
}
