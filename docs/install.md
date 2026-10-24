# Installing MisterDiagram

MisterDiagram is one file: a command-line program that also runs as an MCP server for AI
agents. There is no installer and no window. These notes cover the alpha binaries on the
[Releases page](https://github.com/MisterDiagram/MisterDiagram/releases); building from source
is at the end.

## 1. Download

Pick the archive for your machine from the latest release:

| Machine | Archive |
|---|---|
| Mac, Apple silicon or Intel | `misterdiagram-<version>-macos-universal.zip` |
| Windows on an Intel or AMD processor | `misterdiagram-<version>-windows-x64.zip` |
| Windows on Arm (Snapdragon and similar) | `misterdiagram-<version>-windows-arm64.zip` |
| Linux, x86-64 | `misterdiagram-<version>-linux-x64.tar.gz` |
| Linux, Arm 64-bit (Raspberry Pi 4 and 5 with a 64-bit OS, Graviton, Ampere) | `misterdiagram-<version>-linux-arm64.tar.gz` |

Download `SHA256SUMS` from the same release as well. Each archive holds a folder with the
binary, `QUICKSTART.txt`, `CHANGELOG.md` (what works and the known gaps), the licences
(`LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `ATTRIBUTION.md`), `THIRD-PARTY-LICENSES.txt` (the
Rust crates linked into the binary) and the licences of the five embedded fonts under `fonts/`.

The Linux binaries are statically linked, so they run on any distribution with no libraries
to install.

## 2. Check the download

`SHA256SUMS` lists the SHA-256 checksum of every archive. Check yours against it before you
run anything; a match means the file is the one the release workflow built. Run these in the
folder that holds both files:

macOS:

```sh
shasum -a 256 -c SHA256SUMS --ignore-missing
```

Linux:

```sh
sha256sum -c SHA256SUMS --ignore-missing
```

Windows (PowerShell): print the archive's checksum and compare it with its line in
`SHA256SUMS` (the comparison ignores case):

```powershell
(Get-FileHash .\misterdiagram-0.1.0-alpha.6-windows-x64.zip -Algorithm SHA256).Hash
Select-String windows-x64 .\SHA256SUMS
```

Each line should end in `OK` (macOS, Linux), or the two hexadecimal strings should be the same
(Windows). If they differ, do not run the file; download it again.

## 3. Unpack and get past the first-run warnings

The binaries are not signed. Code signing costs money and paperwork that this volunteer
project has not taken on for an alpha; the source is public and the checksums above let you
confirm what you downloaded. Your operating system will still warn you the first time.

### macOS

Unpack the zip (double-click it, or `ditto -x -k misterdiagram-*.zip .` in Terminal). macOS
marks files downloaded by a browser as quarantined, and Gatekeeper refuses to run an unsigned
program that carries that mark ("cannot be opened because the developer cannot be verified",
or "Apple could not verify"). Remove the mark in Terminal, in the unpacked folder:

```sh
xattr -d com.apple.quarantine misterdiagram
./misterdiagram --version
```

Or use System Settings: run `./misterdiagram --version` once and dismiss the warning, then open
System Settings, Privacy & Security, scroll to the message about misterdiagram, click **Open
Anyway** and confirm. Either way, macOS asks only once.

### Windows

Chrome may block the download itself: "misterdiagram-....zip is dangerous" or "isn't commonly
downloaded". Chrome says this about unsigned programs it has not seen often, not because it
found anything. Open the downloads list (Ctrl+J), and choose **Keep** or **Download dangerous
file** for that file. Edge shows a similar prompt with **Keep** under the "..." menu.

Unpack the zip (right-click, **Extract All**). If you start the program from File Explorer,
Microsoft Defender SmartScreen may say "Windows protected your PC": click **More info**, then
**Run anyway**. Run from a terminal, it usually starts without that prompt. Either way, you can
clear the downloaded-file mark first, in PowerShell:

```powershell
Unblock-File .\misterdiagram.exe
.\misterdiagram.exe --version
```

On a PC with Smart App Control turned on (Windows Security, App & browser control), Windows
blocks unsigned programs outright and offers no "Run anyway". Smart App Control can only be
turned off, not paused for one program; if you would rather keep it on, build from source
(section 9) or wait for a signed release.

### Linux

```sh
tar -xzf misterdiagram-*-linux-x64.tar.gz
cd misterdiagram-*-linux-x64
./misterdiagram --version
```

On an Arm machine, use the `linux-arm64` archive and folder instead.

The binary is already executable in the archive. If a tool you used to unpack it dropped the
permission, restore it with `chmod +x misterdiagram`.

## 4. Put it on your PATH

So that `misterdiagram` works from any folder:

- macOS and Linux: move it into a folder on your PATH, for example
  `mkdir -p ~/.local/bin && mv misterdiagram ~/.local/bin/` (add `~/.local/bin` to PATH in your
  shell profile if it is not there yet), or `sudo mv misterdiagram /usr/local/bin/`.
- Windows: make a folder such as `%LOCALAPPDATA%\Programs\misterdiagram`, move
  `misterdiagram.exe` there, and add that folder to your user PATH (Settings, System, About,
  Advanced system settings, Environment Variables, Path, Edit, New). Open a new terminal
  afterwards.

`misterdiagram --version` prints the version, the commit it was built from and the platform,
for example `misterdiagram 0.1.0-alpha.6 (0123456789ab, aarch64-apple-darwin)`. Include that
line in any issue you report.

## 5. First commands

```sh
misterdiagram --version
misterdiagram render drawing.vsdx --page 1 drawing.png
misterdiagram run --batch two-boxes.json --save two-boxes.vsdx
```

`render` also writes `.svg` and `.pdf`. `QUICKSTART.txt` in the archive has the
`two-boxes.json` script. `misterdiagram commands` lists every authoring command with its
parameters; the [README](../README.md) and [docs/mcp.md](mcp.md) cover the rest.

A line like `warning: page 0 shape 12 User.FirstHeadingExtend: #NAME? SHEET.5!LISTORDER;
used the value saved in the file` means a cell uses a ShapeSheet function MisterDiagram does
not calculate yet (the container and list functions are the usual ones), so the render took
the value Visio saved in the file. An unedited file still draws the way Visio last drew it;
a cell you changed has no saved value and keeps its error. `misterdiagram sheet FILE` shows
every such cell. Please report the function in an issue.

### Connectors and obstacles

`connector.add` glues a dynamic connector (route style 0, the page's default right-angle
routing), and its route goes around shapes that count as obstacles. A shape counts when it is
placeable: its `ObjType` cell is 1, which every shape a connector is glued to gets
automatically. A shape with nothing glued to it keeps `ObjType` 0 and connectors may run
straight through it; Visio 2021 does the same. To make such a shape an obstacle, set the cell before adding connectors:

```json
{"command": "sheet.set", "params": {"id": 3, "name": "ObjType", "value": 1}}
```

Parallel connectors get separate lanes. On larger pages Visio would often pick a different
path between the same shapes; both are valid, and Visio keeps ours when it saves the file
again (CHANGELOG.md, known gaps).

## 6. Use it from an AI agent (MCP)

`misterdiagram mcp` speaks the Model Context Protocol over stdin and stdout
(newline-delimited JSON-RPC). Use the full path to the binary in every client.

**Claude Code:**

```sh
claude mcp add misterdiagram -- /full/path/to/misterdiagram mcp
```

On Windows, use the path to `misterdiagram.exe`, for example
`claude mcp add misterdiagram -- C:\Users\you\AppData\Local\Programs\misterdiagram\misterdiagram.exe mcp`.

**Claude Desktop:** open Settings, Developer, Edit Config, or edit the file directly:

- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`

Add the server under `mcpServers` and restart Claude Desktop:

```json
{
  "mcpServers": {
    "misterdiagram": {
      "command": "/full/path/to/misterdiagram",
      "args": ["mcp"]
    }
  }
}
```

On Windows, double every backslash in the path:
`"command": "C:\\Users\\you\\AppData\\Local\\Programs\\misterdiagram\\misterdiagram.exe"`.

**Any other stdio MCP client:** the command is the full path to the binary and the only argument
is `mcp`. The server offers seventeen tools (open, save, export, render, batch, inspection,
stencils, connect); [docs/mcp.md](mcp.md) describes each one.

Relative file paths resolve from the folder the client starts the server in, which is often
not the folder you expect. Give the agent full paths for files to open and save.

## 7. Memory

A session keeps at most 2 GiB by default. Very large drawings or many open stencils may need
more; set the ceiling in whole MiB with `--memory-limit` or the `MISTERDIAGRAM_MEMORY_LIMIT`
environment variable (the flag wins):

```sh
misterdiagram --memory-limit 4096 run --batch big.json --save big.vsdx
misterdiagram mcp --memory-limit 4096
```

In an MCP client config, put the flag in the arguments: `"args": ["mcp", "--memory-limit", "4096"]`.

## 8. A stencil to try

The built-in network stencil (`{"command": "stencil.open", "params": {"builtin": "network"}}`)
has eleven device masters. For a larger set, try ntwrk-clean-and-flat by DukeNuke3D, a
public-domain (Unlicense) network stencil based on ecceman/affinity:
[github.com/DukeNuke3D/ntwrk-clean-and-flat](https://github.com/DukeNuke3D/ntwrk-clean-and-flat)
([licence](https://github.com/DukeNuke3D/ntwrk-clean-and-flat/blob/main/LICENSE)). Download
`affinity_native.vssx` from that repository, save this as `stencil.json` (with the real path),
and run `misterdiagram run --batch stencil.json --save router.vsdx`:

```json
{"commands": [
  {"command": "stencil.open", "params": {"path": "/full/path/to/affinity_native.vssx"}},
  {"command": "master.drop", "params": {"master": "Router", "x": 2, "y": 6}}
]}
```

An agent does the same through the MCP `batch` or `run_command` tools.

It is not bundled in the archive. MisterDiagram's tests use a copy of it, which is how we know
it works. Your own vendor stencils work the same way; tell us which masters fail to drop, glue
or render.

## 9. Building from source

If there is no binary for your platform, or you want the latest commit: install Rust stable
(edition 2024, Rust 1.92 or newer) from [rustup.rs](https://rustup.rs), then

```sh
git clone https://github.com/MisterDiagram/MisterDiagram
cd MisterDiagram
cargo build --release -p misterdiagram-cli
./target/release/misterdiagram --version
```

A source build reports `unknown commit` in `--version`; that is expected.

## Reporting problems

Open an [issue](https://github.com/MisterDiagram/MisterDiagram/issues) with the
`misterdiagram --version` line, the command you ran and what you expected. Please do not attach
drawings that hold customer names, addresses or other private data: a small file that shows the
same problem, or a screenshot and a description, is enough.

### Stencil folders

Set `MISTERDIAGRAM_STENCIL_PATH` to an ordered list of folders. macOS and Linux
use a colon; Windows uses a semicolon. `--stencil-path` accepts the same list and
replaces the environment setting, including on `misterdiagram mcp`.

```sh
export MISTERDIAGRAM_STENCIL_PATH="$HOME/Stencils:$HOME/TeamStencils"
misterdiagram run --cmd 'stencil.available={}'
misterdiagram run --cmd 'stencil.open={"path":"affinity_native"}'
misterdiagram mcp --stencil-path "$HOME/Stencils:$HOME/TeamStencils"
```

```powershell
$env:MISTERDIAGRAM_STENCIL_PATH = 'C:\Stencils;D:\TeamStencils'
misterdiagram mcp --stencil-path 'C:\Stencils;D:\TeamStencils'
```

In an MCP client config, for example:
`"args": ["mcp", "--stencil-path", "/Users/you/Stencils:/Users/you/TeamStencils"]`.
Use semicolons and doubled JSON backslashes for Windows paths.

A bare filename searches each folder's top level in order. Within a folder, the
exact filename is tried first, then the name with `.vssx` appended if omitted.
The first match wins, and `stencil.open` returns its resolved absolute `path`.
A missing filename reports every folder searched. A path containing a directory
separator, `..`, NUL or a drive prefix uses the existing explicit-path route and
never searches these folders. Without a configured list, the working directory
is searched; an empty list entry also means the working directory.

`stencil.available` (MCP tool `stencil_available`) returns `files` with `name`,
`folder` and byte `size`, plus the configured `folders`. It lists `.vssx` files
without opening or parsing them, including a file that would fail admission when
opened. It does not recurse or build a master index. `stencil.list` reports each
open external stencil's resolved absolute path; the document stencil has no path
until opened or saved, and the built-in library retains `builtin:network`.
Files found by name pass the same per-file and session memory checks as explicit
paths. The search setting persists across document replacement.
