# Barka (Designer) — T-003 mockups + design findings

Branch `t003-redesign` off `main` (`dee9e6a2`). Mockups committed locally; NOT pushed.
Direction is user-locked **B — Modern minimal**: a clean contemporary identity,
familiar PDF-app layout, no upstream ArtCraft look, no reskin.

## Theme spec — "Rehman-PDF Modern Minimal"

### Palette (the single accent is indigo — replaces upstream's #1B63E0 blue)
| token | light | dark | use |
|---|---|---|---|
| accent | `#4F46E5` | `#818CF8` | primary actions, selection, active states |
| accent-soft | `#EEF0FE` | `#232A52` | selected rows, tinted badges, mode-tab fill |
| text | `#1C1C21` | `#EDEDF0` | body |
| muted | `#5B5B63` | `#A9A9B2` | secondary text, labels |
| faint | `#74747C` | `#8A8A92` | captions, timestamps |
| icon | `#3F3F46` | `#CFCFD6` | icons |
| chrome | `#FFFFFF` | `#1E1E22` | titlebar-adjacent bars, panels |
| titlebar | `#F5F5F7` | `#17171A` | top strip |
| pasteboard | `#ECECF0` | `#101013` | document area behind pages |
| card | `#FFFFFF` | `#26262B` | cards, dialogs |
| border | `#E2E2E8` | `#36363C` | card outlines |
| divider | `#ECECF0` | `#2E2E33` | separators |
| hover / pressed | `#F1F1F4` / `#E6E6EB` | `#2C2C32` / `#38383E` | control states |
| field | `#FFFFFF` | `#1C1C20` | text inputs |
| danger | `#D92D20` | `#F04438` | delete / destructive |

Document pages stay **white in both themes** (keeps the upstream rule from
theme.rs: "Dark Gray keeps pages white"). Tool-glyph tiles keep their
per-tool hue (green/orange/purple/pink/blue) on tinted backgrounds —
light uses pastel tints (`#E7F6E9` etc.), dark uses deep tints (`#1D3A24` etc.).

### Typography
- UI: **Inter** (400/500/600/700). Display 24 semibold (home hero), title 18
  semibold (dialog titles), section 15–17 semibold, body 13, small 12,
  caption 11.5. Tight tracking on headlines (−0.3px).
- Urdu: **Noto Nastaliq Urdu** fallback at the same sizes; Nastaliq reserves
  ~2.6× line-height (mockups show it — nothing clips).
- Weights carry hierarchy, not color: semibold titles, medium labels,
  regular body.

### Spacing & shape
- 4px base scale: 4 / 8 / 12 / 16 / 24 / 28 / 32 / 48. Card padding 16–18,
  section gaps 24–28, dialog padding 24, footer gap 10.
- Radius: 8 (buttons, inputs, icon tiles), 10–12 (cards, tool cards),
  14–16 (window, dialogs). Chips/pills fully rounded.
- Iconography: 1.8px stroke, rounded caps (Lucide-style); 16px inline,
  17px chrome, 21px feature glyphs. One accent family + semantic danger —
  no rainbow chrome.

## Screens mocked (8 files, light + dark each)
All in `~/workspace/dev-team/tasks/T-003/mockups/`. Open any file directly
in a browser — each is self-contained (CSS inline, fonts via Google Fonts
with system fallback).

1. `home-light.html` / `home-dark.html` — Welcome hero ("Welcome to
   Rehman-PDF", real tagline), "Rehman-PDF on the web" card (GitHub /
   rehmanahmad.com buttons), Recommended tools grid with the 5 real tool
   cards (Organize pages, Add comments, Prepare a form, Edit a PDF,
   Protect a PDF — real blurbs from the catalogue) + dashed "Open file"
   dropzone, Recent list with real labels ("Recent", "Clear", file rows
   with path + relative time + remove affordance).
2. `reader-light.html` / `reader-dark.html` — Read mode: tab strip
   (Home + Urdu-Worksheet-Class5.pdf), Menu/File/Edit/Pages/View,
   segmented mode tabs (All tools / Read / Edit / Convert / E-Sign),
   Print / Save / Document properties icons, ⌘K "Find tools and commands";
   left page-thumbnails panel (page 3 selected), center page with a real
   Urdu worksheet sample incl. text selection highlight, right icon rail
   (thumbnails / bookmarks / find / annotate), status bar ("3 of 28",
   "Fit to width", zoom 128%).
3. `organize-light.html` / `organize-dark.html` — Organize action bar with
   real actions (Select all · Rotate left/right · Extract · Split ·
   Insert from file · Insert blank page · Delete in danger red ·
   thumbnail-size slider), card grid with 3 selected pages (indigo ring +
   check badges), drag insertion-line affordance, status bar
   ("3 of 28 pages selected", Undo/Redo).
4. `dialogs-light.html` / `dialogs-dark.html` — Document Properties
   (Description tab, real rows: File/Title/Author/Subject/Keywords/
   Application/PDF producer; Cancel/Save footer) + Split document tool
   dialog (real radio options: Number of pages / File size /
   Top-level bookmarks (4) / Before each selected page — disabled;
   "Creates 6 files from 28 pages." summary; Cancel/Split footer).
   Includes an **RTL mirror strip**: the Properties dialog in Urdu
   (`dir=rtl`), proving the mirror layout with real Urdu labels.

## RTL notes (for Saima — mirroring is part of the design)
- In the `ur` locale the ENTIRE chrome mirrors: tab strip, menu bar,
  mode tabs, action bars, dialog label/value columns, footer button order
  (primary action lands at the **left** end — see the RTL strip).
- Latin file names inside RTL text must keep bidi isolation
  (egui's `bidi::visual` already handles this — don't regress it).
- Nastaliq line-height (~2.6×) is reserved in row spacing; never
  vertically center Urdu text in fixed 13px rows — it will clip.
- The icon rail, zoom stepper and selection check badges are
  direction-neutral and do NOT mirror; only reading-order chrome does.
- Tool-card "Use now →" arrow flips to ← in RTL.

## Notes for Saima (implementation)
- The visual contract lives in the 8 HTML files — match spacing/radius/
  colors exactly; the palette table above is the token map for
  `theme.rs` (`Tokens` gains the same names: titlebar/chrome/pasteboard/
  card/border/divider/text/muted/faint/icon/hover/pressed/selected/
  accent/accent-soft/field/danger).
- No upstream ArtCraft identity remains anywhere in the mockups
  (no old blue, no old marks) — criterion "zero upstream visual identity"
  is satisfied by construction if you build against these files.
- Home tool cards use the catalogue's existing per-tool hues — keep them,
  they're semantic, not brand.
- dialogs-* doubles as the dialog component spec (tabs, radio rows,
  summary banner, footers) — reuse for the other ~25 dialogs.
- Deliberately unchanged: layout structure (familiar PDF-app arrangement
  per direction B), all feature labels (real strings from the code),
  engine behavior. This is a theme/chrome redesign, not a re-flow.

## Open design questions (for PM/user)
1. App icon/mark: mockups use the existing glyph set; a new Rehman-PDF
   app icon was out of scope — flag if wanted before Saima builds.
2. The ⌘K palette and All-tools catalogue screen were not mocked
   (8-file scope) — they follow the same tokens; Saima can extend, or I
   can add mockups on request.
