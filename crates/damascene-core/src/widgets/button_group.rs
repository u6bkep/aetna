//! ButtonGroup — adjacent controls joined into one segmented control.
//!
//! Oracle: shadcn/ui `ButtonGroup`
//! (`references/workbench-validation/shadcn-refs/src/components/ui/button-group.tsx`,
//! the pinned late-2025 registry snapshot named in
//! `docs/NAMING_ORACLE.md`). shadcn's horizontal recipe is
//!
//! ```text
//! [&>*:not(:first-child)]:rounded-l-none
//! [&>*:not(:first-child)]:border-l-0
//! [&>*:not(:last-child)]:rounded-r-none
//! ```
//!
//! i.e. *collapse the interior corners, and never draw the interior
//! edge twice.* This module is that recipe: `button_group` is a
//! zero-gap row whose children keep their own variants, events, and
//! keys while the group rewrites the two things that make N controls
//! read as one — corner silhouette and edge seams.
//!
//! The canonical use is the split button:
//!
//! ```ignore
//! use damascene_core::prelude::*;
//!
//! button_group([
//!     button("Slice").primary().key("slice"),
//!     icon_button("chevron-down").primary().key("slice:menu"),
//! ])
//! ```
//!
//! # Seams: why not `border_l()` everywhere
//!
//! On the web a CSS border sits *inside* the box, so two flush
//! neighbours paint 2px of border and shadcn has to delete one with
//! `border-l-0`. Damascene's `.stroke(...)`
//! straddles the boundary (see `paint::draw_ops::combined_overflow`),
//! so two flush stroked neighbours land their 1px strokes on the *same*
//! pixel column — joined at `gap = 0` they already read as a single
//! hairline, and deleting one would leave the seam blank.
//!
//! What does need help is the case a stroke can't express: a control
//! whose stroke color equals its fill (`.primary()`, `.destructive()`
//! — the solid-tint recipe sets both), or a
//! `.ghost()` control with no stroke at all. There the boundary is
//! invisible, so the trailing item gets a per-side
//! `.border_l()` — the HTML/CSS-oracle
//! primitive, exact here because corner collapse has already zeroed
//! that item's left corner radii. Per-side borders join
//! `content_inset`, so the seam costs the item 1px of content box
//! exactly like CSS `border-box`.
//!
//! Each boundary therefore carries exactly one hairline, whatever mix
//! of variants the author passes.
//!
//! # Dogfood note
//!
//! Composes only the public widget-kit surface — `Kind::Custom`, the
//! per-side border chainables, `.hit_overflow()` /
//! `.focus_ring_inside()`, and the `RadiusOrigin::LibraryShape` corner
//! stamp that `tabs_list` uses. An app crate can fork this file and
//! produce an equivalent widget against the same public API.

// Lock in full per-item documentation for this module (issue #73).
#![warn(missing_docs)]

use std::panic::Location;

use crate::tokens;
use crate::tree::*;

/// Join controls into one segmented control — shadcn's `ButtonGroup`.
///
/// A `Size::Hug` row (shadcn's `w-fit`) with no gap. The children keep
/// their own fills, variants, keys, and event contracts; the group only
/// rewrites what makes them read as one control:
///
/// - **Corner collapse.** The first child keeps its leading corners,
///   the last its trailing corners, everything between is squared. A
///   lone child is left alone. The stamp is
///   [`RadiusOrigin::LibraryShape`], so the silhouette survives the
///   metrics pass (`apply_control` only restamps
///   [`RadiusOrigin::ThemeDefault`]) while
///   [`Theme::with_radius_scale`](crate::Theme::with_radius_scale)
///   still scales it — same contract as `tabs_list`'s edge triggers. A
///   child with an explicit `.radius(...)`
///   ([`RadiusOrigin::Fixed`]) keeps it; the author asked for that
///   shape.
/// - **Seams.** Exactly one hairline per boundary — see the module docs
///   for the stroke-straddles-the-boundary reasoning.
/// - **Flush-neighbour hygiene.** Joined children drop their
///   `hit_overflow` (an expanded target would steal the neighbour's
///   edge band — `FindingKind::HitOverflowCollision`) and switch to an
///   inside focus ring (an outward ring is painted over by the next
///   sibling — `FindingKind::FocusRingObscured`). Same idiom as
///   `numeric_input`'s stacked chevron column.
///
/// Unlike [`crate::widgets::toggle::toggle_group`] the group draws no
/// frame of its own: it has no fill and no stroke, so a group of
/// `.ghost()` buttons stays chromeless, exactly as shadcn's
/// `<ButtonGroup>` (a bare `role="group"` flex box) does.
///
/// ```ignore
/// use damascene_core::prelude::*;
///
/// // Split button: primary action + its menu affordance.
/// button_group([
///     button("Save").primary().key("save"),
///     icon_button("chevron-down").primary().key("save:menu"),
/// ])
/// ```
#[track_caller]
pub fn button_group(children: impl IntoIterator<Item = El>) -> El {
    let caller = Location::caller();
    let mut items: Vec<El> = children.into_iter().collect();
    join_row(&mut items);
    El::new(Kind::Custom("button_group"))
        .at_loc(caller)
        .axis(Axis::Row)
        // shadcn's `items-stretch`: a group of mixed-height children
        // still reads as one control.
        .align(Align::Stretch)
        .gap(tokens::SPACE_0)
        .children(items)
        .width(Size::Hug)
        .height(Size::Hug)
}

/// Turn a row of independently-styled controls into one joined
/// segmented control, in place.
///
/// Shared by [`button_group`] and
/// [`crate::widgets::toggle::toggle_group`] so both anatomies collapse
/// corners and resolve seams by the same rule. See [`button_group`] for
/// the contract and the module docs for the seam reasoning.
pub(crate) fn join_row(items: &mut [El]) {
    let last = items.len().saturating_sub(1);
    for (i, item) in items.iter_mut().enumerate() {
        collapse_corners(item, i == 0, i == last);
        // Flush neighbours: an expanded hit target reaches into the
        // next item's rect (`HitOverflowCollision`) and an outward
        // focus ring is painted over by it (`FocusRingObscured`).
        // Inside the joined trough there is no whitespace for either to
        // live in, so both are traded for the dense-stack idiom.
        *item = std::mem::take(item)
            .hit_overflow(Sides::zero())
            .focus_ring_inside();
    }
    for i in 1..items.len() {
        if draws_edge(&items[i - 1]) || draws_edge(&items[i]) {
            // One of the two strokes already lands a hairline on the
            // shared boundary (strokes straddle it, so neighbouring
            // strokes coincide instead of doubling). Adding a border
            // here is what *would* double it.
            continue;
        }
        items[i] = std::mem::take(&mut items[i])
            .border_l()
            .border_color(tokens::BORDER);
    }
}

/// Does this element paint a visible edge on its own boundary? True for
/// the bordered variants (`.secondary()`, `.outline()`, `.current()`,
/// `.selected()`); false for `.ghost()` (no stroke) and the solid
/// tints (`.primary()`, `.destructive()` — `tint` sets stroke = fill,
/// so the stroke is invisible against the fill it sits on).
fn draws_edge(el: &El) -> bool {
    match el.stroke {
        None => false,
        Some(_) if el.stroke_width <= 0.0 => false,
        Some(stroke) => Some(stroke) != el.fill,
    }
}

/// Keep the leading corners on the first item, the trailing corners on
/// the last, and square everything else — shadcn's
/// `rounded-l-none` / `rounded-r-none` pair. Magnitudes come from the
/// item's own radius, so a group of buttons keeps the button radius and
/// a group of tab triggers keeps the trigger radius.
fn collapse_corners(item: &mut El, first: bool, last: bool) {
    if first && last {
        // A lone child is not joined to anything.
        return;
    }
    if item.radius_origin == RadiusOrigin::Fixed {
        // The author set `.radius(...)` explicitly — their shape wins,
        // the same carve-out `tabs_list` makes for customized triggers.
        return;
    }
    let r = item.radius;
    item.radius = Corners {
        tl: if first { r.tl } else { 0.0 },
        bl: if first { r.bl } else { 0.0 },
        tr: if last { r.tr } else { 0.0 },
        br: if last { r.br } else { 0.0 },
    };
    // `LibraryShape`, not `Fixed`: the segment silhouette is ours (the
    // metrics pass must not restamp it) but its magnitude is still the
    // theme's, so `Theme::with_radius_scale(0.0)` squares a joined
    // group along with everything else.
    item.radius_origin = RadiusOrigin::LibraryShape;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::layout;
    use crate::state::UiState;
    use crate::widgets::button::{button, icon_button};

    fn radii(el: &El) -> (f32, f32, f32, f32) {
        (el.radius.tl, el.radius.tr, el.radius.br, el.radius.bl)
    }

    #[test]
    fn group_collapses_interior_corners() {
        let g = button_group([button("One"), button("Two"), button("Three")]);
        let r = tokens::RADIUS_MD;
        assert_eq!(
            radii(&g.children[0]),
            (r, 0.0, 0.0, r),
            "first item keeps only its leading corners",
        );
        assert_eq!(
            radii(&g.children[1]),
            (0.0, 0.0, 0.0, 0.0),
            "middle items are squared on both ends",
        );
        assert_eq!(
            radii(&g.children[2]),
            (0.0, r, r, 0.0),
            "last item keeps only its trailing corners",
        );
    }

    #[test]
    fn collapsed_corners_are_library_shape() {
        // `LibraryShape` is the load-bearing origin: `apply_control` in
        // the metrics pass restamps `ThemeDefault` radii (which would
        // re-round the interior corners), while `with_radius_scale`
        // still scales `LibraryShape`.
        let g = button_group([button("One"), button("Two")]);
        for child in &g.children {
            assert_eq!(child.radius_origin, RadiusOrigin::LibraryShape);
        }
    }

    #[test]
    fn collapsed_silhouette_survives_the_metrics_pass() {
        let mut g = button_group([button("One"), button("Two")]);
        crate::Theme::default().apply_metrics(&mut g);
        assert_eq!(g.children[0].radius.tr, 0.0, "interior corner stays square");
        assert_eq!(g.children[1].radius.tl, 0.0, "interior corner stays square");
        assert!(
            g.children[0].radius.tl > 0.0,
            "outer corner keeps the silhouette",
        );
    }

    #[test]
    fn collapsed_silhouette_still_scales_with_the_theme() {
        let mut g = button_group([button("One"), button("Two")]);
        crate::Theme::default()
            .with_radius_scale(0.0)
            .apply_metrics(&mut g);
        assert_eq!(
            radii(&g.children[0]),
            (0.0, 0.0, 0.0, 0.0),
            "radius_scale(0.0) squares the joined silhouette too",
        );
    }

    #[test]
    fn lone_child_keeps_its_whole_silhouette() {
        let g = button_group([button("Only")]);
        let r = tokens::RADIUS_MD;
        assert_eq!(radii(&g.children[0]), (r, r, r, r));
    }

    #[test]
    fn author_set_radius_wins() {
        let g = button_group([button("One").radius(2.0), button("Two")]);
        assert_eq!(
            radii(&g.children[0]),
            (2.0, 2.0, 2.0, 2.0),
            "an explicit `.radius(...)` is the author's shape",
        );
        assert_eq!(g.children[0].radius_origin, RadiusOrigin::Fixed);
    }

    #[test]
    fn stroked_neighbours_do_not_get_a_doubled_seam() {
        // `button()` is `.secondary()`: stroke BORDER over fill
        // SECONDARY. Its stroke straddles the shared boundary, so the
        // neighbour's stroke coincides with it — one hairline. A
        // `border_l()` on top would be the second line.
        let g = button_group([button("One"), button("Two"), button("Three")]);
        for (i, child) in g.children.iter().enumerate() {
            assert!(
                child.border.is_none(),
                "child {i} must not add a border on top of coincident strokes",
            );
        }
    }

    #[test]
    fn invisible_seam_gets_exactly_one_left_border() {
        // `.primary()` sets stroke == fill, so nothing marks the
        // boundary; the trailing item owns the divider.
        let g = button_group([
            button("Save").primary(),
            icon_button("chevron-down").primary(),
        ]);
        assert!(
            g.children[0].border.is_none(),
            "the leading item owns no seam",
        );
        let border = g.children[1]
            .border
            .as_deref()
            .expect("the trailing item carries the seam");
        assert_eq!(border.widths.left, 1.0, "one 1px divider");
        assert_eq!(
            (border.widths.right, border.widths.top, border.widths.bottom),
            (0.0, 0.0, 0.0),
            "only the shared edge is bordered",
        );
        assert_eq!(border.color, Some(tokens::BORDER));
    }

    #[test]
    fn ghost_neighbours_get_a_divider() {
        let g = button_group([button("One").ghost(), button("Two").ghost()]);
        assert_eq!(
            g.children[1]
                .border
                .as_deref()
                .map(|b| b.widths.left)
                .unwrap_or(0.0),
            1.0,
            "two chromeless items still need a visible division",
        );
    }

    #[test]
    fn split_button_keeps_its_children_interactive() {
        let g = button_group([
            button("Save").primary().key("save"),
            icon_button("chevron-down").primary().key("save:menu"),
        ]);
        assert_eq!(g.children.len(), 2, "no divider nodes are injected");
        assert_eq!(g.children[0].key.as_deref(), Some("save"));
        assert_eq!(g.children[1].key.as_deref(), Some("save:menu"));
        assert!(g.children.iter().all(|c| c.focusable));
        assert_eq!(
            g.children[1].width,
            Size::Fixed(tokens::CONTROL_HEIGHT),
            "the chevron half stays a square icon button",
        );
        assert!(g.key.is_none(), "the group is chrome, not a target");
        assert!(g.fill.is_none() && g.stroke.is_none(), "no frame of its own");
    }

    #[test]
    fn joined_children_are_flush() {
        let mut g = button_group([button("One"), button("Two"), button("Three")]);
        let mut state = UiState::new();
        layout(&mut g, &mut state, Rect::new(0.0, 0.0, 400.0, 60.0));
        for pair in g.children.windows(2) {
            let (a, b) = (pair[0].computed_rect, pair[1].computed_rect);
            assert!(
                (b.x - (a.x + a.w)).abs() < 0.01,
                "items must touch: {a:?} then {b:?}",
            );
        }
    }

    #[test]
    fn joined_children_drop_flush_neighbour_hazards() {
        let g = button_group([button("One"), button("Two")]);
        for child in &g.children {
            assert_eq!(
                child.hit_overflow,
                Sides::zero(),
                "an expanded target would reach into the neighbour",
            );
            assert_eq!(child.focus_ring_placement, FocusRingPlacement::Inside);
        }
    }

    #[test]
    fn joined_group_is_lint_clean() {
        use crate::bundle::lint::FindingKind;
        let mut root = crate::tree::column([button_group([
            button("One"),
            button("Two"),
            button("Three"),
        ])])
        .padding(Sides::all(tokens::SPACE_4));
        let mut state = UiState::new();
        layout(&mut root, &mut state, Rect::new(0.0, 0.0, 400.0, 120.0));
        let report = crate::bundle::lint::lint(&root, &state);
        let relevant: Vec<_> = report
            .findings
            .iter()
            .filter(|f| {
                matches!(
                    f.kind,
                    FindingKind::HitOverflowCollision
                        | FindingKind::FocusRingObscured
                        | FindingKind::CornerStackup
                )
            })
            .collect();
        assert!(
            relevant.is_empty(),
            "joined groups must not trip flush-neighbour lints:\n{}",
            report.text(),
        );
    }
}
