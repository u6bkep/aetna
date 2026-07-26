//! Table — shadcn-shaped table anatomy.
//!
//! The boring path mirrors the common web component shape:
//! `table([table_header([table_row([...])]), table_body([...])])`.
//! Rows carry the theme-facing table metrics; `table_header` promotes
//! direct `table_row` children from body-row metrics to header metrics.
//!
//! Plain rows are static data, like an HTML `<tr>`. A row the user
//! should be able to select or open is a [`table_row_keyed`] — the
//! `<tr tabindex="0">` shape (issue #144): Tab reaches it, ↑ / ↓ and
//! Home / End step between the body's keyed rows, and Enter / Space
//! or a click emit `Activate` / `Click` routed to its key.

// Lock in full per-item documentation for this module (issue #73).
#![warn(missing_docs)]

use std::panic::Location;

use super::text::text;
use crate::a11y::Role;
use crate::metrics::MetricsRole;
use crate::tokens;
use crate::tree::*;

/// Table root — a full-width clipped column holding [`table_header`]
/// and [`table_body`], like an HTML `<table>`.
#[track_caller]
pub fn table<I, E>(children: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    El::new(Kind::Custom("table"))
        .at_loc(Location::caller())
        .role(Role::Table)
        .children(children)
        .axis(Axis::Column)
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .align(Align::Stretch)
        .clip()
}

/// Header section (like `<thead>`). Direct [`table_row`] children are
/// promoted from body-row metrics to header metrics and squared off.
#[track_caller]
pub fn table_header<I, E>(rows: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    let mut header = El::new(Kind::Custom("table_header"))
        .at_loc(Location::caller())
        .children(rows)
        .axis(Axis::Column)
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .align(Align::Stretch);

    // Promote `table_row(...)` children from body-row metrics to header
    // metrics. Table chrome lives on the cells, so rows stay hug-height
    // and stretch their children vertically.
    for row in &mut header.children {
        if row.metrics_role == Some(MetricsRole::TableRow) {
            row.metrics_role = Some(MetricsRole::TableHeader);
            if row.radius_origin == RadiusOrigin::ThemeDefault {
                row.radius = crate::tree::Corners::ZERO;
            }
        }
    }

    // shadcn's header row carries the same border-b as body rows —
    // it is what visually separates <thead> from <tbody>.
    if let Some(last) = header.children.pop() {
        header = header.child(last.border_b());
    }
    header
}

/// Body section holding the data [`table_row`]s, like `<tbody>`.
/// Every row except the last carries a 1px `.border_b()` — the shadcn
/// table is row-bordered (`tr` gets `border-b`, with `tbody
/// tr:last-child` unbordered), not a full cell grid.
///
/// The body is a vertical arrow-nav group: with focus on a
/// [`table_row_keyed`], ↑ / ↓ step to the adjacent keyed row and Home
/// / End jump to the first / last (the rules between rows aren't
/// focusable, so they're invisible to the stepping). Bodies of plain
/// rows have no members and the flag is inert.
#[track_caller]
pub fn table_body<I, E>(rows: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    let mut children: Vec<El> = rows.into_iter().map(Into::into).collect();
    let n = children.len();
    for row in children.iter_mut().take(n.saturating_sub(1)) {
        *row = std::mem::take(row).border_b();
    }
    El::new(Kind::Custom("table_body"))
        .at_loc(Location::caller())
        .children(children)
        .axis(Axis::Column)
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .align(Align::Stretch)
        .arrow_nav(ArrowNav::Vertical)
}

/// A row of cells (like `<tr>`) carrying the theme's table-row
/// metrics; cells stretch vertically so their padded rows align.
///
/// The `Align::Stretch` that does the stretching also means a cell's
/// *text* sits at the top of a row taller than one line — visible as
/// soon as the row takes a fixed height. Add `.align(Align::Center)` to
/// the row for vertically centered cells.
#[track_caller]
pub fn table_row<I, E>(cells: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    row(cells)
        .at_loc(Location::caller())
        .metrics_role(MetricsRole::TableRow)
        .role(Role::Row)
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .align(Align::Stretch)
        .default_gap(0.0)
        .default_radius(0.0)
}

/// A keyboard-operable data row — [`table_row`] with an identity, the
/// `<tr tabindex="0">` shape (issue #144). `key` makes it a hit-test
/// and focus target: a click or Enter / Space emits `Click` /
/// `Activate` routed to `key` (fold with
/// [`UiEvent::is_click_or_activate`](crate::event::UiEvent::is_click_or_activate)),
/// Tab reaches it, and inside a [`table_body`] ↑ / ↓ / Home / End
/// step between keyed rows. The focus ring sits inside the row so it
/// isn't clipped by the table. Mark the current row
/// `.aria_selected(true)` and give it a selected fill so sighted and
/// screen-reader users see the same state. Row stepping applies while
/// the *row* is focused: with focus on a control inside a cell (an
/// actions-column button) the arrows belong to that control, and
/// Shift+Tab returns to the row.
///
/// ```ignore
/// table_body(items.iter().enumerate().map(|(i, it)| {
///     table_row_keyed(format!("inv-row-{i}"), [table_cell(text(&it.name))])
///         .aria_selected(state.selected == Some(i))
/// }))
/// ```
#[track_caller]
pub fn table_row_keyed<I, E>(key: impl Into<String>, cells: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    table_row(cells)
        .at_loc(Location::caller())
        .key(key)
        .focusable()
        .focus_ring_inside()
}

/// Header cell from a plain label (like `<th>`) — muted medium-weight
/// label text on a transparent ground (shadcn header rows carry no
/// fill; the border-b rule below the row is the header chrome).
///
/// Takes a string, so a header holding anything else — a sort caret, an
/// icon, a checkbox — goes through [`table_head_el`], which applies the
/// same chrome to an arbitrary `El`.
#[track_caller]
pub fn table_head(label: impl Into<String>) -> El {
    table_head_el(text(label))
}

/// Header cell from arbitrary content — applies the header chrome and
/// recursively restyles text descendants to the muted caption treatment.
#[track_caller]
pub fn table_head_el(content: impl Into<El>) -> El {
    let mut el = content
        .into()
        .at_loc(Location::caller())
        .ellipsis()
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .padding(Sides::xy(tokens::SPACE_3, tokens::SPACE_2))
        .radius(0.0);
    // The cell chrome is applied to the content El itself (no wrapper
    // node), so only stamp the header-cell role when the content
    // hasn't already declared one — a sortable-header `button` keeps
    // its `Role::Button`.
    if el.a11y.as_deref().is_none_or(|p| p.role.is_none()) {
        el = el.role(Role::ColumnHeader);
    }
    apply_head_style(&mut el);
    el
}

/// Body cell (like `<td>`) — wraps arbitrary content in the padded,
/// ellipsizing cell chrome. Cells carry no borders of their own;
/// the horizontal rules between rows are the rows' `.border_b()`,
/// applied by [`table_body`].
#[track_caller]
pub fn table_cell(content: impl Into<El>) -> El {
    let el = content
        .into()
        .at_loc(Location::caller())
        .ellipsis()
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .padding(Sides::xy(tokens::SPACE_3, tokens::SPACE_2))
        .radius(0.0);
    // As in [`table_head_el`]: the chrome lands on the content El
    // itself, so keep any role the content already carries (e.g. a
    // `button` in an actions column).
    if el.a11y.as_deref().is_none_or(|p| p.role.is_none()) {
        el.role(Role::Cell)
    } else {
        el
    }
}

fn apply_head_style(el: &mut El) {
    if el.kind == Kind::Text {
        el.text_role = TextRole::Label;
        if el.font_weight == FontWeight::Regular {
            el.font_weight = FontWeight::Medium;
        }
        el.text_color = Some(tokens::MUTED_FOREGROUND);
    }
    for child in &mut el.children {
        apply_head_style(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_row_keyed_is_a_focusable_keyed_row() {
        let r = table_row_keyed("row-1", [table_cell("a")]);
        assert_eq!(r.key.as_deref(), Some("row-1"));
        assert!(r.focusable, "keyed rows are Tab / arrow reachable");
        assert_eq!(r.focus_ring_placement, FocusRingPlacement::Inside);
        assert_eq!(r.metrics_role, Some(MetricsRole::TableRow));
        assert_eq!(
            r.a11y.as_deref().and_then(|p| p.role),
            Some(Role::Row),
            "still a table row to AT"
        );

        // Plain rows stay static data.
        let plain = table_row([table_cell("a")]);
        assert!(plain.key.is_none());
        assert!(!plain.focusable);
    }

    #[test]
    fn table_body_is_a_vertical_arrow_nav_group() {
        let body = table_body([
            table_row_keyed("r0", [table_cell("a")]),
            table_row_keyed("r1", [table_cell("b")]),
        ]);
        assert_eq!(body.arrow_nav, Some(ArrowNav::Vertical));
        // Two rows, no interleaved rule element: the row separator is a
        // `border_b()` on every row but the last, so every child of the
        // body is a keyed, arrow-steppable row.
        assert_eq!(body.children.len(), 2);
        assert_eq!(body.children[0].key.as_deref(), Some("r0"));
        assert_eq!(body.children[1].key.as_deref(), Some("r1"));
        assert_eq!(
            body.children[0].border.as_deref().map(|b| b.widths.bottom),
            Some(1.0),
            "all but the last row carry the separating rule"
        );
        assert!(
            body.children[1]
                .border
                .as_deref()
                .is_none_or(|b| b.widths.bottom == 0.0),
            "the last row is unruled"
        );
    }

    #[test]
    fn table_header_promotes_direct_table_rows() {
        let header = table_header([table_row([table_head("Name")])]);

        // Promoted row carrying the head/body separating border-b.
        assert_eq!(header.children.len(), 1);
        assert_eq!(
            header.children[0].metrics_role,
            Some(MetricsRole::TableHeader)
        );
        assert_eq!(header.children[0].align, Align::Stretch);
        let border = header.children[0].border.as_deref().unwrap();
        assert_eq!(border.widths, Sides::bottom(1.0));
        assert_eq!(border.color, None, "header border uses the token default");
    }

    #[test]
    fn table_head_el_styles_rich_text_children() {
        let head = table_head_el(text_runs([text("Rich "), text("head").bold()]));

        assert_eq!(head.kind, Kind::Inlines);
        assert_eq!(head.children[0].text_role, TextRole::Label);
        assert_eq!(head.children[0].font_weight, FontWeight::Medium);
        assert_eq!(head.children[1].text_role, TextRole::Label);
        assert_eq!(head.children[1].font_weight, FontWeight::Bold);
        assert_eq!(head.children[1].text.as_deref(), Some("head"));
    }

    #[test]
    fn table_rows_are_border_separated_not_grid() {
        // shadcn table anatomy: padded borderless cells, transparent
        // header, and `border-b` on every row but the last.
        let body_cell = table_cell(text("Ada"));
        assert_eq!(
            body_cell.padding,
            Sides::xy(tokens::SPACE_3, tokens::SPACE_2)
        );
        assert_eq!(body_cell.stroke, None);
        assert_eq!(body_cell.radius, Corners::ZERO);

        let head = table_head("Name");
        assert_eq!(head.fill, None);
        assert_eq!(head.stroke, None);

        let body = table_body([
            table_row([table_cell(text("a"))]),
            table_row([table_cell(text("b"))]),
        ]);
        assert_eq!(body.children.len(), 2, "rows only — no rule children");
        let first = body.children[0].border.as_deref().unwrap();
        assert_eq!(first.widths, Sides::bottom(1.0));
        assert!(
            body.children[1].border.is_none(),
            "tbody tr:last-child is unbordered"
        );
    }

    #[test]
    fn table_header_text_emits_glyph_run_after_layout() {
        use crate::Rect;
        use crate::draw_ops::draw_ops;
        use crate::ir::DrawOp;
        use crate::layout::layout;
        use crate::state::UiState;

        let mut tree = table([
            table_header([table_row([table_head("Name"), table_head("Role")])]),
            table_body([table_row([
                table_cell(text("Ada")),
                table_cell(text("dev")),
            ])]),
        ]);
        let mut state = UiState::new();
        layout(&mut tree, &mut state, Rect::new(0.0, 0.0, 320.0, 200.0));

        let ops = draw_ops(&tree, &state);
        assert!(
            ops.iter().any(|op| matches!(
                op,
                DrawOp::GlyphRun { text, .. } if text == "Name"
            )),
            "expected header text to be painted; ops were {ops:?}"
        );
        let rule_quads = ops
            .iter()
            .filter(|op| {
                matches!(op, DrawOp::Quad { id, rect, .. }
                    if id.ends_with(".border-b") && rect.h == 1.0)
            })
            .count();
        assert!(
            rule_quads >= 1,
            "expected the header/body separating border-b quad, got {rule_quads}"
        );
    }

    /// The row_rule → `.border_b()` conversion is pixel-neutral: each
    /// deleted 1px rule child is replaced by its row growing 1px via
    /// the border joining the content inset. Lay out the legacy
    /// interleaved-rule anatomy next to the current one and assert the
    /// total painted height and every rule's y position are unchanged.
    #[test]
    fn border_b_conversion_is_pixel_neutral_with_legacy_rules() {
        use crate::Rect;
        use crate::draw_ops::draw_ops;
        use crate::ir::DrawOp;
        use crate::layout::layout;
        use crate::state::UiState;

        fn legacy_rule() -> El {
            El::new(Kind::Group)
                .fill(tokens::BORDER)
                .width(Size::Fill(1.0))
                .height(Size::Fixed(1.0))
        }
        fn rows() -> [El; 3] {
            ["Ada", "Grace", "Edsger"].map(|n| table_row([table_cell(text(n))]))
        }

        // Legacy anatomy: rules interleaved between rows, plus one
        // under the header row (what table_header/table_body emitted
        // before per-side borders).
        let [r0, r1, r2] = rows();
        let legacy_header = El::new(Kind::Custom("table_header"))
            .children([table_row([table_head("Name")]), legacy_rule()])
            .axis(Axis::Column)
            .width(Size::Fill(1.0))
            .height(Size::Hug)
            .align(Align::Stretch);
        let legacy_body = El::new(Kind::Custom("table_body"))
            .children([r0, legacy_rule(), r1, legacy_rule(), r2])
            .axis(Axis::Column)
            .width(Size::Fill(1.0))
            .height(Size::Hug)
            .align(Align::Stretch);
        let mut legacy = table([legacy_header, legacy_body]);
        // table_header's promotion only runs in the real constructor.
        legacy.children[0].children[0].metrics_role = Some(MetricsRole::TableHeader);

        let mut current = table([
            table_header([table_row([table_head("Name")])]),
            table_body(rows()),
        ]);

        let viewport = Rect::new(0.0, 0.0, 320.0, 400.0);
        let mut state = UiState::new();
        layout(&mut legacy, &mut state, viewport);
        let mut state2 = UiState::new();
        layout(&mut current, &mut state2, viewport);

        // Total painted height: the tables hug their content, so the
        // roots' computed heights must match exactly.
        assert_eq!(
            legacy.computed_rect.h, current.computed_rect.h,
            "conversion changed the table's total height"
        );

        // Rule positions: every legacy 1px rule quad has a border-b
        // quad at the same y in the converted table.
        let legacy_ops = draw_ops(&legacy, &state);
        let current_ops = draw_ops(&current, &state2);
        let legacy_rule_ys: Vec<f32> = legacy_ops
            .iter()
            .filter_map(|op| match op {
                DrawOp::Quad { id, rect, .. } if rect.h == 1.0 && !id.ends_with(".border-b") => {
                    Some(rect.y)
                }
                _ => None,
            })
            .collect();
        let border_ys: Vec<f32> = current_ops
            .iter()
            .filter_map(|op| match op {
                DrawOp::Quad { id, rect, .. } if id.ends_with(".border-b") => Some(rect.y),
                _ => None,
            })
            .collect();
        assert_eq!(legacy_rule_ys.len(), 3, "header rule + two body rules");
        assert_eq!(
            legacy_rule_ys, border_ys,
            "border-b quads must land where the legacy rules painted"
        );
    }
}
