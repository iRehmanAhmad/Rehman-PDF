# Urdu text-extraction corpus

Three small contributor-generated Urdu PDFs for text-extraction (reading-order)
regression tests (`crates/render/src/text.rs`). No personal data; the Urdu text is
original, written for these tests.

| File | What |
|---|---|
| `urdu-paragraph.pdf` | Plain Urdu paragraph (5 lines). |
| `urdu-mixed.pdf` | Two lines mixing Urdu and English (`English Grammar`, `Science`, digits). |
| `urdu-list.pdf` | Single-column list: header `ہفتے کے دن` + 7 days of the week. |

Font: **Noto Naskh Arabic** (OFL-1.1, https://github.com/google/fonts/ofl/notonaskharabic).
The task specified Noto Nastaliq Urdu, but fpdf2's HarfBuzz shaper emits unusable
output with the variable Nastaliq font (extreme vertical cascading, broken ToUnicode),
so the corpus uses Naskh — the script Pakistani school textbooks are set in — which
shapes cleanly. The font file itself is NOT committed (AGENTS.md §1.4); only the PDFs
are committed. These PDFs need `ATTRIBUTION.toml` entries (PM task; the brief forbids
editing that file here, so `cargo xtask assets` will flag them until added).

Generation: fpdf2 **without** the shaping engine; each line is pre-ordered to visual
order with python-bidi (`get_display`) before writing, so the content streams hold
visual-order glyphs with a correct ToUnicode CMap — the same shape as real-world Urdu
PDFs produced by desktop publishers.

```sh
# Noto Naskh Arabic is OFL-1.1.
curl -sL -o /tmp/NotoNaskhArabic.ttf \
  "https://github.com/google/fonts/raw/main/ofl/notonaskharabic/NotoNaskhArabic%5Bwght%5D.ttf"
pip install fpdf2 python-bidi pymupdf
python3 - <<'EOF'
from fpdf import FPDF
from bidi.algorithm import get_display
FONT = '/tmp/NotoNaskhArabic.ttf'
def make(title):
    pdf = FPDF(); pdf.set_margins(20, 20, 20); pdf.add_page()
    pdf.add_font('urdu', '', FONT); pdf.set_font('urdu', '', 18)
    pdf.set_title(title + ' - Rehman-PDF test corpus'); pdf.set_author('Rehman-PDF')
    return pdf
vis = get_display
pdf = make('Urdu paragraph')
for ln in ["اردو ہماری قومی زبان ہے۔ یہ پاکستان کے طول و عرض میں",
    "بولی اور سمجھی جاتی ہے۔ ہر بچے کو اردو پڑھنا اور لکھنا",
    "آنا چاہیے۔ کتابیں علم کا خزانہ ہیں، اور جو بچہ کتاب",
    "پڑھتا ہے وہ کامیاب ہوتا ہے۔ استاد کا فرض ہے کہ وہ",
    "بچوں میں پڑھنے کا شوق پیدا کرے۔"]:
    pdf.cell(0, 14, vis(ln), align='L', new_x="LMARGIN", new_y="NEXT")
pdf.output('urdu-paragraph.pdf')
pdf = make('Urdu-English mixed lines')
pdf.cell(0, 14, vis("کلاس 5 کے 32 طلبہ نے English Grammar کا ٹیسٹ دیا اور 20 میں سے 18 نمبر حاصل کیے۔"), align='L', new_x="LMARGIN", new_y="NEXT")
pdf.ln(6)
pdf.cell(0, 14, vis("آج کی تاریخ 18 اکتوبر 2026 ہے۔ Science کا سبق نمبر 3 کل پڑھایا جائے گا۔"), align='L', new_x="LMARGIN", new_y="NEXT")
pdf.output('urdu-mixed.pdf')
pdf = make('Urdu word list')
pdf.cell(0, 14, vis("ہفتے کے دن"), align='R', new_x="LMARGIN", new_y="NEXT"); pdf.ln(4)
for day in ["پیر", "منگل", "بدھ", "جمعرات", "جمعہ", "ہفتہ", "اتوار"]:
    pdf.cell(0, 14, vis(day), align='R', new_x="LMARGIN", new_y="NEXT")
pdf.output('urdu-list.pdf')
EOF
```

Expected extraction (logical/reading order), asserted by the tests in `text.rs`:

- `urdu-paragraph.pdf` → the five Urdu lines above, in typing order.
- `urdu-mixed.pdf` → `کلاس 5 کے 32 طلبہ نے English Grammar کا ٹیسٹ دیا اور 20 میں سے 18 نمبر حاصل کیے۔`
  then `آج کی تاریخ 18 اکتوبر 2026 ہے۔ Science کا سبق نمبر 3 کل پڑھایا جائے گا۔`
- `urdu-list.pdf` → `ہفتے کے دن`, then each day name on its own line.

Known limitation: multi-column tables mixing RTL and LTR columns are NOT handled —
`reverse_rtl` (`crates/render/src/text.rs`) does paragraph-level bidi recovery, not
column-aware layout. A two-column Urdu↔English word list was tried and dropped for
this reason; the single-column list above is the committed fixture.
