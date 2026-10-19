# Font availability for native library labels. No fonts are installed or copied.
$out='\\Mac\Home\Projects\diagramcraft\corpus\_vm\out\network-font-witness'
New-Item -ItemType Directory -Force $out|Out-Null
try {
 Add-Type -AssemblyName System.Drawing
 $fonts=New-Object System.Drawing.Text.InstalledFontCollection
 $fonts.Families.Name|Sort-Object|Out-File (Join-Path $out 'families.txt')
 $fonts.Dispose()
} catch {$_|Out-File (Join-Path $out 'error.txt')}
'DONE'|Out-File (Join-Path $out 'DONE')
