# Barka (Designer) — T-003 app-icon concepts: findings

User asked for 3 icon concepts to choose from (2026-10-19). All follow the locked
"T-003 Modern Minimal" theme: indigo-led (`#4F46E5` light / `#818CF8` dark family),
flat, 2–3 colors max, no tiny text, no lorem ipsum.

## The three concepts

**A — "R" monogram + PDF page fold** (`concept-a.svg/png`)
Indigo-gradient rounded tile (`#5B54F0`→`#4338CA`) carrying a white document page
with a folded top-right corner (`#A5B4FC` fold) and a geometric **R** drawn in
indigo `#4F46E5` (round-cap strokes: stem + P-bowl + diagonal leg). The most
literal option: name + "it's a PDF app" in one glance.

**B — Urdu accent** (`concept-b.svg/png`)
White rounded tile (`#D9E1FD` hairline so it holds on light backgrounds) with a
single bold Nastaliq-style sweeping descender in indigo `#4F46E5`, plus a diamond
dot above the stroke echoing Urdu letter dots. Abstract and minimal — no literal
text, no cliché. The Urdu-identity option.

**C — Abstract stacked sheets** (`concept-c.svg/png`)
Indigo-gradient tile (`#5B54F0`→`#3730A3`) with three fanned document sheets:
two translucent white sheets behind, one solid white front sheet with folded
corner and three light-indigo text bars. Pure "documents/PDF" abstraction, no
letters at all.

## Design decisions & iterations (impeccable: checked renders, not labels)
- A shipped on first render — the R/page/fold composition was clean immediately.
- B went through 3 iterations: v1 had a disconnected floating dot and a pointless
  baseline bar (removed); a v3 "echo stroke" experiment (thin parallel curve for
  reed-pen contrast) read as two worms and was reverted; final = single bold
  swoosh + dot, scaled 1.14× for small-size ink coverage, stronger tile hairline.
- C shipped on first render; fan angles (9°/−7°) verified not to clip the tile.
- Rendering: hand-written SVG (512×512 viewBox), PNG previews via cairosvg 2.9.1
  (installed in a throwaway venv `/tmp/iconenv` — not committed).

## Legibility verification (`contact-sheet.png`)
Each concept rendered at 512 on light (`#F5F5F7`) and dark (`#17171A`), plus 48px
and 16px upscaled 4× with nearest-neighbor for honest pixel inspection:
- **A: PASS** — R reads clearly down to 16px on both backgrounds.
- **B: PASS (weakest at 16px)** — at 16px it becomes an abstract indigo mark;
  acceptable for the category (cf. swoosh-type marks), ink coverage improved in
  the final scale-up. White tile holds on dark; hairline defines it on light.
- **C: PASS** — sheet fan + fold silhouette holds at 16px on both backgrounds.

## How to verify
- Open `icons/concept-{a,b,c}.svg` in any browser (they are self-contained).
- `icons/contact-sheet.png` shows all three at 512/48/16 on light + dark.
- Re-render: `/tmp/iconenv/bin/python -c "import cairosvg; cairosvg.svg2png(url='concept-a.svg', write_to='x.png', output_width=512, output_height=512)"`.

## Open / next
- Winner gets exported to the full hicolor set (`16/24/32/48/64/128/256/512` +
  scalable SVG) replacing `assets/app-icon/.../ai.storyteller.pdfcraft.*` —
  AFTER the user picks. App source untouched so far.
- My take for the PM: A is the safest (instantly readable, name-linked), B is
  the most distinctive (only one with real Urdu character), C is the most
  "app-store generic". No wrong answer — it's a taste call.
