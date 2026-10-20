//! Shared dialog chrome — T-003 "Modern minimal" (the dialogs-* mockups are the spec).
//!
//! Every modal dialog is built from these pieces: an 18 pt semibold title, an optional tab
//! row, radio rows, an accent-soft summary banner, label/value rows, and a footer with a
//! divider and Cancel + primary buttons. In the `ur` (RTL) locale the whole dialog mirrors:
//! the tab row starts at the right, label/value columns swap sides, and the primary action
//! lands at the left end of the footer.

use egui::{Align, Align2, CornerRadius, Layout, Sense, Stroke, vec2};

use crate::theme::{self, Tokens};

/// An 18 pt semibold dialog title ("Document Properties", "Split document", …).
pub fn title(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).font(theme::semibold(18.0)));
    ui.add_space(8.0);
}

/// A tab row under the title (Document Properties' Description / Initial View / …).
/// 12.5 pt medium muted tabs; the active one is accent semibold with a 2 px accent underline,
/// and a divider runs along the row's bottom edge. Returns the tab that was clicked, if any.
pub fn tab_bar<T: Copy + PartialEq>(ui: &mut egui::Ui, tabs: &[(T, &str)], active: T) -> Option<T> {
    let t = Tokens::get(ui.ctx());
    let mut picked = None;
    let row = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (id, label) in tabs {
            let is_active = *id == active;
            let font = if is_active { theme::semibold(12.5) } else { theme::medium(12.5) };
            let w = ui.fonts_mut(|f| f.layout_no_wrap((*label).to_owned(), font.clone(), t.text).size().x);
            let (rect, resp) = ui.allocate_exact_size(vec2(w + 24.0, 34.0), Sense::click());
            resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Button, ui.is_enabled(), is_active, *label));
            let press = theme::Press::track(ui, &resp);
            let body = rect.translate(press.offset());
            if !is_active {
                press.wash(ui, rect, 6, false);
            }
            ui.painter().text(body.center(), Align2::CENTER_CENTER, *label, font, if is_active { t.accent } else { t.text_muted });
            if is_active {
                let bar = egui::Rect::from_min_max(
                    egui::pos2(rect.left() + 10.0, rect.bottom() - 1.0),
                    egui::pos2(rect.right() - 10.0, rect.bottom() + 1.0),
                );
                ui.painter().rect_filled(bar, CornerRadius::same(1), t.accent);
            }
            if theme::hand(resp).clicked() {
                picked = Some(*id);
            }
        }
    });
    // The divider under the row; the active tab's underline overlaps it.
    let r = row.response.rect;
    ui.painter().hline(r.x_range(), r.bottom(), Stroke::new(1.0, t.divider));
    ui.add_space(8.0);
    picked
}

/// The mockup's summary banner: accent-soft fill, 9 pt corners ("Creates 6 files from 28
/// pages."). Accepts rich text so part of the message can be emphasized.
pub fn summary(ui: &mut egui::Ui, text: egui::RichText) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE.fill(t.accent_soft).corner_radius(CornerRadius::same(9)).inner_margin(egui::Margin::symmetric(14, 10)).show(ui, |ui| {
        ui.label(text.color(t.text_muted).font(theme::regular(12.5)));
    });
}

/// One mockup footer button: 8/18 padding, 9 pt corners, 12.5 semibold. The primary button
/// fills with the accent and white text; the secondary one is an outlined field button.
pub fn button(ui: &mut egui::Ui, label: &str, primary: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::semibold(12.5);
    let w = ui.fonts_mut(|f| f.layout_no_wrap(label.to_owned(), font.clone(), t.text).size().x);
    let (rect, resp) = ui.allocate_exact_size(vec2(w + 36.0, 33.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label));
    let press = theme::Press::track(ui, &resp);
    let (fill, stroke, ink) = if primary {
        (press.fill(t.accent, t.accent.gamma_multiply(0.92), t.accent.gamma_multiply(0.8)), Stroke::NONE, egui::Color32::WHITE)
    } else {
        (press.fill(t.field, t.hover, t.pressed), Stroke::new(1.0, t.border), t.text)
    };
    ui.painter().rect(rect, CornerRadius::same(9), fill, stroke, egui::StrokeKind::Inside);
    ui.painter().text(rect.translate(press.offset()).center(), Align2::CENTER_CENTER, label, font, ink);
    theme::hand(resp)
}

/// The divider a dialog footer sits under (mockup: 1 px divider, 22 px above, 16 px below).
pub fn divider(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(22.0);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(r.x_range(), r.center().y, Stroke::new(1.0, t.divider));
    ui.add_space(16.0);
}

/// The mockup's dialog footer: a divider, spacing, then Cancel + the primary action. In RTL
/// the order mirrors so the primary action lands at the left end of the footer (the
/// dialogs-* RTL strip). Returns `(primary_clicked, cancel_clicked)`.
pub fn footer(ui: &mut egui::Ui, cancel_label: &str, primary_label: &str, primary_enabled: bool) -> (bool, bool) {
    let rtl = crate::i18n::current().rtl();
    divider(ui);
    let (mut primary, mut cancel) = (false, false);
    // LTR: first added lands rightmost (primary at the right end). RTL: first added lands
    // leftmost, so adding the primary first puts it at the left end, as mocked.
    ui.with_layout(if rtl { Layout::left_to_right(Align::Center) } else { Layout::right_to_left(Align::Center) }, |ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        if ui.add_enabled_ui(primary_enabled, |ui| button(ui, primary_label, true)).inner.clicked() {
            primary = true;
        }
        if button(ui, cancel_label, false).clicked() {
            cancel = true;
        }
    });
    (primary, cancel)
}

/// A footer with a single primary-styled button ("Close").
pub fn footer_single(ui: &mut egui::Ui, label: &str) -> bool {
    let rtl = crate::i18n::current().rtl();
    divider(ui);
    ui.with_layout(if rtl { Layout::left_to_right(Align::Center) } else { Layout::right_to_left(Align::Center) }, |ui| {
        button(ui, label, true).clicked()
    })
    .inner
}

/// The layout direction a dialog's trailing button row uses: primary at the inline end.
/// LTR puts the primary rightmost; RTL puts it leftmost (the mockup's RTL strip).
pub fn footer_layout() -> Layout {
    if crate::i18n::current().rtl() { Layout::left_to_right(Align::Center) } else { Layout::right_to_left(Align::Center) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Paint the footer and return its text shapes: (text, x-center).
    fn footer_texts(rtl: bool, ctx: &egui::Context) -> Vec<(String, f32)> {
        let previous = crate::i18n::current();
        crate::i18n::set_current(crate::i18n::Lang::from_code(if rtl { "ur" } else { "en" }).unwrap());
        theme::apply(ctx, theme::ThemeKind::Light);
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            ui.set_width(400.0);
            footer(ui, "Cancel", "Split", true);
        });
        let texts: Vec<(String, f32)> = output
            .shapes
            .iter()
            .filter_map(|s| match &s.shape {
                egui::Shape::Text(t) => Some((t.galley.job.text.clone(), t.pos.x)),
                _ => None,
            })
            .collect();
        output.drop_without_applying_deltas();
        crate::i18n::set_current(previous);
        texts
    }

    #[test]
    fn footer_orders_buttons_per_direction() {
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        for (rtl, primary_left) in [(false, false), (true, true)] {
            let texts = footer_texts(rtl, &ctx);
            let cancel = texts.iter().find(|(t, _)| t == "Cancel").map(|(_, x)| *x).expect("Cancel painted");
            let primary = texts.iter().find(|(t, _)| t == "Split").map(|(_, x)| *x).expect("primary painted");
            assert_eq!(primary < cancel, primary_left, "rtl={rtl}: primary left of cancel = {primary_left}");
        }
    }

    #[test]
    fn footer_paints_divider_and_accent_primary() {
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        theme::apply(&ctx, theme::ThemeKind::Light);
        let t = Tokens::for_kind(theme::ThemeKind::Light);
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            ui.set_width(400.0);
            footer(ui, "Cancel", "Split", true);
        });
        let dividers =
            output.shapes.iter().filter(|s| matches!(&s.shape, egui::Shape::LineSegment { stroke, .. } if stroke.color == t.divider)).count();
        assert!(dividers >= 1, "the footer draws its divider");
        let primary_fill = output.shapes.iter().filter(|s| matches!(&s.shape, egui::Shape::Rect(r) if r.fill == t.accent)).count();
        assert!(primary_fill >= 1, "the primary button fills with the accent");
        output.drop_without_applying_deltas();
    }

    #[test]
    fn title_is_18pt_semibold_and_summary_uses_accent_soft() {
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        theme::apply(&ctx, theme::ThemeKind::Light);
        let t = Tokens::for_kind(theme::ThemeKind::Light);
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            title(ui, "Split document");
            summary(ui, egui::RichText::new("Creates 6 files from 28 pages."));
        });
        let titles: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|s| match &s.shape {
                egui::Shape::Text(x) if x.galley.job.text == "Split document" => Some(x),
                _ => None,
            })
            .collect();
        assert_eq!(titles.len(), 1);
        let size = titles[0].galley.job.sections.first().map(|s| s.format.font_id.size).unwrap_or(0.0);
        assert!((size - 18.0).abs() < 0.01, "title is 18 pt, got {size}");
        let banners = output.shapes.iter().filter(|s| matches!(&s.shape, egui::Shape::Rect(r) if r.fill == t.accent_soft)).count();
        assert!(banners >= 1, "the summary banner fills with accent-soft");
        output.drop_without_applying_deltas();
    }

    #[test]
    fn tab_bar_marks_the_active_tab() {
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        theme::apply(&ctx, theme::ThemeKind::Light);
        let t = Tokens::for_kind(theme::ThemeKind::Light);
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            ui.set_width(500.0);
            let picked = tab_bar(ui, &[(0u8, "Description"), (1u8, "Security")], 0u8);
            assert_eq!(picked, None, "no click yet");
        });
        let active: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|s| match &s.shape {
                egui::Shape::Text(x) if x.galley.job.text == "Description" => Some(x.galley.job.sections[0].format.color),
                _ => None,
            })
            .collect();
        assert_eq!(active, vec![t.accent], "the active tab is accent");
        let underlines =
            output.shapes.iter().filter(|s| matches!(&s.shape, egui::Shape::Rect(r) if r.fill == t.accent && r.rect.height() <= 2.5)).count();
        assert!(underlines >= 1, "the active tab gets its accent underline");
        output.drop_without_applying_deltas();
    }
}
