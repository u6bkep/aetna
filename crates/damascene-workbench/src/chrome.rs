//! The chrome widgets core lacks, styled from the workbench theme.
//!
//! Core already ships the shell widgets an application needs most —
//! `menubar`, `toolbar`, `editor_tabs`, `sidebar`, `resize_handle`,
//! `number_scrubber`. This module adds the four it does not, in the
//! order `docs/WORKBENCH_VISION.md` names them: a status bar, a chip, a
//! pane header, and the hairline rule.
//!
//! # Everything here is a stock `El`
//!
//! No new `Kind`, no new metrics role, no core changes. These are
//! recipes: `row`/`column` with workbench fills, per-side borders, and
//! the dense heights from [`crate::tokens`]. An app is free to inline
//! any of them and tune it.
//!
//! # Borders, not interleaved rules
//!
//! The bars here separate themselves with `.border_t()` / `.border_b()`
//! — real CSS-semantics per-side borders, which join padding in the
//! layout content inset. The retired `damascene-carbon` experiment had
//! to interleave 1px filled siblings for the same effect (its
//! `hairline()` had 12 call sites); that workaround is what
//! `docs/VOCABULARY_PARITY.md` §1 was written to delete.
//!
//! [`hairline`] survives anyway, because per-side borders paint on the
//! bordered element. A rule *between* two siblings that neither of them
//! should own — a separator inside a scrolling list, a divider between
//! toolbar groups — is still a node of its own.

#![warn(missing_docs)]

use damascene_core::tokens;
use damascene_core::tree::*;
use damascene_core::widgets::text::text;

use crate::tokens as vs;

/// The workbench status bar — a [`vs::STATUS_BAR_HEIGHT`] strip of
/// terse live readouts across the bottom of the shell.
///
/// # Anatomy
///
/// One `row`, filled [`vs::STATUS_BAR_BG`], with a top border in
/// [`vs::STATUS_BAR_BORDER`]:
///
/// ```text
/// row  fill=statusBar.background  border-t=statusBar.border  h=22
///   ├─ row   (leading items, gap SPACE_3)
///   ├─ spacer
///   └─ row   (trailing items, gap SPACE_3)
/// ```
///
/// The two item rows are always present, even when empty, so the
/// spacer's split point does not depend on the contents — an app can
/// hand either side an empty iterator without the other side moving.
///
/// Deliberately the shortest region in the shell: at 22px it reads as
/// instrumentation rather than as content. Text placed here should be
/// `.caption()` (12px); the bar sets [`vs::STATUS_BAR_FG`] as its text
/// color so plain `text()` children land on the right value.
pub fn status_bar<L, LE, T, TE>(leading: L, trailing: T) -> El
where
    L: IntoIterator<Item = LE>,
    LE: Into<El>,
    T: IntoIterator<Item = TE>,
    TE: Into<El>,
{
    let items = |children: Vec<El>| {
        row(children)
            .gap(tokens::SPACE_3)
            .align(Align::Center)
            .height(Size::Fill(1.0))
    };
    let leading: Vec<El> = leading.into_iter().map(Into::into).collect();
    let trailing: Vec<El> = trailing.into_iter().map(Into::into).collect();

    row([items(leading), spacer(), items(trailing)])
        .fill(vs::STATUS_BAR_BG)
        .text_color(vs::STATUS_BAR_FG)
        .border_t()
        .border_color(vs::STATUS_BAR_BORDER)
        .padding(Sides::x(tokens::SPACE_2))
        .height(Size::Fixed(vs::STATUS_BAR_HEIGHT))
        .width(Size::Fill(1.0))
        .align(Align::Center)
}

/// A dense status chip — VS Code's `badge`, at
/// [`vs::CHIP_HEIGHT`].
///
/// # Anatomy
///
/// ```text
/// row  fill=badge.background  radius=2  h=16  px=SPACE_1
///   └─ text  caption  color=badge.foreground
/// ```
///
/// # Badge-adjacent, deliberately not a badge
///
/// The stock `badge()` is the right shape but the wrong density: its
/// smallest rung is 18px with 6px of horizontal padding and a 6px
/// radius, and it is a tinted outline (`StyleProfile::Tinted`) rather
/// than a solid fill. A chip is 16px, 4px padding, solid
/// [`vs::BADGE_BG`], and square-ish. Reach for `badge()` when you want
/// shadcn's status pill; reach for `chip` for counts and flags packed
/// into chrome.
///
/// The radius is set with `.radius()` rather than left to the theme, so
/// it is *explicit* and the theme's radius scale leaves it alone. That
/// is what pins a chip at exactly [`vs::RADIUS`] under any scale,
/// including a consumer who squares the app entirely.
pub fn chip(label: impl Into<String>) -> El {
    row([text(label).caption().text_color(vs::BADGE_FG)])
        .fill(vs::BADGE_BG)
        .radius(vs::RADIUS)
        .height(Size::Fixed(vs::CHIP_HEIGHT))
        .width(Size::Hug)
        .padding(Sides::x(tokens::SPACE_1))
        .align(Align::Center)
        .justify(Justify::Center)
}

/// A compact pane title strip with a trailing action slot — VS Code's
/// `sideBarSectionHeader`.
///
/// # Anatomy
///
/// ```text
/// row  fill=sideBarSectionHeader.background
///      border-b=sideBarSectionHeader.border  h=22  px=SPACE_2
///   ├─ text   (title, caption, medium, color=sideBarSectionHeader.foreground)
///   ├─ spacer
///   └─ row    (trailing actions, gap SPACE_1)
/// ```
///
/// The trailing row is always present so the spacer's split point is
/// contents-independent, matching [`status_bar`].
///
/// At [`vs::PANE_HEADER_HEIGHT`] this is the *section header* rung, not
/// VS Code's taller editor group header — it is the strip that caps an
/// accordion section or a docked panel. For an editor group, use core's
/// `editor_tabs` on [`vs::EDITOR_GROUP_HEADER_TABS_BG`].
pub fn pane_header<I, E>(title: impl Into<String>, trailing: I) -> El
where
    I: IntoIterator<Item = E>,
    E: Into<El>,
{
    let trailing: Vec<El> = trailing.into_iter().map(Into::into).collect();

    row([
        text(title)
            .caption()
            .font_weight(FontWeight::Medium)
            .text_color(vs::SIDE_BAR_SECTION_HEADER_FG)
            .ellipsis(),
        spacer(),
        row(trailing).gap(tokens::SPACE_1).align(Align::Center),
    ])
    .fill(vs::SIDE_BAR_SECTION_HEADER_BG)
    .border_b()
    .border_color(vs::SIDE_BAR_SECTION_HEADER_BORDER)
    .padding(Sides::x(tokens::SPACE_2))
    .height(Size::Fixed(vs::PANE_HEADER_HEIGHT))
    .width(Size::Fill(1.0))
    .align(Align::Center)
}

/// A 1px horizontal rule in `tokens::BORDER`.
///
/// Under [`crate::theme::theme`] that token resolves to
/// [`vs::PANEL_BORDER`] (`panel.border`, `#2B2B2B`); it is written as
/// the *stock* token rather than the workbench one so the rule follows a
/// palette swap in a downstream theme of this vocabulary.
///
/// Per-side borders make this unnecessary for the common cases — a bar
/// separating itself from what is above or below it should use
/// `.border_t()` / `.border_b()`, which cost no node and no layout row.
/// `hairline` is for the case per-side borders cannot express: a rule
/// laid *between* siblings that neither sibling owns.
pub fn hairline() -> El {
    divider()
        .width(Size::Fill(1.0))
        .height(Size::Fixed(vs::HAIRLINE))
        .fill(tokens::BORDER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_bar_has_the_documented_anatomy() {
        let bar = status_bar([chip("2")], [text("Ln 1, Col 1")]);

        assert_eq!(bar.height, Size::Fixed(vs::STATUS_BAR_HEIGHT));
        assert_eq!(bar.fill, Some(vs::STATUS_BAR_BG));
        assert_eq!(bar.children.len(), 3, "leading / spacer / trailing");

        let border = bar.border.as_ref().expect("status bar borders itself");
        assert_eq!(border.widths.top, 1.0, "over-rule above the bar");
        assert_eq!(border.widths.bottom, 0.0);
        assert_eq!(border.color, Some(vs::STATUS_BAR_BORDER));

        assert_eq!(bar.children[0].children.len(), 1);
        assert_eq!(bar.children[2].children.len(), 1);
    }

    #[test]
    fn status_bar_item_rows_survive_empty_sides() {
        // The split point must not depend on the contents.
        let bar = status_bar(Vec::<El>::new(), Vec::<El>::new());
        assert_eq!(bar.children.len(), 3);
        assert!(bar.children[0].children.is_empty());
        assert!(bar.children[2].children.is_empty());
    }

    #[test]
    fn chip_is_denser_and_squarer_than_the_stock_badge() {
        use damascene_core::widgets::badge::{BADGE_RADIUS, badge};

        let c = chip("3");
        assert_eq!(c.height, Size::Fixed(vs::CHIP_HEIGHT));
        assert_eq!(c.width, Size::Hug);
        assert_eq!(c.fill, Some(vs::BADGE_BG));
        assert_eq!(c.radius.tl, vs::RADIUS);

        // Measured against the real badge rather than against recalled
        // numbers: shorter than the badge's own default height (the
        // badge metrics ladder only goes up from there — 18/20/24/28)
        // and squarer than its 6px corner.
        let b = badge("3");
        let fixed = |s: Size| match s {
            Size::Fixed(v) => v,
            other => panic!("expected a fixed height, got {other:?}"),
        };
        assert!(fixed(c.height) < fixed(b.height));
        assert!(c.radius.tl < BADGE_RADIUS);
        assert!(b.fill.is_some());
    }

    #[test]
    fn chip_radius_is_explicit_so_the_radius_scale_leaves_it_alone() {
        // If this flag ever stops being set, a consumer squaring the app
        // with `with_radius_scale(0.0)` would flatten chips too.
        assert!(chip("3").explicit_radius);
    }

    #[test]
    fn pane_header_has_the_documented_anatomy() {
        let h = pane_header("EXPLORER", [text("+")]);

        assert_eq!(h.height, Size::Fixed(vs::PANE_HEADER_HEIGHT));
        assert_eq!(h.fill, Some(vs::SIDE_BAR_SECTION_HEADER_BG));
        assert_eq!(h.children.len(), 3, "title / spacer / trailing");
        assert_eq!(h.children[0].text.as_deref(), Some("EXPLORER"));
        assert_eq!(h.children[2].children.len(), 1);

        let border = h.border.as_ref().expect("pane header borders itself");
        assert_eq!(border.widths.bottom, 1.0, "under-rule below the strip");
        assert_eq!(border.widths.top, 0.0);
        assert_eq!(border.color, Some(vs::SIDE_BAR_SECTION_HEADER_BORDER));
    }

    #[test]
    fn pane_header_keeps_its_trailing_slot_when_empty() {
        let h = pane_header("OUTLINE", Vec::<El>::new());
        assert_eq!(h.children.len(), 3);
        assert!(h.children[2].children.is_empty());
    }

    #[test]
    fn hairline_is_one_pixel_of_the_stock_border_token() {
        let r = hairline();
        assert_eq!(r.height, Size::Fixed(vs::HAIRLINE));
        assert_eq!(r.width, Size::Fill(1.0));
        // The *stock* token, so a palette swap moves it.
        assert_eq!(r.fill.and_then(|c| c.token), Some("border"));
    }

    #[test]
    fn hairline_resolves_to_the_panel_border_under_this_theme() {
        let p = crate::theme::palette();
        let fill = hairline().fill.expect("hairline is filled");
        let resolved = p.resolve(fill);
        assert_eq!(
            (resolved.r, resolved.g, resolved.b),
            (vs::PANEL_BORDER.r, vs::PANEL_BORDER.g, vs::PANEL_BORDER.b)
        );
    }
}
