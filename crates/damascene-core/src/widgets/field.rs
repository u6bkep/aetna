//! Field — shadcn-shaped labelled control for settings rows and
//! inspector panes.
//!
//! Oracle: shadcn/ui's `Field` family (`Field`, `FieldContent`,
//! `FieldDescription`, `FieldGroup`, `FieldSeparator`, `FieldSet`,
//! `FieldLegend` — see
//! `references/workbench-validation/shadcn-refs/src/components/ui/field.tsx`),
//! per `docs/NAMING_ORACLE.md`. This is the preferred path for the two
//! shapes the validation rounds kept missing: the **settings row with a
//! description** and the **inspector row**.
//!
//! - [`field`] — label above control (shadcn's default
//!   `orientation="vertical"`).
//! - [`field_with`] + [`FieldOpts`] — adds `description(...)` and
//!   `horizontal()`, the settings-row shape: label + description
//!   stacked on the left, control trailing right, vertically centered.
//! - [`field_group`] / [`field_separator`] / [`field_set`] — the
//!   surrounding rhythm: stacks, rules, and legends.
//!
//! The canonical settings row:
//!
//! ```ignore
//! use damascene_core::prelude::*;
//!
//! struct Prefs { smooth_scrolling: bool }
//!
//! impl App for Prefs {
//!     fn build(&self, _cx: &BuildCx) -> El {
//!         field_group([field_with(
//!             "Smooth scrolling",
//!             checkbox("smooth_scrolling", self.smooth_scrolling),
//!             FieldOpts::default()
//!                 .description("Animate scroll position instead of jumping by line.")
//!                 .horizontal(),
//!         )])
//!     }
//! }
//! ```
//!
//! For a full vertical form with validation messages
//! ([`crate::widgets::form::form_message`]) or an explicit control
//! wrapper ([`crate::widgets::form::form_control`]), the
//! [`crate::widgets::form`] anatomy still serves; `field_row` remains
//! for the compact single-line label/control pair with no description.
//!
//! # Dogfood note
//!
//! Pure composition over the public widget-kit surface — `row`,
//! `column`, [`crate::widgets::text::text`] role modifiers, and the
//! stock [`crate::widgets::separator::separator`]. No internal
//! machinery; an app crate can fork this file and produce an
//! equivalent helper.

// Lock in full per-item documentation for this module (issue #73).
#![warn(missing_docs)]

use std::panic::Location;

use super::separator::separator;
use super::text::text;
use crate::metrics::MetricsRole;
use crate::tokens;
use crate::tree::*;

/// Optional configuration for [`field_with`]. The defaults reproduce
/// [`field`] verbatim, so callers only set what they need — the same
/// shape as [`crate::widgets::text_input::TextInputOpts`].
#[derive(Clone, Copy, Debug, Default)]
pub struct FieldOpts<'a> {
    /// One dim helper line (shadcn's `FieldDescription` —
    /// `text-muted-foreground`). Vertical fields place it under the
    /// control; horizontal fields stack it under the label in the
    /// leading column, matching field.tsx's `FieldContent` composition.
    pub description: Option<&'a str>,
    /// Switch to shadcn's `orientation="horizontal"`: leading column of
    /// label + description, control trailing right, vertically
    /// centered. The settings-row / inspector-row shape.
    pub horizontal: bool,
}

impl<'a> FieldOpts<'a> {
    /// Set the dim helper line (see [`FieldOpts::description`]).
    pub fn description(mut self, d: &'a str) -> Self {
        self.description = Some(d);
        self
    }

    /// Use the horizontal settings-row orientation (see
    /// [`FieldOpts::horizontal`]).
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }
}

/// A labelled control (shadcn's `Field`, default vertical orientation):
/// label ([`TextRole::Label`]) above the control, both full width.
///
/// For a description line or the horizontal settings-row shape, use
/// [`field_with`]. For a compact label/control pair with no
/// description, [`crate::widgets::form::field_row`] also serves.
#[track_caller]
pub fn field(label: impl Into<String>, control: impl Into<El>) -> El {
    field_with(label, control, FieldOpts::default())
}

/// Like [`field`], but takes a [`FieldOpts`] for a description line
/// and/or the horizontal orientation. Pass `FieldOpts::default()` for
/// an output identical to [`field`].
#[track_caller]
pub fn field_with(label: impl Into<String>, control: impl Into<El>, opts: FieldOpts<'_>) -> El {
    let label_el = text(label)
        .at_loc(Location::caller())
        .label()
        .ellipsis()
        .width(Size::Fill(1.0));

    if opts.horizontal {
        // shadcn `orientation="horizontal"`: `FieldContent` (label +
        // description, `flex-1`) leads; the control trails right.
        // field.tsx's `gap-1.5` (6px) content gap translates to
        // SPACE_1, the nearest step on the damascene scale — it keeps
        // label + description reading as one unit.
        let mut content = vec![label_el];
        if let Some(d) = opts.description {
            content.push(field_description(d));
        }
        let content = column(content)
            .default_gap(tokens::SPACE_1)
            .width(Size::Fill(1.0))
            .height(Size::Hug);

        row([content, opts_control(control)])
            .at_loc(Location::caller())
            .metrics_role(MetricsRole::FormItem)
            .align(Align::Center)
            .default_gap(tokens::SPACE_3)
            .width(Size::Fill(1.0))
            .height(Size::Hug)
    } else {
        // shadcn vertical `Field`: `flex-col gap-3`, children full
        // width; the description sits under the control.
        let mut children = vec![label_el, opts_control(control)];
        if let Some(d) = opts.description {
            children.push(field_description(d));
        }
        column(children)
            .at_loc(Location::caller())
            .metrics_role(MetricsRole::FormItem)
            .default_gap(tokens::SPACE_3)
            .width(Size::Fill(1.0))
            .height(Size::Hug)
    }
}

/// The control slot passes through untouched so apps reading routed
/// events via `target_key` see the control's own key.
fn opts_control(control: impl Into<El>) -> El {
    control.into()
}

/// The dim helper line (shadcn's `FieldDescription` — `text-sm
/// text-muted-foreground`, rendered here in the caption role so it
/// steps down from the label the way the settings references do).
#[track_caller]
fn field_description(description: impl Into<String>) -> El {
    text(description)
        .at_loc(Location::caller())
        .caption()
        .muted()
        .wrap_text()
        .fill_width()
}

/// Vertical stack of [`field`]s (shadcn's `FieldGroup`, `flex-col
/// gap-7` → [`tokens::SPACE_7`] on the damascene scale).
///
/// Carries `RING_WIDTH` of horizontal default padding so trailing
/// controls flush at the group's edges keep room for their focus-ring
/// band inside a clipping ancestor (`scroll(...)`) — the same reserve
/// [`crate::widgets::form::form`] and
/// [`crate::widgets::item::item_group`] make.
#[track_caller]
pub fn field_group<I, E>(children: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    column(children)
        .at_loc(Location::caller())
        .metrics_role(MetricsRole::Form)
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .default_gap(tokens::SPACE_7)
        .default_padding(Sides::xy(tokens::RING_WIDTH, 0.0))
}

/// Hairline rule between fields (shadcn's `FieldSeparator`) — the
/// stock [`crate::widgets::separator::separator`] under the anatomy's
/// name.
#[track_caller]
pub fn field_separator() -> El {
    separator().at_loc(Location::caller())
}

/// A legended group of fields (shadcn's `FieldSet` + `FieldLegend`):
/// a small medium-weight legend (field.tsx's legend tone — `text-base
/// font-medium`, `mb-3` → [`tokens::SPACE_3`]) above a stack of fields
/// at the fieldset rhythm (`gap-6` → [`tokens::SPACE_6`]).
#[track_caller]
pub fn field_set<I, E>(legend: impl Into<String>, children: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    let legend_el = text(legend)
        .at_loc(Location::caller())
        .title()
        .font_weight(FontWeight::Medium)
        .ellipsis()
        .width(Size::Fill(1.0));

    let group = column(children)
        .at_loc(Location::caller())
        .default_gap(tokens::SPACE_6)
        .width(Size::Fill(1.0))
        .height(Size::Hug);

    column([legend_el, group])
        .at_loc(Location::caller())
        .metrics_role(MetricsRole::Form)
        .default_gap(tokens::SPACE_3)
        .width(Size::Fill(1.0))
        .height(Size::Hug)
        .default_padding(Sides::xy(tokens::RING_WIDTH, 0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::checkbox::checkbox;
    use crate::widgets::switch::switch;

    #[test]
    fn field_stacks_label_above_control() {
        // shadcn's default vertical orientation: label first, control
        // second, in a full-width column at the Field gap.
        let f = field("Email", switch("notify", true));

        assert_eq!(f.axis, Axis::Column);
        assert_eq!(f.metrics_role, Some(MetricsRole::FormItem));
        assert_eq!(f.width, Size::Fill(1.0));
        assert_eq!(f.gap, tokens::SPACE_3);
        assert_eq!(f.children.len(), 2);

        let label = &f.children[0];
        assert_eq!(label.text.as_deref(), Some("Email"));
        assert_eq!(label.text_role, TextRole::Label);

        let control = &f.children[1];
        assert_eq!(control.key.as_deref(), Some("notify"));
        assert!(f.key.is_none(), "field carries no key of its own");
    }

    #[test]
    fn vertical_description_sits_under_the_control() {
        let f = field_with(
            "Theme",
            switch("theme", false),
            FieldOpts::default().description("Applies to all windows."),
        );

        assert_eq!(f.children.len(), 3);
        let description = &f.children[2];
        assert_eq!(description.text.as_deref(), Some("Applies to all windows."));
        assert_eq!(description.text_role, TextRole::Caption);
        assert_eq!(description.text_color, Some(tokens::MUTED_FOREGROUND));
    }

    #[test]
    fn horizontal_stacks_description_under_label_in_leading_column() {
        // The settings-row shape: FieldContent (label + description)
        // leads; the control trails; the row is vertically centered.
        let f = field_with(
            "Smooth scrolling",
            checkbox("smooth", true),
            FieldOpts::default()
                .description("Animate scroll position instead of jumping by line.")
                .horizontal(),
        );

        assert_eq!(f.axis, Axis::Row);
        assert_eq!(f.align, Align::Center);
        assert_eq!(f.width, Size::Fill(1.0));
        assert_eq!(f.children.len(), 2);

        let content = &f.children[0];
        assert_eq!(content.axis, Axis::Column);
        assert_eq!(content.width, Size::Fill(1.0));
        assert_eq!(content.gap, tokens::SPACE_1);
        assert_eq!(content.children.len(), 2);
        assert_eq!(content.children[0].text.as_deref(), Some("Smooth scrolling"));
        assert_eq!(content.children[0].text_role, TextRole::Label);
        assert_eq!(content.children[1].text_role, TextRole::Caption);
        assert_eq!(
            content.children[1].text_color,
            Some(tokens::MUTED_FOREGROUND)
        );

        let control = &f.children[1];
        assert_eq!(control.key.as_deref(), Some("smooth"));
    }

    #[test]
    fn horizontal_control_trails_right_and_centers_vertically() {
        // Layout-level check of the settings-row promise: the leading
        // column absorbs the slack, so the control's right edge lands
        // on the row's right edge, vertically centered against the
        // two-line label block.
        let mut root = field_with(
            "Reduce motion",
            switch("reduce_motion", false),
            FieldOpts::default()
                .description("Disable panel transitions, easing, and decorative animation.")
                .horizontal(),
        );
        let mut state = crate::UiState::new();
        crate::layout::layout(&mut root, &mut state, Rect::new(0.0, 0.0, 600.0, 200.0));

        let row_rect = root.computed_rect;
        let control = &root.children[1];
        let control_right = control.computed_rect.x + control.computed_rect.w;
        assert!(
            (control_right - (row_rect.x + row_rect.w)).abs() < 0.5,
            "control should land flush right, right edge {} vs row {}",
            control_right,
            row_rect.x + row_rect.w
        );

        let row_mid = row_rect.y + row_rect.h / 2.0;
        let control_mid = control.computed_rect.y + control.computed_rect.h / 2.0;
        assert!(
            (control_mid - row_mid).abs() < 0.5,
            "control should center vertically, mid {} vs row mid {}",
            control_mid,
            row_mid
        );

        let content = &root.children[0];
        assert!(
            content.computed_rect.w > control.computed_rect.w,
            "leading column should absorb the row's slack"
        );
    }

    #[test]
    fn field_group_uses_the_shadcn_group_gap() {
        // FieldGroup is `gap-7` — SPACE_7 on the damascene scale.
        let g = field_group([
            field("One", switch("a", false)),
            field_separator(),
            field("Two", switch("b", true)),
        ]);

        assert_eq!(g.axis, Axis::Column);
        assert_eq!(g.metrics_role, Some(MetricsRole::Form));
        assert_eq!(g.width, Size::Fill(1.0));
        assert_eq!(g.gap, tokens::SPACE_7);
        assert_eq!(g.padding, Sides::xy(tokens::RING_WIDTH, 0.0));
        assert_eq!(g.children.len(), 3);
        assert_eq!(g.children[1].kind, Kind::Divider);
    }

    #[test]
    fn field_set_places_a_medium_legend_over_the_group() {
        // FieldLegend's default tone is `text-base font-medium` with
        // `mb-3`; the fieldset stacks its fields at `gap-6`.
        let s = field_set("Motion & chrome", [field("One", switch("a", false))]);

        assert_eq!(s.axis, Axis::Column);
        assert_eq!(s.gap, tokens::SPACE_3);
        assert_eq!(s.children.len(), 2);

        let legend = &s.children[0];
        assert_eq!(legend.text.as_deref(), Some("Motion & chrome"));
        assert_eq!(legend.text_role, TextRole::Title);
        assert_eq!(legend.font_size, tokens::TEXT_BASE.size);
        assert_eq!(legend.font_weight, FontWeight::Medium);

        let group = &s.children[1];
        assert_eq!(group.axis, Axis::Column);
        assert_eq!(group.gap, tokens::SPACE_6);
        assert_eq!(group.children.len(), 1);
    }
}
