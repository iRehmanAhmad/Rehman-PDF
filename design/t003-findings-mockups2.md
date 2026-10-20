# Barka (Designer) — T-003 round 2: ⌘K palette + All-tools catalogue mockups

User asked for the 2 screens flagged out-of-scope in the first handoff (2026-10-19).
Same Modern Minimal theme/tokens as the 8 existing mockups; all names are real
command strings from the codebase (`crates/engine/src/commands.rs`,
`crates/ui-egui/src/*.rs`), blurbs describe the real features — no lorem ipsum.

## Files (in `~/workspace/dev-team/tasks/T-003/mockups/`, mirrored to `design/t003-mockups/`)

**`palette-light.html` / `palette-dark.html` — ⌘K command palette**
Floating card (radius 16, card bg, 24/70 shadow) with search input showing a live
query `page` + blinking caret, 8 real results for "page" (Organize pages, Page
thumbnails, Go to page…, Rotate pages…, Insert pages from file…, Delete pages,
Extract pages…, Number pages…), each with tool-hue tile + the tool group it belongs
to (Organize/View/Navigate), match substring highlighted in accent semibold,
selected row = accent-soft fill. Footer: ↑↓ navigate · ↵ run · esc close.

**`catalogue-light.html` / `catalogue-dark.html` — All-tools catalogue**
Full page: "All tools" header + search field, 7 real tool groups (Organize 9,
Create 1, Edit 4, Review 5, Forms 7, Protect & Sign 4, Measure & Access 3 = 33
cards), each card = 40px tool-hue tile + real command name + functional blurb.
Tile hues: Organize blue · Create green · Edit orange · Review pink ·
Forms purple · Protect indigo · Measure neutral. Dark mode uses the deep tints
per the theme spec (verified no light-only hex remains).

## Design notes for Saima
- Palette rows: no shortcut chips — group label only (upstream contract).
- Catalogue grid is 3 columns; in the `ur` locale it flows RTL, Latin names keep
  bidi isolation.
- Glyphs are 1.8px round-cap inline SVGs matching the iconography spec; reuse
  freely for the remaining ~25 dialogs.

## Verification
- HTML tag-balance check passed on all 4 files (python html.parser).
- Dark files generated from light via token swap; grepped clean of light-only
  hex values (one missed radio-dot fill caught and fixed).
- Not pushed. App source untouched.
