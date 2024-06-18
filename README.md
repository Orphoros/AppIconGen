# AppIconGen

![GitHub release (latest by date)](https://img.shields.io/github/v/release/Orphoros/AppIconGen?label=latest%20release)

## About

AppIconGen is a CLI tool that generates various image formats for application software icons from a single PNG image file. The tool is designed to be used as a part of a build process for software development projects.

> [!NOTE]
> AppIconGen is currently in development and is only available for testing purposes as an alpha release.

## Features

The input file must be a single PNG file with a resolution of `1024x1024` pixels or greater. This CLI tool can generate the following images:

| Icon Type | File Format | Default Resolution | Usage |
| --------- | ---------- | ---------------- | ----- |
| `ICNS`    | .icns      | `16x16`, `32x32`, `128x128`, `256x256`, `512x512`, `16x16@2x`, `32x32@2x`, `128x128@2x`, `256x256@2x`, `512x512@2x` |macOS application icon |
| `ICO`     | .ico       | `16x16`, `32x32`, `48x48`, `96x96`, `256x256` | Windows application icon or website favicon |
| `TRAY`    | .png       | `512x512` | White system tray icon |

## Usage

To generate all the image formats, run the following command:

```sh
appicongen icon.png
```

The command will output the following files: `appicon.icns`, `appicon.ico`, and `appicon_tray.png`.

To rename the output files, use the `-o` flag:

```sh
appicongen -o myiconname icon.png
```

By default, the tool generates all the image formats. To generate only specific image formats, use the `-i`, `-I`, or `-t` flags:

```sh
appicongen -i -t icon.png
```

Optionally, the `-r` flag can be used to specify what resolutions to generate for the ICO file:

```sh
appicongen -i -r "16,32,48,256" icon.png
```

If no input file is specified, AppIconGen will look for a file named `icon.png` in the current directory.

## Installation

You can download the latest version of the precompiled binaries for your platform from the table below. The binaries do not automatically update themselves. It is not recommended to manually install the precompiled binaries. Instead, use the package manager for your platform to install the tool.

| Platform |  Architecture | Download |
| -------- | -------------- | -------- |
| Windows  | x64            | [appicongen.x86_64-pc-windows-msvc.zip](https://github.com/Orphoros/AppIconGen/releases/latest/download/appicongen.x86_64-pc-windows-msvc.zip) |
| macOS    | x64 (Intel)    | [appicongen.x86_64-apple-darwin.tar.gz](https://github.com/Orphoros/AppIconGen/releases/latest/download/appicongen.x86_64-apple-darwin.tar.gz) |
| macOS    | Apple Silicon  | [appicongen.aarch64-apple-darwin.tar.gz](https://github.com/Orphoros/AppIconGen/releases/latest/download/appicongen.aarch64-apple-darwin.tar.gz) |
| Linux    | x64            | [appicongen.x86_64-unknown-linux-gnu.tar.gz](https://github.com/Orphoros/AppIconGen/releases/latest/download/appicongen.x86_64-unknown-linux-gnu.tar.gz) |
| Linux    | ARM            | [appicongen.aarch64-unknown-linux-gnu.tar.gz](https://github.com/Orphoros/AppIconGen/releases/latest/download/appicongen.aarch64-unknown-linux-gnu.tar.gz) |

### Scoop installation for Windows

You can install AppIconGen on Windows using [Scoop](https://scoop.sh/).

```pwsh
scoop bucket add orphoros_scoop-bucket https://github.com/Orphoros/scoop-bucket
```

```pwsh
scoop install appicongen
```

### Homebrew installation for macOS

You can install AppIconGen on macOS using [Homebrew](https://brew.sh/).

```bash
brew tap orphoros/core
```

```bash
brew install appicongen
```
