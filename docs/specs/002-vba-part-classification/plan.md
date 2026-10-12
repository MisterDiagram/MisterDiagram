# Implementation plan

Classify each part once at the import boundary using Package::content_type.
Remove parts whose type is application/vnd.ms-office.vbaProject or whose
basename matches the legacy vbaProject.bin check. Reuse the computed content
type when storing retained Part values; no downstream filters are needed.

Content types already support extension defaults and part overrides through a
cached parser. Use that owner rather than reparsing XML or guessing extensions.
No public API or error changes. Original package bytes, including metadata and
relationships, remain inspectable; import is not a package sanitization API.

Add contributor-authored in-memory packages covering default/override metadata,
case variants, ordinary opaque parts, and a renamed payload in a non-macro main
document. Demonstrate failure before changing import, then run cargo xtask ci.

Reference:
https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
