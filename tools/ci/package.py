#!/usr/bin/env python3
"""Release packaging for the misterdiagram binaries (MIT OR Apache-2.0).

  package.py archive PLATFORM BINARY THIRDPARTY OUTDIR
                                               one archive, refusing a missing or extra member;
                                               THIRDPARTY is third_party.py's output for the target
  package.py sums DIR                          write DIR/SHA256SUMS over every archive in DIR
  package.py notes [TAG]                       print the CHANGELOG entry for the workspace
                                               version; with TAG, refuse unless TAG is v<version>

PLATFORM is one of macos-universal, windows-x64, windows-arm64, linux-x64, linux-arm64. Windows
and macOS get .zip, Linux gets .tar.gz; the binary is mode 0755 in both. Every member sits in a
top folder named after the archive. Timestamps come from SOURCE_DATE_EPOCH when it is set.
Package only a binary built with CRAFT_FONTS_DIR empty (as release.yml does): a local build
that embeds craft-fonts would ship fonts whose licences are not in the archive.
"""
import hashlib
import io
import os
from pathlib import Path
import sys
import tarfile
import time
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[2]
PLATFORMS = ('macos-universal', 'windows-x64', 'windows-arm64', 'linux-x64', 'linux-arm64')
FONTS = ('arimo', 'caladea', 'carlito', 'cousine', 'noto-sans-symbols2', 'tinos')
# The fixed member list; gathered files are compared against it, never the other way round.
NOTICES = ('LICENSE-MIT', 'LICENSE-APACHE', 'ATTRIBUTION.md', 'NOTICE', 'CHANGELOG.md', 'QUICKSTART.txt',
           'THIRD-PARTY-LICENSES.txt')


def version():
    with open(ROOT / 'Cargo.toml', 'rb') as f:
        return tomllib.load(f)['workspace']['package']['version']


def expected(platform):
    binary = 'misterdiagram.exe' if platform.startswith('windows') else 'misterdiagram'
    return {binary, *NOTICES, *(f'fonts/{font}/OFL.txt' for font in FONTS)}


def gather(binary, third_party):
    files = {binary.name: binary, 'THIRD-PARTY-LICENSES.txt': third_party}
    for name in ('LICENSE-MIT', 'LICENSE-APACHE', 'ATTRIBUTION.md', 'NOTICE', 'CHANGELOG.md'):
        files[name] = ROOT / name
    files['QUICKSTART.txt'] = ROOT / 'tools/ci/QUICKSTART.txt'
    for licence in sorted((ROOT / 'crates/fonts/assets').glob('*/OFL.txt')):
        files[f'fonts/{licence.parent.name}/OFL.txt'] = licence
    return files


def archive(platform, binary, third_party, outdir):
    if platform not in PLATFORMS:
        raise SystemExit(f'unknown platform {platform!r}; expected one of {", ".join(PLATFORMS)}')
    binary = Path(binary)
    want = expected(platform)
    files = gather(binary, Path(third_party))
    missing, extra = sorted(want - files.keys()), sorted(files.keys() - want)
    absent = sorted(name for name, path in files.items() if not path.is_file())
    if missing or extra or absent:
        raise SystemExit(f'refusing to package {platform}: missing {missing}, extra {extra}, not on disk {absent}')
    name = f'misterdiagram-{version()}-{platform}'
    stamp = int(os.environ.get('SOURCE_DATE_EPOCH') or time.time())
    outdir = Path(outdir)
    outdir.mkdir(parents=True, exist_ok=True)
    if platform.startswith('linux'):
        path = outdir / f'{name}.tar.gz'
        with tarfile.open(path, 'w:gz', format=tarfile.PAX_FORMAT) as tar:
            for member in sorted(files):
                data = files[member].read_bytes()
                info = tarfile.TarInfo(f'{name}/{member}')
                info.size, info.mtime, info.uid, info.gid, info.uname, info.gname = len(data), stamp, 0, 0, '', ''
                info.mode = 0o755 if member == binary.name else 0o644
                tar.addfile(info, io.BytesIO(data))
        with tarfile.open(path) as tar:
            got = {m.name.removeprefix(name + '/') for m in tar.getmembers()}
            mode = tar.getmember(f'{name}/{binary.name}').mode
    else:
        path = outdir / f'{name}.zip'
        date = time.gmtime(max(stamp, 315532800))[:6]
        with zipfile.ZipFile(path, 'w', compression=zipfile.ZIP_DEFLATED) as zf:
            for member in sorted(files):
                info = zipfile.ZipInfo(f'{name}/{member}', date_time=date)
                info.compress_type = zipfile.ZIP_DEFLATED
                info.create_system = 3
                info.external_attr = (0o100755 if member == binary.name else 0o100644) << 16
                zf.writestr(info, files[member].read_bytes())
        with zipfile.ZipFile(path) as zf:
            got = {i.filename.removeprefix(name + '/') for i in zf.infolist()}
            mode = zf.getinfo(f'{name}/{binary.name}').external_attr >> 16 & 0o777
    if got != want or mode != 0o755:
        path.unlink()
        raise SystemExit(f'archive {path.name} came out wrong: members {sorted(got ^ want)} differ, binary mode {oct(mode)}')
    print(path)
    return path


def sums(directory):
    directory = Path(directory)
    archives = sorted(p for p in directory.iterdir() if p.name.endswith(('.zip', '.tar.gz')))
    names = {p.name for p in archives}
    want = {f'misterdiagram-{version()}-{platform}.{"tar.gz" if platform.startswith("linux") else "zip"}' for platform in PLATFORMS}
    if names != want:
        raise SystemExit(f'SHA256SUMS needs exactly the five archives: missing {sorted(want - names)}, extra {sorted(names - want)}')
    lines = [f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n' for p in archives]
    (directory / 'SHA256SUMS').write_text(''.join(lines), encoding='utf-8', newline='\n')
    print(''.join(lines), end='')


def notes(tag=None):
    v = version()
    if tag is not None and tag != f'v{v}':
        raise SystemExit(f'tag {tag!r} does not match the workspace version v{v}')
    text = (ROOT / 'CHANGELOG.md').read_text(encoding='utf-8').splitlines()
    starts = [i for i, line in enumerate(text) if line.split(' (')[0] == f'## {v}']
    if len(starts) != 1:
        raise SystemExit(f'CHANGELOG.md needs exactly one "## {v}" entry, found {len(starts)}')
    end = next((i for i in range(starts[0] + 1, len(text)) if text[i].startswith('## ')), len(text))
    body = '\n'.join(text[starts[0] + 1:end]).strip()
    if not body:
        raise SystemExit(f'CHANGELOG.md entry {v} is empty')
    # A release page renders every newline, so join the changelog's hard-wrapped lines: a
    # continuation line (indented, or plain text after text) joins the line before it.
    lines = []
    for line in body.split('\n'):
        joins = lines and lines[-1].strip() and line.strip() and not line.lstrip().startswith(('- ', '#', '|', '```'))
        if joins and (line.startswith('  ') or not lines[-1].startswith('#')):
            lines[-1] = lines[-1].rstrip() + ' ' + line.strip()
        else:
            lines.append(line)
    body = '\n'.join(lines)
    # Relative links in the changelog would point nowhere on a release page.
    print(body.replace('](docs/', f'](https://github.com/MisterDiagram/MisterDiagram/blob/{tag or "master"}/docs/'))


if __name__ == '__main__':
    args = sys.argv[1:]
    if args[:1] == ['archive'] and len(args) == 5:
        archive(*args[1:])
    elif args[:1] == ['sums'] and len(args) == 2:
        sums(args[1])
    elif args[:1] == ['notes'] and len(args) in (1, 2):
        notes(*args[1:])
    else:
        raise SystemExit(__doc__)
