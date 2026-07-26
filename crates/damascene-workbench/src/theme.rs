//! The workbench theme — palette, metrics, and fonts as one decision.
//!
//! # Two vocabularies, one set of pixels
//!
//! An app on this theme paints from two token sets at once:
//!
//! - **VS Code keys** ([`crate::tokens`]) used by this crate's chrome.
//! - **shadcn names** (`damascene_core::tokens`) used by every stock
//!   widget you drop in — `button`, `checkbox`, `text_input`, `sidebar`,
//!   `editor_tabs`.
//!
//! Unlike the retired Carbon experiment, both sets resolve through the
//! *same* [`Palette`]: [`palette`] maps VS Code values onto the shadcn
//! slots **and** registers the workbench keys through
//! `Palette::with_token`, so a palette swap moves both. That is what the
//! open token namespace bought (see `docs/VOCABULARY_PARITY.md` §2).
//!
//! # Dark only
//!
//! `docs/WORKBENCH_VISION.md` left "whether the crate ships light themes
//! at all" open. It does not, yet: Dark Modern is the calibrated
//! reference and there is no consumer asking for light. When one does,
//! the shape is a second `palette_light()` over VS Code's Light Modern.

#![warn(missing_docs)]

use damascene_core::Theme;
use damascene_core::metrics::ComponentSize;
use damascene_core::theme::palette::Palette;
use damascene_core::tree::FontFamily;

use crate::tokens as vs;

/// The multiplicative radius scale that squares the shadcn ladder down
/// to the workbench's ~0–4px band.
///
/// # The arithmetic
///
/// `Theme::with_radius_scale` multiplies every *theme-default* (not
/// author-specified) nonzero corner, including the output of
/// `control_metrics` — which is the ladder that actually rounds buttons,
/// icon buttons and inputs (`damascene-core/src/metrics.rs`):
///
/// | `ComponentSize` | stock control radius | × scale |
/// |---|---|---|
/// | `Xs` | 5px | 1.43px |
/// | `Sm` | 6px | 1.71px |
/// | `Md` | 7px | **2.00px** |
/// | `Lg` | 8px | 2.29px |
///
/// Solving the `Md` row for [`vs::RADIUS`] (2px) gives the constant:
///
/// ```text
/// scale = RADIUS / control_radius(Md) = 2.0 / 7.0 = 0.285714…
/// ```
///
/// `Md` is the solve point because it is shadcn's own default rung — the
/// one a reader means by "the default control radius" — even though this
/// theme ships controls at `Xs`. Anchoring on the shared reference rung
/// keeps the number checkable against upstream shadcn rather than
/// against a local choice.
///
/// The container ladder rides along: `RADIUS_SM` 4px → 1.14px,
/// `RADIUS_MD` 8px → 2.29px, `RADIUS_LG` 12px → 3.43px, and the stock
/// badge's 6px → 1.71px. The whole system lands inside the 0–5px band
/// `docs/WORKBENCH_VISION.md` measured off real tools.
///
/// Two carve-outs in core apply and are wanted here: `RADIUS_PILL`
/// (999px) is exempt, so avatars and pill affordances stay round the way
/// `rounded-full` ignores `--radius` on the web; and an explicit
/// `.radius()` survives untouched, which is how [`crate::chrome::chip`]
/// pins itself at exactly 2px instead of inheriting a scaled value.
pub const RADIUS_SCALE: f32 = vs::RADIUS / 7.0;

/// Type scale: 13/14 — VS Code's 13px workbench UI font over
/// damascene's 14px `TEXT_SM` body/label baseline.
///
/// `Theme::with_type_scale` is the rem analogue: every role- and
/// rung-derived size scales together, line heights included, so the
/// whole ladder lands where VS Code's does — body/label 14 → 13px,
/// caption 12 → 11.1px (VS Code's small text runs ~11px), titles and
/// headings proportional. Hand-picked `.font_size(...)` values
/// survive, exactly as `text-[15px]` ignores a rem theme. Control
/// heights are deliberately independent — density is
/// `with_default_component_size`'s job.
pub const TYPE_SCALE: f32 = 13.0 / 14.0;

/// Dark Modern mapped onto the shadcn palette slots, plus every
/// workbench key registered as an extra token.
///
/// # The slot mapping
///
/// | slot | VS Code key | value |
/// |---|---|---|
/// | `background` | `editor.background` | `#1F1F1F` |
/// | `foreground` | `foreground` | `#CCCCCC` |
/// | `card` | `sideBar.background` | `#181818` |
/// | `card-foreground` | `sideBar.foreground` | `#CCCCCC` |
/// | `popover` | `quickInput.background` | `#222222` |
/// | `popover-foreground` | `quickInput.foreground` | `#CCCCCC` |
/// | `primary` | `button.background` | `#0078D4` |
/// | `primary-foreground` | `button.foreground` | `#FFFFFF` |
/// | `secondary` | `button.secondaryHoverBackground` | `#2B2B2B` |
/// | `secondary-foreground` | `button.secondaryForeground` | `#CCCCCC` |
/// | `muted` | `list.hoverBackground` | `#2A2D2E` |
/// | `muted-foreground` | `descriptionForeground` | `#9D9D9D` |
/// | `accent` | `list.activeSelectionBackground` | `#04395E` |
/// | `accent-foreground` | `list.activeSelectionForeground` | `#FFFFFF` |
/// | `destructive` | `errorForeground` | `#F85149` |
/// | `destructive-foreground` | `button.foreground` | `#FFFFFF` |
/// | `border` | `panel.border` | `#2B2B2B` |
/// | `input` | `input.border` | `#3C3C3C` |
/// | `ring` | `focusBorder` | `#0078D4` |
/// | `success` | `editorGutter.addedBackground` | `#2EA043` |
/// | `success-foreground` | `button.foreground` | `#FFFFFF` |
/// | `warning` | `chat.editedFileForeground` | `#E2C08D` |
/// | `warning-foreground` | `editor.background` | `#1F1F1F` |
/// | `info` | `editorGutter.modifiedBackground` | `#0078D4` |
/// | `info-foreground` | `button.foreground` | `#FFFFFF` |
/// | `link-foreground` | `textLink.foreground` | `#4daafc` |
/// | `selection-bg` | `list.activeSelectionBackground` | `#04395E` |
/// | `selection-bg-unfocused` | `editor.inactiveSelectionBackground` | `#3A3D41` |
///
/// # The rows that are decisions rather than transcriptions
///
/// - **`card` = `sideBar.background`.** The load-bearing row. In the
///   stock dark palette `card == background`, so a card is visible only
///   as an outline and reads as a sheet floating on a canvas. Pointing
///   `card` at the side bar's *darker* value converts every stock
///   `card()`, `sidebar()` and `popover`-adjacent surface into a layered
///   workbench panel — diagnosis 3 of `docs/WORKBENCH_VISION.md`, fixed
///   by one row of a table.
/// - **`secondary` = `button.secondaryHoverBackground`.** VS Code's
///   `button.secondaryBackground` is literally `#00000000` — its
///   secondary button is a ghost that only acquires a fill on hover.
///   shadcn's `secondary` slot is a *rest* fill and is used for far more
///   than buttons, so transcribing the transparent value would make
///   secondary surfaces vanish. The hover value is the same color family
///   at the intended weight. The literal key is still available as
///   [`vs::BUTTON_SECONDARY_BG`].
/// - **`warning` = `chat.editedFileForeground`.** Dark Modern sets no
///   `*Warning.foreground` key at all. This is the theme's own amber for
///   "touched, look at it", and it is the only amber in the chain at a
///   brightness matching `destructive` (`#F85149`) and `success`
///   (`#2EA043`). Because it is a light tint rather than a saturated
///   fill, `warning-foreground` is dark (`editor.background`) where the
///   other status foregrounds are white.
/// - **`muted`/`accent` split.** The stock dark palette collapses
///   `secondary`/`muted`/`accent`/`border`/`input` onto one value. Here
///   `accent` is the *selected* list row and `muted` is the *hovered*
///   one, which is exactly the pair VS Code distinguishes and is what
///   makes list interaction legible.
///
/// # Slots left on the stock dark values
///
/// `overlay-scrim`, `scrollbar-thumb` and `scrollbar-thumb-active` come
/// through `..Palette::damascene_dark()`. No theme in the Dark Modern
/// chain sets a scrim or scrollbar key, and the calibration reference
/// deliberately vendors only the theme files — inventing values here
/// would be recalled color, which the calibration plan forbids.
pub fn palette() -> Palette {
    let p = Palette {
        background: vs::EDITOR_BG,
        foreground: vs::FOREGROUND,

        card: vs::SIDE_BAR_BG,
        card_foreground: vs::SIDE_BAR_FG,

        popover: vs::QUICK_INPUT_BG,
        popover_foreground: vs::QUICK_INPUT_FG,

        primary: vs::BUTTON_BG,
        primary_foreground: vs::BUTTON_FG,

        secondary: vs::BUTTON_SECONDARY_HOVER_BG,
        secondary_foreground: vs::BUTTON_SECONDARY_FG,

        muted: vs::LIST_HOVER_BG,
        muted_foreground: vs::DESCRIPTION_FG,

        accent: vs::LIST_ACTIVE_SELECTION_BG,
        accent_foreground: vs::LIST_ACTIVE_SELECTION_FG,

        destructive: vs::ERROR_FG,
        destructive_foreground: vs::BUTTON_FG,

        border: vs::PANEL_BORDER,
        input: vs::INPUT_BORDER,
        ring: vs::FOCUS_BORDER,

        success: vs::EDITOR_GUTTER_ADDED_BG,
        success_foreground: vs::BUTTON_FG,
        warning: vs::CHAT_EDITED_FILE_FG,
        warning_foreground: vs::EDITOR_BG,
        info: vs::EDITOR_GUTTER_MODIFIED_BG,
        info_foreground: vs::BUTTON_FG,

        link_foreground: vs::TEXT_LINK_FG,

        selection_bg: vs::LIST_ACTIVE_SELECTION_BG,
        selection_bg_unfocused: vs::EDITOR_INACTIVE_SELECTION_BG,

        ..Palette::damascene_dark()
    };
    register_workbench_tokens(p)
}

/// Register the workbench key set on a palette's extra namespace.
///
/// Split out from [`palette`] so a downstream *theme* of the workbench
/// vocabulary — eutectic's ui-oracle values, say — can start from its own
/// slot mapping and still resolve `sideBar.background` and friends.
///
/// Keys whose value is a pure transcription of a slot are registered
/// anyway: the point of the namespace is that
/// `Color::srgb_token("sideBar.background", …)` follows a palette swap,
/// which requires the entry to exist even when it currently agrees with
/// `card`.
///
/// `foreground` is deliberately absent: it is a stock shadcn name, so
/// `Palette::lookup` would shadow the extra entry with `Palette::foreground`
/// before ever consulting the map. Registering it would be dead weight
/// that reads as if it worked.
///
/// # Retheming a single key
///
/// Registering the same extra name twice keeps the last color, so an app
/// recolors the workbench by chaining `with_token` *after* this call — no
/// forked constants, and every `vs::` color carrying that name re-points
/// at paint time:
///
/// ```ignore
/// use damascene_core::Color;
/// use damascene_core::theme::palette::Palette;
/// use damascene_workbench::theme;
///
/// // Blue side bar everywhere `vs::SIDE_BAR_BG` is painted — including
/// // the shadcn `card` slot, which maps to that same token.
/// let p: Palette = theme::register_workbench_tokens(Palette::damascene_dark())
///     .with_token("sideBar.background", Color::rgba(20, 30, 60, 255));
/// ```
///
/// The order matters only against this function; stock shadcn names
/// (`background`, `foreground`, `border`, …) still win over extras and
/// have to be set on the palette's own fields instead.
pub fn register_workbench_tokens(p: Palette) -> Palette {
    p
        // Editor / text
        .with_token("editor.background", vs::EDITOR_BG)
        .with_token("editor.foreground", vs::EDITOR_FG)
        .with_token("descriptionForeground", vs::DESCRIPTION_FG)
        .with_token("errorForeground", vs::ERROR_FG)
        .with_token("textLink.foreground", vs::TEXT_LINK_FG)
        // Side bar
        .with_token("sideBar.background", vs::SIDE_BAR_BG)
        .with_token("sideBar.foreground", vs::SIDE_BAR_FG)
        .with_token("sideBar.border", vs::SIDE_BAR_BORDER)
        .with_token("sideBarTitle.foreground", vs::SIDE_BAR_TITLE_FG)
        .with_token(
            "sideBarSectionHeader.background",
            vs::SIDE_BAR_SECTION_HEADER_BG,
        )
        .with_token(
            "sideBarSectionHeader.foreground",
            vs::SIDE_BAR_SECTION_HEADER_FG,
        )
        .with_token(
            "sideBarSectionHeader.border",
            vs::SIDE_BAR_SECTION_HEADER_BORDER,
        )
        // Bars
        .with_token("statusBar.background", vs::STATUS_BAR_BG)
        .with_token("statusBar.foreground", vs::STATUS_BAR_FG)
        .with_token("statusBar.border", vs::STATUS_BAR_BORDER)
        .with_token(
            "statusBarItem.hoverBackground",
            vs::STATUS_BAR_ITEM_HOVER_BG,
        )
        .with_token(
            "statusBarItem.remoteBackground",
            vs::STATUS_BAR_ITEM_REMOTE_BG,
        )
        .with_token(
            "statusBarItem.remoteForeground",
            vs::STATUS_BAR_ITEM_REMOTE_FG,
        )
        .with_token(
            "statusBarItem.prominentBackground",
            vs::STATUS_BAR_ITEM_PROMINENT_BG,
        )
        .with_token("titleBar.activeBackground", vs::TITLE_BAR_ACTIVE_BG)
        .with_token("titleBar.activeForeground", vs::TITLE_BAR_ACTIVE_FG)
        .with_token("titleBar.border", vs::TITLE_BAR_BORDER)
        .with_token("activityBar.background", vs::ACTIVITY_BAR_BG)
        .with_token("activityBar.foreground", vs::ACTIVITY_BAR_FG)
        .with_token(
            "activityBar.inactiveForeground",
            vs::ACTIVITY_BAR_INACTIVE_FG,
        )
        .with_token("activityBar.border", vs::ACTIVITY_BAR_BORDER)
        .with_token("panel.background", vs::PANEL_BG)
        .with_token("panel.border", vs::PANEL_BORDER)
        .with_token("panelTitle.activeForeground", vs::PANEL_TITLE_ACTIVE_FG)
        .with_token("panelTitle.inactiveForeground", vs::PANEL_TITLE_INACTIVE_FG)
        // Editor group header and tabs
        .with_token(
            "editorGroupHeader.tabsBackground",
            vs::EDITOR_GROUP_HEADER_TABS_BG,
        )
        .with_token(
            "editorGroupHeader.tabsBorder",
            vs::EDITOR_GROUP_HEADER_TABS_BORDER,
        )
        .with_token("editorGroup.border", vs::EDITOR_GROUP_BORDER)
        .with_token("tab.activeBackground", vs::TAB_ACTIVE_BG)
        .with_token("tab.activeForeground", vs::TAB_ACTIVE_FG)
        .with_token("tab.inactiveBackground", vs::TAB_INACTIVE_BG)
        .with_token("tab.inactiveForeground", vs::TAB_INACTIVE_FG)
        .with_token("tab.border", vs::TAB_BORDER)
        .with_token("tab.activeBorderTop", vs::TAB_ACTIVE_BORDER_TOP)
        // Controls
        .with_token("focusBorder", vs::FOCUS_BORDER)
        .with_token("button.background", vs::BUTTON_BG)
        .with_token("button.foreground", vs::BUTTON_FG)
        .with_token("button.hoverBackground", vs::BUTTON_HOVER_BG)
        .with_token("button.secondaryBackground", vs::BUTTON_SECONDARY_BG)
        .with_token("button.secondaryForeground", vs::BUTTON_SECONDARY_FG)
        .with_token(
            "button.secondaryHoverBackground",
            vs::BUTTON_SECONDARY_HOVER_BG,
        )
        .with_token("input.background", vs::INPUT_BG)
        .with_token("input.border", vs::INPUT_BORDER)
        .with_token("input.foreground", vs::INPUT_FG)
        .with_token("input.placeholderForeground", vs::INPUT_PLACEHOLDER_FG)
        .with_token("dropdown.background", vs::DROPDOWN_BG)
        .with_token("dropdown.border", vs::DROPDOWN_BORDER)
        .with_token("dropdown.listBackground", vs::DROPDOWN_LIST_BG)
        .with_token("checkbox.background", vs::CHECKBOX_BG)
        .with_token("checkbox.border", vs::CHECKBOX_BORDER)
        .with_token("progressBar.background", vs::PROGRESS_BAR_BG)
        // Floating surfaces
        .with_token("quickInput.background", vs::QUICK_INPUT_BG)
        .with_token("quickInput.foreground", vs::QUICK_INPUT_FG)
        .with_token("editorWidget.background", vs::EDITOR_WIDGET_BG)
        .with_token("widget.border", vs::WIDGET_BORDER)
        .with_token("menu.background", vs::MENU_BG)
        .with_token("menu.selectionBackground", vs::MENU_SELECTION_BG)
        .with_token("pickerGroup.border", vs::PICKER_GROUP_BORDER)
        // Badge and list
        .with_token("badge.background", vs::BADGE_BG)
        .with_token("badge.foreground", vs::BADGE_FG)
        .with_token(
            "list.activeSelectionBackground",
            vs::LIST_ACTIVE_SELECTION_BG,
        )
        .with_token(
            "list.activeSelectionForeground",
            vs::LIST_ACTIVE_SELECTION_FG,
        )
        .with_token(
            "list.inactiveSelectionBackground",
            vs::LIST_INACTIVE_SELECTION_BG,
        )
        .with_token("list.hoverBackground", vs::LIST_HOVER_BG)
        // Status colors and selection
        .with_token("editorGutter.addedBackground", vs::EDITOR_GUTTER_ADDED_BG)
        .with_token(
            "editorGutter.deletedBackground",
            vs::EDITOR_GUTTER_DELETED_BG,
        )
        .with_token(
            "editorGutter.modifiedBackground",
            vs::EDITOR_GUTTER_MODIFIED_BG,
        )
        .with_token("chat.editedFileForeground", vs::CHAT_EDITED_FILE_FG)
        .with_token(
            "editor.inactiveSelectionBackground",
            vs::EDITOR_INACTIVE_SELECTION_BG,
        )
}

/// The VS Code workbench theme: [`palette`], `Xs` controls,
/// [`RADIUS_SCALE`], flat chrome with shadowed overlays, Inter +
/// JetBrains Mono.
///
/// The four structural inversions from `docs/WORKBENCH_VISION.md`,
/// and where each one lives:
///
/// 1. **Control scale** — `with_default_component_size(Xs)` puts stock
///    buttons and inputs on the 28px rung instead of shadcn's 36px `Md`,
///    and [`TYPE_SCALE`] takes the whole type ladder to VS Code's 13px
///    baseline (body/label 13px, captions ~11px).
/// 2. **Radius + shadow** — [`RADIUS_SCALE`] takes every theme-default
///    corner, controls included, down to the 1–3.5px band, and
///    `with_shadow_scale(0.0)` flattens every recipe shadow: cards and
///    buttons stop floating. Overlays are re-elevated selectively — a
///    `Popover`-role uniform restores `SHADOW_MD` on menus, tooltips,
///    dialogs, and palettes, matching VS Code's shadowed
///    `widget.shadow` on its otherwise flat chrome.
/// 3. **Layered surfaces** — the `card = sideBar.background` row of
///    [`palette`], plus [`crate::chrome`]'s hairline separators.
/// 4. **Whitespace** — the crate's chrome recipes; the stock container
///    paddings are unchanged, because damascene deliberately has no
///    density knob (`docs/VOCABULARY_PARITY.md`, "Rejected").
///
/// Fonts follow the stock configuration — Inter for UI, JetBrains Mono
/// for code — which is also what a default `Theme` already carries. Both
/// are set explicitly so the theme states its whole intent in one place
/// rather than inheriting half of it.
pub fn theme() -> Theme {
    use damascene_core::shader::UniformValue;
    use damascene_core::tree::SurfaceRole;

    Theme::default()
        .with_palette(palette())
        .with_default_component_size(ComponentSize::Xs)
        .with_radius_scale(RADIUS_SCALE)
        .with_shadow_scale(0.0)
        .with_type_scale(TYPE_SCALE)
        // Flat app, shadowed overlays: the scaled-away role default is
        // omitted rather than zeroed, exactly so this re-elevation can
        // land (see `Theme::with_shadow_scale`). One tier for the whole
        // popover family — VS Code's single `widget.shadow` — rather
        // than shadcn's md/lg split.
        .with_role_uniform(
            SurfaceRole::Popover,
            "shadow",
            UniformValue::F32(damascene_core::tokens::SHADOW_MD),
        )
        .with_font_family(FontFamily::Inter)
        .with_mono_font_family(FontFamily::JetBrainsMono)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb(c: damascene_core::tree::Color) -> (f32, f32, f32) {
        (c.r, c.g, c.b)
    }

    #[test]
    fn card_is_the_side_bar_not_the_background() {
        // The load-bearing row: this is what converts floating cards
        // into layered panels. If it ever collapses back to
        // `background`, the crate's reason to exist is gone.
        let p = palette();
        assert_ne!(rgb(p.card), rgb(p.background));
        assert_eq!(rgb(p.card), rgb(vs::SIDE_BAR_BG));
    }

    #[test]
    fn docked_surfaces_sink_and_floating_surfaces_rise() {
        let p = palette();
        assert!(p.card.r < p.background.r, "side bar sinks below the editor");
        assert!(
            p.popover.r > p.background.r,
            "quick input rises above the editor"
        );
    }

    #[test]
    fn muted_accent_border_and_input_are_not_collapsed() {
        // The stock dark palette maps secondary/muted/accent/border/input
        // onto one value; VS Code distinguishes all of them.
        let p = palette();
        assert_ne!(rgb(p.accent), rgb(p.muted));
        assert_ne!(rgb(p.border), rgb(p.input));
        assert_ne!(rgb(p.secondary), rgb(p.muted));
    }

    #[test]
    fn secondary_is_not_the_transparent_literal() {
        // `button.secondaryBackground` is `#00000000`. Transcribing it
        // would make every secondary surface invisible.
        let p = palette();
        assert!(p.secondary.a > 0.0);
        assert_eq!(rgb(p.secondary), rgb(vs::BUTTON_SECONDARY_HOVER_BG));
    }

    #[test]
    fn workbench_keys_resolve_through_the_palette() {
        // The whole point of the open token namespace: a VS Code key
        // minted with `srgb_token` must come back from `lookup`, not
        // fall through to its baked literal.
        let p = palette();
        for key in [
            "sideBar.background",
            "statusBar.background",
            "panel.border",
            "focusBorder",
            "button.background",
            "quickInput.background",
            "badge.background",
            "list.activeSelectionBackground",
            "list.hoverBackground",
            "editorGroupHeader.tabsBackground",
            "tab.activeBackground",
            "titleBar.activeBackground",
        ] {
            assert!(p.lookup(key).is_some(), "{key} must resolve");
        }
        assert_eq!(
            rgb(p.lookup("sideBar.background").unwrap()),
            rgb(vs::SIDE_BAR_BG)
        );
        assert_eq!(
            rgb(p.lookup("badge.background").unwrap()),
            rgb(vs::BADGE_BG)
        );
    }

    #[test]
    fn every_workbench_token_resolves_to_itself() {
        // Guards the registration list against drift: any constant in
        // `tokens` whose key is registered must round-trip to its own
        // value. `foreground` is the documented exception — a stock
        // shadcn name that `lookup` shadows before reaching the map.
        let p = palette();
        for c in crate::tokens::ALL {
            let key = c.token.expect("workbench tokens carry their key");
            let resolved = p.lookup(key).unwrap_or_else(|| panic!("{key} unresolved"));
            if key == "foreground" {
                // Shadowed by `Palette::foreground` — same value here,
                // by construction of the slot mapping.
                assert_eq!(rgb(resolved), rgb(p.foreground));
            } else {
                assert_eq!(
                    rgb(resolved),
                    rgb(*c),
                    "{key} resolved to a different value"
                );
            }
        }
    }

    #[test]
    fn stock_border_token_resolves_to_the_panel_hairline() {
        // Chrome that paints `damascene_core::tokens::BORDER` — every
        // stock widget, and `chrome::hairline` — must land on VS Code's
        // hairline color under this theme.
        let p = palette();
        assert_eq!(
            rgb(p.lookup("border").unwrap()),
            rgb(vs::PANEL_BORDER),
            "the stock `border` token must resolve to panel.border"
        );
    }

    #[test]
    fn radius_scale_lands_a_default_md_control_on_the_workbench_radius() {
        // The documented solve: scale = RADIUS / control_radius(Md).
        // `control_metrics` is private, so the ladder value is restated
        // here; if core's Md control radius ever moves off 7px this
        // assertion is the tripwire, via the rendered-radius test below.
        let md_control_radius = 7.0_f32;
        assert!((RADIUS_SCALE * md_control_radius - vs::RADIUS).abs() < 1e-4);
    }

    #[test]
    fn radius_scale_squares_a_stock_button_through_the_metrics_pass() {
        // End-to-end rather than arithmetic-only: the metrics pass is
        // `pub(crate)`, so drive it the way a host does — through the
        // bundle renderer, which applies the theme to the tree in place.
        use damascene_core::bundle::artifact::render_bundle_themed;
        use damascene_core::tree::Rect;
        use damascene_core::widgets::button::button;

        let t = theme();
        let mut el = button("Save");
        render_bundle_themed(&mut el, Rect::new(0.0, 0.0, 200.0, 60.0), &t);
        assert!(
            el.radius.tl > 0.0 && el.radius.tl < 2.0,
            "an Xs button (5px stock) must land in the workbench radius band, got {}",
            el.radius.tl
        );
        // 28px, not shadcn's 36px `Md` — inversion 1.
        assert_eq!(el.height, damascene_core::tree::Size::Fixed(28.0));
    }

    #[test]
    fn theme_builds_with_the_workbench_palette_and_metrics() {
        let t = theme();
        assert_eq!(rgb(t.palette().card), rgb(vs::SIDE_BAR_BG));
        assert_eq!(
            t.metrics().default_component_size(),
            ComponentSize::Xs,
            "workbench controls sit on the 28px rung"
        );
        assert_eq!(t.metrics().radius_scale(), RADIUS_SCALE);
        assert_eq!(t.mono_font_family(), FontFamily::JetBrainsMono);
        assert_eq!(t.font_family(), FontFamily::Inter);
    }
}
