# Rehman-PDF app icon

<img src="pdfcraft-small.svg" alt="Rehman-PDF app icon: a white R on stacked pages, on indigo" width="128">

**Mark:** stacked white pages with a folded corner, the front page carrying a bold indigo **R**.

**Style:** flat geometric vector; a full-bleed indigo field, no frame or roundel.

**Palette:** two colours.

| Colour | Hex | Used for |
|---|---|---|
| Indigo (app colour) | `#4F46E5` | the full-bleed field and the R |
| White | `#FFFFFF` | the pages |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112` that clips everything. Windows and Linux
icons use the full-bleed tile. macOS icons put it on Apple's grid (an 824 px body centred on a transparent
1024 px canvas).

**Provenance:** the mark is drawn from the app icon the project's author chose for the T-003 redesign
(`tasks/T-003/icons/chosen-by-user.png` in the dev-team workspace — his own AI-generated image,
contributor-original). Licence: [LICENSE.txt](LICENSE.txt) (`MIT OR Apache-2.0`, like the repo).

## Files

| File | What it is |
|---|---|
| `pdfcraft.svg` | the master vector; every PNG, `.ico` and `.icns` is rendered from it |
| `pdfcraft-small.svg` | a lighter copy for places where size matters, such as this README |
| `pdfcraft-1024.png` | 1024 px on Apple's grid; also the runtime Dock icon on macOS |
| `pdfcraft.icns` | macOS icon (16–1024 px) |
| `pdfcraft.ico` | Windows icon (16–256 px), embedded in `pdfcraft.exe` by `apps/pdfcraft/build.rs` |
| `pdfcraft-document.svg` | Windows PDF file icon master: page outline, indigo badge, PDF lettering |
| `pdfcraft-document.ico` | Windows PDF file icon (16–256 px), embedded as resource 2 in `pdfcraft.exe` |
| `hicolor/<n>x<n>/apps/com.rehmanahmad.rehmanpdf.png` | Linux hicolor theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/com.rehmanahmad.rehmanpdf.svg` | Linux scalable icon (copy of the master) |

Where it shows: `apps/pdfcraft/src/main.rs` sets the window icon (Dock, taskbar, Alt-Tab, launcher) and the
Wayland app id `com.rehmanahmad.rehmanpdf`; `packaging/linux/com.rehmanahmad.rehmanpdf.desktop` names the
hicolor icon.

Windows PDF files use resource 2 (`"pdfcraft.exe",-2`) when Rehman-PDF is chosen as the default
reader; the app keeps resource 1. The document icon artwork is contributor-original
(`MIT OR Apache-2.0`), covered by [LICENSE-document.txt](LICENSE-document.txt). Both icons are
embedded, requiring no runtime sidecar. This provides a file type icon; first-page thumbnails still
need a thumbnail provider.

## Regenerate

```sh
python3 <dev-team>/tasks/T-003/icons/regenerate.py   # Pillow only; also writes the .icns
packaging/icons.sh        # the upstream script: needs resvg and python3; iconutil (macOS) for the .icns
cargo xtask assets        # then update the sha256 values in ATTRIBUTION.toml and run with --write
```
