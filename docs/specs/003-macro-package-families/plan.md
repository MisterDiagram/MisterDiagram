# Plan

Extend the supported main-content-type match in the OPC owner. Extend only
the CLI info extension gate; sheet/render already use the format-aware importer.
Use existing PackageOut test builders for all six families and preservation.
No public API, layer, dependency or resource-limit change.

Microsoft's public format introduction lists the six extensions:
https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx

The Apache Tika project's published MIME mapping independently identifies the
two main content type strings: https://issues.apache.org/jira/browse/TIKA-1286
Only identifiers were consulted; implementation and sample attachments were not used.
