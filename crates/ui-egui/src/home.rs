//! Home tab: recommended tools, open card, pinned folders and recent files (local only, never
//! another app's list).

use egui::{Align2, Color32, CornerRadius, Rect, Sense, Stroke, vec2};
use pdfcraft_engine::catalog;

use crate::theme::{self, Tokens};
use crate::{PdfCraftApp, icons, panels::human_size, widgets};

const RECOMMENDED: [&str; 5] = ["organize", "comment", "form", "edit", "protect"];

/// A soft shadow that rises on hover and settles when the card is pressed.
fn card_lift(ui: &egui::Ui, t: &Tokens, rect: Rect, press: theme::Press) {
    let lift = press.hover * (1.0 - press.down);
    if lift > 0.04 {
        let shadow = egui::Shadow { offset: [0, 3], blur: (14.0 * lift) as u8, spread: 0, color: t.page_shadow.gamma_multiply(0.85 * lift) };
        ui.painter().add(shadow.as_shape(rect, CornerRadius::same(12)));
    }
}

/// Paint one "Rehman-PDF on the web" button at its rect; true when clicked.
fn paint_web_button(ui: &mut egui::Ui, t: &Tokens, rect: Rect, def: (&str, &str, &'static str, bool), font: &egui::FontId, rtl: bool) -> bool {
    let (icon_name, label, cmd, primary) = def;
    let resp = ui.interact(rect, ui.id().with(("web-btn", cmd)), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    let press = theme::Press::track(ui, &resp);
    let fill = if primary { t.accent } else { press.fill(t.field, t.hover, t.pressed) };
    let stroke = if primary { Stroke::NONE } else { Stroke::new(1.0, t.border) };
    ui.painter().rect(rect, CornerRadius::same(9), fill, stroke, egui::StrokeKind::Inside);
    let body = rect.translate(press.offset());
    let ink = if primary { Color32::WHITE } else { t.text };
    let glyph = if primary { Color32::WHITE } else { t.icon };
    // Icon + label start at the inline-start edge of the button.
    let sx = if rtl { body.right() - 15.0 } else { body.left() + 15.0 };
    let icon_rect = Rect::from_min_size(egui::pos2(if rtl { sx - 15.0 } else { sx }, body.top()), vec2(15.0, 34.0));
    icons::paint(ui, icon_rect, icon_name, 15.0, glyph);
    let tx = if rtl { sx - 15.0 - 8.0 } else { sx + 15.0 + 8.0 };
    ui.painter().text(egui::pos2(tx, body.center().y), if rtl { Align2::RIGHT_CENTER } else { Align2::LEFT_CENTER }, label, font.clone(), ink);
    theme::hand(resp).clicked()
}

/// "Rehman-PDF on the web" card buttons (T-003 mockup): a primary indigo GitHub
/// button and an outlined rehmanahmad.com button. Returns the command to run.
/// Side by side while they fit; stacked vertically below ~380 pt content width,
/// where the second button would otherwise paint past the card (T-019).
fn web_buttons(ui: &mut egui::Ui) -> Option<&'static str> {
    let t = Tokens::get(ui.ctx());
    let rtl = crate::i18n::current().rtl();
    let font = theme::semibold(12.5);
    let defs = [("code-xml", tl!("GitHub"), "help.github", true), ("globe", tl!("rehmanahmad.com"), "help.website", false)];
    let widths: Vec<f32> =
        defs.iter().map(|(_, label, _, _)| ui.fonts_mut(|f| f.layout_no_wrap((*label).to_owned(), font.clone(), t.text).size().x) + 53.0).collect();
    let total = widths.iter().sum::<f32>() + 10.0;
    let avail = ui.available_width();
    let mut clicked = None;
    if total <= avail {
        let (row, _) = ui.allocate_exact_size(vec2(total, 34.0), Sense::hover());
        let mut order: Vec<usize> = (0..defs.len()).collect();
        if rtl {
            order.reverse();
        }
        let mut cx = if rtl { row.right() } else { row.left() };
        for i in order {
            let w = widths[i];
            let rect = Rect::from_min_size(egui::pos2(if rtl { cx - w } else { cx }, row.top()), vec2(w, 34.0));
            cx = if rtl { cx - w - 10.0 } else { cx + w + 10.0 };
            if paint_web_button(ui, &t, rect, defs[i], &font, rtl) {
                clicked = Some(defs[i].2);
            }
        }
    } else {
        let (area, _) = ui.allocate_exact_size(vec2(avail, 34.0 * 2.0 + 10.0), Sense::hover());
        let mut y = area.top();
        for (i, def) in defs.iter().enumerate() {
            let w = widths[i].min(avail);
            let x = if rtl { area.right() - w } else { area.left() };
            let rect = Rect::from_min_size(egui::pos2(x, y), vec2(w, 34.0));
            if paint_web_button(ui, &t, rect, *def, &font, rtl) {
                clicked = Some(def.2);
            }
            y += 44.0;
        }
    }
    clicked
}

/// Dashed rounded-rectangle outline (the open-file dropzone). egui has no dashed
/// stroke, so the outline is drawn as short segments along the rounded-rect path.
fn dashed_rounded_rect(ui: &egui::Ui, rect: Rect, radius: f32, stroke: Stroke) {
    const DASH: f32 = 7.0;
    const GAP: f32 = 5.0;
    fn push(path: &mut Vec<egui::Pos2>, p: egui::Pos2) {
        if path.last() != Some(&p) {
            path.push(p);
        }
    }
    fn arc(path: &mut Vec<egui::Pos2>, c: egui::Pos2, r: f32, a0: f32, a1: f32) {
        for i in 1..=12 {
            let a = (a0 + (a1 - a0) * i as f32 / 12.0).to_radians();
            push(path, c + vec2(a.cos() * r, a.sin() * r));
        }
    }
    let r = radius.min(rect.width() / 2.0).min(rect.height() / 2.0);
    // Closed path, clockwise from the top edge, with arcs sampled into points.
    let mut path: Vec<egui::Pos2> = Vec::new();
    push(&mut path, egui::pos2(rect.left() + r, rect.top()));
    push(&mut path, egui::pos2(rect.right() - r, rect.top()));
    arc(&mut path, egui::pos2(rect.right() - r, rect.top() + r), r, -90.0, 0.0);
    push(&mut path, egui::pos2(rect.right(), rect.bottom() - r));
    arc(&mut path, egui::pos2(rect.right() - r, rect.bottom() - r), r, 0.0, 90.0);
    push(&mut path, egui::pos2(rect.left() + r, rect.bottom()));
    arc(&mut path, egui::pos2(rect.left() + r, rect.bottom() - r), r, 90.0, 180.0);
    push(&mut path, egui::pos2(rect.left(), rect.top() + r));
    arc(&mut path, egui::pos2(rect.left() + r, rect.top() + r), r, 180.0, 270.0);
    // Walk the path, emitting a dash wherever the running distance falls in one.
    let painter = ui.painter();
    let mut acc = 0.0;
    let n = path.len();
    let mut i = 0;
    while i < n {
        let a = path[i];
        let b = path[(i + 1) % n];
        let len = a.distance(b);
        if len > 0.0 {
            let mut s = 0.0;
            while s < len {
                let cycle = acc % (DASH + GAP);
                let in_dash = cycle < DASH;
                let remain = if in_dash { DASH - cycle } else { DASH + GAP - cycle };
                let piece = remain.min(len - s);
                if in_dash && piece > 0.01 {
                    painter.line_segment([a.lerp(b, s / len), a.lerp(b, (s + piece) / len)], stroke);
                }
                s += piece;
                acc += piece;
            }
        }
        i += 1;
    }
}

/// The home-screen tool card blurbs (T-003 mockup; translated like any UI string).
fn tool_blurb(id: &str) -> &'static str {
    match id {
        "organize" => "Select, rotate, delete, insert and reorder pages · deep undo",
        "comment" => "Sticky notes, highlights and markup · summarize to a report",
        "form" => "Add fillable fields, checkboxes and signatures · bulk edit",
        "edit" => "Fix text, images and links in place · RTL-aware editing",
        "protect" => "Passwords and permissions · certificates and redaction",
        _ => "",
    }
}

/// One vertical tool card (T-003 mockup): a tinted 40 px glyph tile, a one-line
/// title, a two-line blurb and the "Use now →" action. In RTL the card order is
/// reversed by the caller and the internals mirror to the inline-start edge.
fn tool_card(app: &mut PdfCraftApp, ui: &mut egui::Ui, t: &Tokens, g: &'static catalog::ToolGroup, w: f32, h: f32, rtl: bool) {
    let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tl!(g.label)));
    let press = theme::Press::track(ui, &resp);
    card_lift(ui, t, rect, press);
    let fill = press.fill(t.card, t.hover, t.pressed);
    ui.painter().rect(rect, CornerRadius::same(12), fill, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    let body = rect.translate(press.offset());
    let pad = 16.0;
    let text_w = w - 2.0 * pad;
    // Inline-start x; in RTL everything anchors to the right edge.
    let ix = if rtl { body.right() - pad } else { body.left() + pad };
    let tile_x = if rtl { ix - 40.0 } else { ix };
    let tile_rect = Rect::from_min_size(egui::pos2(tile_x, body.top() + pad), vec2(40.0, 40.0));
    let (tint, stroke) = t.glyph_tile(g.hue);
    ui.painter().rect_filled(tile_rect, CornerRadius::same(11), tint);
    icons::paint(ui, tile_rect, g.icon, 21.0, stroke);

    let mut title_job = egui::text::LayoutJob::simple_singleline(tl!(g.label).to_owned(), theme::semibold(13.5), t.text);
    title_job.wrap = egui::text::TextWrapping { max_width: text_w, max_rows: 1, break_anywhere: true, ..Default::default() };
    if rtl {
        title_job.halign = egui::Align::RIGHT;
    }
    let title = ui.fonts_mut(|f| f.layout_job(title_job));
    let title_elided = title.elided;
    let title_y = body.top() + pad + 40.0 + 12.0;
    ui.painter().galley(egui::pos2(if rtl { ix - text_w } else { ix }, title_y), title, t.text);
    let title_h = ui.fonts_mut(|f| f.layout_no_wrap(tl!(g.label).to_owned(), theme::semibold(13.5), t.text).size().y);

    // The blurb is two lines at most, so the action below always has room —
    // in every language (the translated-action test pins this).
    let blurb = tl!(tool_blurb(g.id));
    let mut job = egui::text::LayoutJob::simple(blurb.to_owned(), theme::regular(12.0), t.text_muted, text_w);
    job.wrap.max_rows = 2;
    if rtl {
        job.halign = egui::Align::RIGHT;
    }
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    let blurb_elided = galley.elided;
    let blurb_pos = egui::pos2(if rtl { ix - text_w } else { ix }, title_y + title_h + 4.0);
    ui.painter().galley(blurb_pos, galley, t.text_muted);

    // "Use now →" from the mockup; the arrow is its own shape so the
    // translated-action test still finds the bare label. In RTL the arrow
    // flips direction and leads the label.
    let action_font = theme::semibold(12.0);
    let action_y = body.bottom() - pad;
    let arrow = if rtl { "←" } else { "→" };
    let label = tl!("Use now");
    let label_w = ui.fonts_mut(|f| f.layout_no_wrap(label.to_owned(), action_font.clone(), t.accent).size().x);
    let arrow_w = ui.fonts_mut(|f| f.layout_no_wrap(arrow.to_owned(), action_font.clone(), t.accent).size().x);
    if rtl {
        ui.painter().text(egui::pos2(ix, action_y), Align2::RIGHT_CENTER, arrow, action_font.clone(), t.accent);
        ui.painter().text(egui::pos2(ix - arrow_w - 4.0, action_y), Align2::RIGHT_CENTER, label, action_font.clone(), t.accent);
    } else {
        ui.painter().text(egui::pos2(ix, action_y), Align2::LEFT_CENTER, label, action_font.clone(), t.accent);
        ui.painter().text(egui::pos2(ix + label_w + 4.0, action_y), Align2::LEFT_CENTER, arrow, action_font.clone(), t.accent);
    }
    let resp = if title_elided || blurb_elided { resp.on_hover_text(format!("{}\n{blurb}", tl!(g.label))) } else { resp };
    if theme::hand(resp).clicked() {
        app.select_tool(g.id);
    }
}

/// Like [`widgets::row_text`], but anchored to a right edge (for RTL rows).
fn row_text_end(ui: &egui::Ui, right: f32, center_y: f32, text: impl Into<String>, font: egui::FontId, color: Color32, max_width: f32) {
    let mut job = egui::text::LayoutJob::simple_singleline(text.into(), font, color);
    job.wrap = egui::text::TextWrapping::truncate_at_width(max_width.max(0.0));
    let galley = ui.painter().layout_job(job);
    let w = galley.size().x.min(max_width.max(0.0));
    ui.painter().galley(egui::pos2(right - w, center_y - galley.size().y / 2.0), galley, color);
}

/// Two-half breakpoint (T-019, Barka §1): the home page splits into true 50/50
/// halves only when the content width reaches this; below it the halves stack,
/// tools first, Recent below. 1024 pt content → 500 pt halves, the narrowest
/// the tool cards survive honestly.
const TWO_HALF_MIN_CONTENT: f32 = 1024.0;
/// Gutter between the halves (matches the page's 26 pt vertical rhythm).
const HALF_GUTTER: f32 = 24.0;

/// Columns for the Recommended tools grid at the given width (T-019, Barka
/// §3): 3 columns down to ~528 pt, then 2, then 1 — the old fixed 3-column
/// formula becomes the overflow source below that.
fn grid_columns(width: f32) -> usize {
    if width >= 528.0 {
        3
    } else if width >= 330.0 {
        2
    } else {
        1
    }
}

/// The tools half: Welcome + tagline, the web card, the Recommended tools grid
/// and the pinned folders — the existing content, in the existing order.
fn tools_half(app: &mut PdfCraftApp, ui: &mut egui::Ui, t: &Tokens) {
    ui.label(egui::RichText::new(tl!("Welcome to Rehman-PDF")).font(theme::semibold(24.0)));
    ui.label(
        egui::RichText::new(tl!("An open-source PDF workbench — local, private, and scriptable.")).color(t.text_muted).font(theme::regular(14.0)),
    );
    ui.add_space(14.0);
    egui::Frame::NONE
        .fill(t.card)
        .stroke(Stroke::new(1.0, t.border))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin { left: 18, right: 18, top: 16, bottom: 16 })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(egui::RichText::new(tl!("Rehman-PDF on the web")).font(theme::semibold(15.0)));
            ui.label(egui::RichText::new(tl!("Project page, source code and the author's site.")).color(t.text_muted));
            ui.add_space(10.0);
            if let Some(cmd) = web_buttons(ui) {
                app.execute(cmd);
            }
        });
    ui.add_space(26.0);

    // Recommended tools: a grid of vertical tool cards plus the dashed
    // open-file dropzone (T-003 mockup). In RTL the card order reverses.
    ui.label(egui::RichText::new(tl!("Recommended tools")).font(theme::semibold(15.0)));
    ui.add_space(12.0);
    {
        let rtl = crate::i18n::current().rtl();
        let gap = 14.0;
        let cols = grid_columns(ui.available_width());
        let col_w = (ui.available_width() - (cols as f32 - 1.0) * gap) / cols as f32;
        let card_h = 178.0;
        let mut order: Vec<&str> = RECOMMENDED.into_iter().collect();
        if rtl {
            order.reverse();
        }
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(gap, gap);
            for id in order {
                let Some(g) = catalog::group(id) else { continue };
                tool_card(app, ui, t, g, col_w, card_h, rtl);
            }
            let (rect, resp) = ui.allocate_exact_size(vec2(col_w, card_h), Sense::click());
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tl!("Open file")));
            let press = theme::Press::track(ui, &resp);
            card_lift(ui, t, rect, press);
            ui.painter().rect_filled(rect, CornerRadius::same(12), press.fill(t.pasteboard, t.hover, t.pressed));
            dashed_rounded_rect(ui, rect, 12.0, Stroke::new(1.5, t.border));
            let body = rect.translate(press.offset());
            icons::paint(ui, Rect::from_center_size(body.center() - vec2(0.0, 30.0), vec2(28.0, 28.0)), "folder-open", 26.0, t.icon);
            ui.painter().text(body.center() + vec2(0.0, 10.0), Align2::CENTER_CENTER, tl!("Open file"), theme::medium(13.0), t.text);
            ui.painter().text(
                body.center() + vec2(0.0, 30.0),
                Align2::CENTER_CENTER,
                tl!("or drop a PDF anywhere"),
                theme::regular(11.5),
                t.text_faint,
            );
            if theme::hand(resp).clicked() {
                app.open_dialog();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Tightened so the folder list (and its "Show more" link) stays above
        // the fold on typical window heights (T-003 fix round 2).
        ui.add_space(16.0);
        crate::folders_ui::section(app, ui);
    }
}

/// The Clear button for the Recent header (ghost button, trash icon).
fn clear_button(ui: &mut egui::Ui) -> bool {
    let resp = widgets::ghost_button(ui, "trash-2", tl!("Clear"));
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tl!("Clear Recent Files")));
    resp.on_hover_text(tl!("Clear Recent Files")).clicked()
}

/// The Recent half: the existing Recent design moved into its own half
/// (T-019) — same rows, same card, header moved inside it. The header uses
/// the web card's language (inner margin 18/16, title semibold 15, Clear
/// ghost button at the header's inline-end, shown only when non-empty).
/// A file deleted while the app runs paints greyed (Barka §4): faint name
/// and path, neutral tile, no pages·size detail, no hover wash, no pointing
/// hand. Clicking it still goes through `open_recent`, which prunes the
/// entry with a plain toast (Asma §2) — the row itself never opens a file
/// that isn't there.
fn recent_half(app: &mut PdfCraftApp, ui: &mut egui::Ui, t: &Tokens) {
    let rtl = crate::i18n::current().rtl();
    let mut clear = false;
    let mut open = None;
    let mut remove = None;
    egui::Frame::NONE
        .fill(t.card)
        .stroke(Stroke::new(1.0, t.border))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(egui::Margin { left: 18, right: 18, top: 16, bottom: 16 })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            // Header inside the card: title at the inline-start edge, Clear at
            // the inline-end (mirrored in RTL, like the rows below).
            if rtl {
                ui.horizontal(|ui| {
                    if !app.recent.is_empty() {
                        clear = clear_button(ui);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(tl!("Recent")).font(theme::semibold(15.0)));
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(tl!("Recent")).font(theme::semibold(15.0)));
                    if !app.recent.is_empty() {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            clear = clear_button(ui);
                        });
                    }
                });
            }
            if app.recent.is_empty() {
                // The existing already-translated line, inside the card, muted,
                // inline-start aligned like every other line in the panel.
                ui.add_space(10.0);
                if rtl {
                    ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                        ui.label(
                            egui::RichText::new(tl!("Files you open in Rehman-PDF appear here. Drop a PDF anywhere to open it.")).color(t.text_muted),
                        );
                    });
                } else {
                    ui.label(
                        egui::RichText::new(tl!("Files you open in Rehman-PDF appear here. Drop a PDF anywhere to open it.")).color(t.text_muted),
                    );
                }
                return;
            }
            ui.add_space(12.0);
            // Rows separated by dividers. At very narrow widths the pages·size
            // detail is the first thing to drop (name + path stay).
            let show_detail = ui.available_width() >= 340.0;
            let mut first = true;
            for r in &app.recent {
                // Restore already filters dead entries at startup; a greyed
                // entry is only a file deleted or moved while the app runs.
                #[cfg(not(target_arch = "wasm32"))]
                let missing = !std::path::Path::new(&r.path).exists();
                #[cfg(target_arch = "wasm32")]
                let missing = false;
                let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 60.0), Sense::click());
                if !first {
                    ui.painter().hline(rect.left()..=rect.right(), rect.top(), Stroke::new(1.0, t.divider));
                }
                first = false;
                resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &r.name));
                let press = theme::Press::track(ui, &resp);
                // The row, not its response: the pointer stays "in" the row over its remove button.
                let hovered = ui.rect_contains_pointer(rect);
                if !missing {
                    if hovered && press.hover < 0.04 && press.down < 0.04 {
                        // The remove button sits in the row, so the row response may not be hovered.
                        ui.painter().rect_filled(rect, CornerRadius::same(8), t.hover);
                    } else {
                        press.wash(ui, rect, 8, false);
                    }
                }
                if hovered {
                    // Remove just this file from the list (#430); the file itself is untouched.
                    let x_center = if rtl { rect.left_center() + vec2(26.0, 0.0) } else { rect.right_center() - vec2(26.0, 0.0) };
                    let x_rect = Rect::from_center_size(x_center, vec2(28.0, 28.0));
                    let x = ui.interact(x_rect, ui.id().with(("remove-recent", &r.path)), Sense::click());
                    let label = crate::i18n::fmt(tl!("Remove {name} from Recent"), &[("name", &r.name)]);
                    x.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &label));
                    if x.hovered() {
                        ui.painter().rect_filled(x_rect, CornerRadius::same(6), t.pressed);
                    }
                    icons::paint(ui, x_rect.shrink(6.0), "x", 16.0, t.icon);
                    if x.on_hover_text(tl!("Remove from Recent")).clicked() {
                        remove = Some(r.path.clone());
                    }
                }
                let body = rect.translate(press.offset());
                // File tile at the inline-start edge: danger-tinted for a live
                // file, neutral for one that is no longer there.
                let tile_x = if rtl { body.right() - 16.0 - 36.0 } else { body.left() + 16.0 };
                let tile = Rect::from_min_size(egui::pos2(tile_x, body.top() + 12.0), vec2(36.0, 36.0));
                if missing {
                    ui.painter().rect_filled(tile, CornerRadius::same(9), t.pasteboard);
                    icons::paint(ui, tile, "file-text", 19.0, t.text_faint);
                } else {
                    ui.painter().rect_filled(tile, CornerRadius::same(9), t.danger_tile());
                    icons::paint(ui, tile, "file-text", 19.0, t.danger);
                }
                // Page count and size at the inline-end edge — dropped for a
                // missing file (it describes a file that isn't there).
                let detail = if show_detail && !missing {
                    Some(ui.painter().text(
                        if rtl { body.left_center() + vec2(48.0, 0.0) } else { body.right_center() - vec2(48.0, 0.0) },
                        if rtl { Align2::LEFT_CENTER } else { Align2::RIGHT_CENTER },
                        format!("{}  ·  {}", crate::i18n::trn(crate::i18n::current(), r.pages as u64, "{n} page", "{n} pages"), human_size(r.size)),
                        theme::regular(12.0),
                        t.text_faint,
                    ))
                } else {
                    None
                };
                // The name and path stop short of the detail (or the row's
                // inline-end edge when the detail is dropped); hovering shows
                // the whole path.
                let text_edge = if rtl { body.right() - 65.0 } else { body.left() + 65.0 };
                let text_end = match detail {
                    Some(d) if rtl => d.right() + 16.0,
                    Some(d) => d.left() - 16.0,
                    None if rtl => body.left() + 16.0,
                    None => body.right() - 16.0,
                };
                let width = if rtl { text_edge - text_end } else { text_end - text_edge };
                let name_color = if missing { t.text_faint } else { t.text };
                if rtl {
                    row_text_end(ui, text_edge, body.top() + 21.0, crate::bidi::visual(&r.name), theme::semibold(13.0), name_color, width);
                    row_text_end(ui, text_edge, body.top() + 40.0, crate::bidi::visual(&r.path), theme::regular(11.5), t.text_faint, width);
                } else {
                    widgets::row_text(
                        ui,
                        egui::pos2(text_edge, body.top() + 21.0),
                        crate::bidi::visual(&r.name),
                        theme::semibold(13.0),
                        name_color,
                        width,
                    );
                    widgets::row_text(
                        ui,
                        egui::pos2(text_edge, body.top() + 40.0),
                        crate::bidi::visual(&r.path),
                        theme::regular(11.5),
                        t.text_faint,
                        width,
                    );
                }
                if missing {
                    // A dead row stays clickable, but it never opens a file
                    // that isn't there: open_recent prunes it with a plain
                    // note (Asma §2). No pointing hand on a dead row.
                    if resp.on_hover_text(&r.path).clicked() {
                        open = Some(r.path.clone());
                    }
                } else if theme::hand(resp).on_hover_text(&r.path).clicked() {
                    open = Some(r.path.clone());
                }
            }
        });
    if let Some(p) = open {
        app.open_recent(&p);
    }
    if let Some(p) = remove {
        app.recent.retain(|r| r.path != p);
    }
    if clear {
        app.execute("file.clear_recent");
    }
}

pub fn show(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        egui::Frame::NONE.inner_margin(egui::Margin { left: 36, right: 36, top: 28, bottom: 28 }).show(ui, |ui| {
            let content_w = ui.available_width();
            let rtl = crate::i18n::current().rtl();
            if content_w >= TWO_HALF_MIN_CONTENT {
                // True 50/50 halves with a 24 pt gutter (T-019, Barka §1).
                // The Recent panel's top edge aligns with the Welcome title's
                // top — no dead band above Recent. In RTL the halves mirror:
                // tools at the inline-start (right), Recent at the left.
                let half = (content_w - HALF_GUTTER) / 2.0;
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let mut halves = [true, false]; // true = tools half
                    if rtl {
                        halves.reverse();
                    }
                    for (i, is_tools) in halves.iter().enumerate() {
                        if i > 0 {
                            ui.add_space(HALF_GUTTER);
                        }
                        ui.vertical(|ui| {
                            ui.set_width(half);
                            if *is_tools {
                                tools_half(app, ui, &t);
                            } else {
                                recent_half(app, ui, &t);
                            }
                        });
                    }
                });
            } else {
                // Narrow: stack, tools first, Recent below (Barka §3).
                tools_half(app, ui, &t);
                ui.add_space(26.0);
                recent_half(app, ui, &t);
            }
            ui.add_space(26.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(tl!("Free and open source · your files never leave this computer"))
                        .font(theme::regular(12.0))
                        .color(t.text_faint),
                );
            });
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translated_card_description_leaves_room_for_the_action() {
        let de = crate::i18n::Lang::from_code("de").unwrap();
        crate::i18n::set_current(de);
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        let mut app = PdfCraftApp::new();
        let blurb = crate::i18n::tr(de, tool_blurb("form"));
        let output = ctx
            .run_ui(egui::RawInput { screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))), ..Default::default() }, |ui| {
                show(&mut app, ui)
            });
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let text_shapes: Vec<_> =
            output.shapes.iter().filter_map(|shape| if let egui::Shape::Text(text) = &shape.shape { Some(text) } else { None }).collect();
        let description = text_shapes.iter().find(|text| text.galley.job.text == blurb).unwrap();
        let action = text_shapes
            .iter()
            .find(|text| text.galley.job.text == crate::i18n::tr(de, "Use now") && (text.pos.x - description.pos.x).abs() < 0.1)
            .unwrap();
        assert!(description.galley.rows.len() <= 2);
        assert!(
            description.pos.y + description.galley.size().y + 4.0 <= action.pos.y,
            "the translated description must leave a visible gap before its action"
        );
        output.drop_without_applying_deltas();
    }

    /// The home screen follows the T-003 mockup: hero, web card with the two
    /// buttons, a 3-column grid of 5 vertical tool cards plus the dropzone, and
    /// the footer. This pins the structure (not exact pixels).
    #[test]
    fn home_layout_matches_mockup() {
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        let mut app = PdfCraftApp::new();
        let output = ctx
            .run_ui(egui::RawInput { screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))), ..Default::default() }, |ui| {
                show(&mut app, ui)
            });
        let texts: Vec<&egui::epaint::TextShape> =
            output.shapes.iter().filter_map(|shape| if let egui::Shape::Text(text) = &shape.shape { Some(text) } else { None }).collect();
        let has = |s: &str| texts.iter().any(|t| t.galley.job.text == s);
        for s in [
            "Welcome to Rehman-PDF",
            "An open-source PDF workbench — local, private, and scriptable.",
            "Rehman-PDF on the web",
            "Project page, source code and the author's site.",
            "GitHub",
            "rehmanahmad.com",
            "Recommended tools",
            "Organize pages",
            "Add comments",
            "Prepare a form",
            "Edit a PDF",
            "Protect a PDF",
            "Open file",
            "or drop a PDF anywhere",
            "Free and open source · your files never leave this computer",
        ] {
            assert!(has(s), "home screen is missing {s:?}");
        }
        assert_eq!(texts.iter().filter(|t| t.galley.job.text == "Use now").count(), 5, "one action per tool card");
        // The five tool blurbs are all painted.
        for id in RECOMMENDED {
            assert!(has(tool_blurb(id)), "missing blurb for {id}");
        }
        // Glyph tiles use the mockup's tinted colors (light theme).
        let tints = [
            (egui::Color32::from_rgb(0xE7, 0xF6, 0xE9), "organize"),
            (egui::Color32::from_rgb(0xFF, 0xF1, 0xE2), "comment"),
            (egui::Color32::from_rgb(0xF0, 0xEA, 0xFB), "form"),
            (egui::Color32::from_rgb(0xFD, 0xEB, 0xF3), "edit"),
            (egui::Color32::from_rgb(0xE5, 0xEB, 0xFD), "protect"),
        ];
        let rects: Vec<&egui::epaint::RectShape> =
            output.shapes.iter().filter_map(|shape| if let egui::Shape::Rect(r) = &shape.shape { Some(r) } else { None }).collect();
        for (tint, id) in tints {
            assert!(
                rects.iter().any(|r| r.fill == tint && (r.rect.width() - 40.0).abs() < 0.5 && (r.rect.height() - 40.0).abs() < 0.5),
                "missing 40px glyph tile in {tint:?} for {id}"
            );
        }
        // The "Use now" actions are the indigo accent, 12 px.
        let accent = egui::Color32::from_rgb(0x4F, 0x46, 0xE5);
        for action in texts.iter().filter(|t| t.galley.job.text == "Use now") {
            assert!(action.galley.job.sections.iter().all(|s| s.format.color == accent), "action must be accent-colored");
            let size = action.galley.job.sections.first().map(|s| s.format.font_id.size).unwrap_or(0.0);
            assert!((size - 12.0).abs() < 0.01, "action must be 12px, got {size}");
        }
        output.drop_without_applying_deltas();
    }

    /// Render Home at `width` × 900 and return the frame's shapes.
    fn render_home(width: f32, app: &mut PdfCraftApp) -> Vec<egui::epaint::ClippedShape> {
        render_home_sized(width, 900.0, app)
    }

    /// Render Home at `width` × `height`. Stacked-layout tests use a tall
    /// viewport: below the breakpoint the tools content is taller than a
    /// 900 pt window, and egui culls what the scroll area clips off-screen,
    /// so the Recent heading would not paint at all in a short frame.
    fn render_home_sized(width: f32, height: f32, app: &mut PdfCraftApp) -> Vec<egui::epaint::ClippedShape> {
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        let output = ctx
            .run_ui(egui::RawInput { screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, height))), ..Default::default() }, |ui| {
                show(app, ui)
            });
        let shapes = output.shapes.clone();
        output.drop_without_applying_deltas();
        shapes
    }

    fn text_shapes(shapes: &[egui::epaint::ClippedShape]) -> Vec<&egui::epaint::TextShape> {
        shapes.iter().filter_map(|shape| if let egui::Shape::Text(text) = &shape.shape { Some(text) } else { None }).collect()
    }

    /// Nothing visible paints past the window's left/right edge: a shape's
    /// painted part is its rect clipped by its clip rect.
    fn assert_no_horizontal_overflow(shapes: &[egui::epaint::ClippedShape], width: f32) {
        for shape in shapes {
            let visible = shape.clip_rect.intersect(shape.shape.visual_bounding_rect());
            if visible.is_positive() {
                assert!(
                    visible.left() >= -1.0 && visible.right() <= width + 1.0,
                    "horizontal overflow at {width}px: painted {visible:?} (clip {:?})",
                    shape.clip_rect
                );
            }
        }
    }

    fn text_pos(shapes: &[egui::epaint::ClippedShape], text: &str) -> egui::Pos2 {
        text_shapes(shapes).iter().find(|t| t.galley.job.text == text).map(|t| t.pos).unwrap_or_else(|| panic!("home is missing {text:?}"))
    }

    /// T-019: the two halves never scroll sideways — at desktop widths, at the
    /// stack breakpoint, and down to the ~360 pt honest floor.
    #[test]
    fn home_two_halves_have_no_horizontal_overflow() {
        for width in [1400.0, 1100.0, 1000.0, 800.0, 600.0, 440.0, 380.0] {
            let mut app = PdfCraftApp::new();
            let shapes = render_home(width, &mut app);
            assert_no_horizontal_overflow(&shapes, width);
        }
        // With recents listed, in English and in Urdu (the long labels).
        for width in [1400.0, 600.0] {
            let mut app = PdfCraftApp::new();
            app.recent.push(crate::RecentFile {
                name: "A very long file name that will need truncating.pdf".into(),
                path: "/tmp/somewhere/deep/in/a/folder/A very long file name that will need truncating.pdf".into(),
                pages: 12,
                size: 1_234_567,
            });
            let shapes = render_home(width, &mut app);
            assert_no_horizontal_overflow(&shapes, width);
        }
        crate::i18n::set_current(crate::i18n::Lang::from_code("ur").unwrap());
        let mut app = PdfCraftApp::new();
        let shapes = render_home(600.0, &mut app);
        assert_no_horizontal_overflow(&shapes, 600.0);
        crate::i18n::set_current(crate::i18n::Lang::EN);
    }

    /// T-019: at desktop widths the halves sit side by side, Recent's top
    /// aligned with Welcome's; below the breakpoint they stack, tools first.
    #[test]
    fn home_halves_split_and_stack() {
        // Pin the locale: the language is thread-local and another test may
        // have left this thread on a different language (T-019 flake: the
        // test once ran in Urdu and "Recent" was not found in English).
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let mut app = PdfCraftApp::new();
        let shapes = render_home(1400.0, &mut app);
        let welcome = text_pos(&shapes, "Welcome to Rehman-PDF");
        let recent = text_pos(&shapes, "Recent");
        assert!(recent.x > welcome.x + 400.0, "Recent must sit in the right half at 1400px, got welcome {welcome:?} recent {recent:?}");
        assert!((recent.y - welcome.y).abs() < 80.0, "Recent's top must align with Welcome's, got welcome {welcome:?} recent {recent:?}");
        for width in [1000.0, 800.0, 600.0] {
            let mut app = PdfCraftApp::new();
            let shapes = render_home_sized(width, 3000.0, &mut app);
            let welcome = text_pos(&shapes, "Welcome to Rehman-PDF");
            let recent = text_pos(&shapes, "Recent");
            assert!(recent.y > welcome.y + 300.0, "Recent must stack below the tools at {width}px, got welcome {welcome:?} recent {recent:?}");
        }
    }

    /// T-019: in Urdu the halves mirror — tools at the inline-start (right),
    /// Recent at the left; the stack order is unchanged.
    #[test]
    fn home_halves_mirror_in_rtl() {
        crate::i18n::set_current(crate::i18n::Lang::from_code("ur").unwrap());
        let mut app = PdfCraftApp::new();
        let shapes = render_home(1400.0, &mut app);
        let welcome = text_pos(&shapes, crate::i18n::tr(crate::i18n::Lang::from_code("ur").unwrap(), "Welcome to Rehman-PDF"));
        let recent = text_pos(&shapes, crate::i18n::tr(crate::i18n::Lang::from_code("ur").unwrap(), "Recent"));
        assert!(recent.x < welcome.x, "in Urdu Recent must sit in the left half, got welcome {welcome:?} recent {recent:?}");
        let mut app = PdfCraftApp::new();
        let shapes = render_home_sized(600.0, 3000.0, &mut app);
        let welcome = text_pos(&shapes, crate::i18n::tr(crate::i18n::Lang::from_code("ur").unwrap(), "Welcome to Rehman-PDF"));
        let recent = text_pos(&shapes, crate::i18n::tr(crate::i18n::Lang::from_code("ur").unwrap(), "Recent"));
        assert!(recent.y > welcome.y + 300.0, "in Urdu Recent must still stack below the tools, got welcome {welcome:?} recent {recent:?}");
        crate::i18n::set_current(crate::i18n::Lang::EN);
    }

    /// T-019: the tools grid steps 3 → 2 → 1 columns as the half narrows.
    #[test]
    fn home_grid_columns_step_down() {
        // Pin English: the test looks up the "Use now" action label.
        crate::i18n::set_current(crate::i18n::Lang::EN);
        for (width, cols) in [(600.0, 3), (440.0, 2), (380.0, 1)] {
            let mut app = PdfCraftApp::new();
            let shapes = render_home(width, &mut app);
            let texts = text_shapes(&shapes);
            let actions: Vec<_> = texts.iter().filter(|t| t.galley.job.text == "Use now").collect();
            assert_eq!(actions.len(), 5, "five tool cards at {width}px");
            let top = actions.iter().map(|t| t.pos.y).fold(f32::INFINITY, f32::min);
            let first_row = actions.iter().filter(|t| (t.pos.y - top).abs() < 1.0).count();
            assert_eq!(first_row, cols, "first grid row at {width}px");
        }
    }

    /// T-019: recents list most-recent-first with name, path and detail, and
    /// the empty state shows the existing line inside the Recent card.
    #[test]
    fn home_recent_lists_files_most_recent_first() {
        // Pin English: the test looks up the "12 pages" detail and the
        // English empty-state line.
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let dir = std::env::temp_dir().join("rehmanpdf-t019-home-test");
        let _ = std::fs::create_dir_all(&dir);
        let first = dir.join("first.pdf");
        let second = dir.join("second.pdf");
        std::fs::write(&first, b"%PDF-1.7\n").unwrap();
        std::fs::write(&second, b"%PDF-1.7\n").unwrap();
        let mut app = PdfCraftApp::new();
        // Most recent first, as the app maintains the list: second.pdf on top.
        app.recent.push(crate::RecentFile { name: "second.pdf".into(), path: second.to_string_lossy().into_owned(), pages: 12, size: 20 });
        app.recent.push(crate::RecentFile { name: "first.pdf".into(), path: first.to_string_lossy().into_owned(), pages: 3, size: 10 });
        let shapes = render_home(1400.0, &mut app);
        let texts = text_shapes(&shapes);
        let y = |name: &str| texts.iter().find(|t| t.galley.job.text == name).map(|t| t.pos.y).unwrap();
        assert!(y("second.pdf") < y("first.pdf"), "most recent first");
        assert!(texts.iter().any(|t| t.galley.job.text.contains("12 pages")), "detail shows for a live file");
        let _ = std::fs::remove_file(first);
        let _ = std::fs::remove_file(second);

        let mut app = PdfCraftApp::new();
        let shapes = render_home(1400.0, &mut app);
        assert!(
            text_shapes(&shapes).iter().any(|t| t.galley.job.text == "Files you open in Rehman-PDF appear here. Drop a PDF anywhere to open it."),
            "empty state shows when nothing is recent"
        );
    }

    /// T-019 (Barka §4): a file deleted while the app runs paints greyed —
    /// faint name, no pages·size detail — and clicking it prunes the entry
    /// with a plain toast instead of trying to open it.
    #[test]
    fn home_missing_recent_is_greyed_and_click_prunes_it() {
        // Pin English: the test checks the English prune toast.
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let mut app = PdfCraftApp::new();
        app.recent.push(crate::RecentFile { name: "gone.pdf".into(), path: "/definitely/not/here/gone.pdf".into(), pages: 12, size: 20 });
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1400.0, 900.0));
        let output = ctx.run_ui(egui::RawInput { screen_rect: Some(screen), ..Default::default() }, |ui| show(&mut app, ui));
        let shapes = output.shapes.clone();
        output.drop_without_applying_deltas();
        let t = Tokens::get(&ctx);
        let texts = text_shapes(&shapes);
        let name = texts.iter().find(|t| t.galley.job.text == "gone.pdf").expect("missing entry paints its name");
        assert!(
            name.galley.job.sections.iter().all(|s| s.format.color == t.text_faint),
            "a missing file's name paints faint, got {:?}",
            name.galley.job.sections.iter().map(|s| s.format.color).collect::<Vec<_>>()
        );
        assert!(!text_shapes(&shapes).iter().any(|t| t.galley.job.text.contains("12 pages")), "no detail for a file that isn't there");
        // Click the row: press, then release, on the name.
        let pos = name.pos + vec2(10.0, 0.0);
        for pressed in [true, false] {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(screen),
                    events: vec![
                        egui::Event::PointerMoved(pos),
                        egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed, modifiers: egui::Modifiers::default() },
                    ],
                    ..Default::default()
                },
                |ui| show(&mut app, ui),
            );
            output.drop_without_applying_deltas();
        }
        assert!(app.recent.is_empty(), "clicking a missing entry prunes it");
        let toast = app.toast.as_ref().map(|(m, _)| m.clone()).unwrap_or_default();
        assert!(toast.contains("gone.pdf") && toast.contains("removed from Recent"), "plain prune toast, got {toast:?}");
    }
}
