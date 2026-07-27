//! Badge — small status chip (rounded-md, per current shadcn).
//!
//! Default style is `info` (accent-tinted). Apply status modifiers from
//! [`crate::style`]: `.success()`, `.warning()`, `.destructive()`,
//! `.info()`, `.muted()`.
//!
//! `.outline()` produces shadcn's `Badge` `variant="outline"`:
//! transparent fill, `border`-token stroke, plain foreground label —
//! the quiet chip for metadata (`badge("v2").outline()`) where a
//! status tint would over-claim.

// Lock in full per-item documentation for this module (issue #73).
#![warn(missing_docs)]

use std::panic::Location;

use crate::metrics::MetricsRole;
use crate::style::StyleProfile;
use crate::tokens;
use crate::tree::*;

/// Badge corner radius — shadcn's `rounded-md` (current shadcn badges
/// moved off the full pill).
pub const BADGE_RADIUS: f32 = 6.0;

/// Small status chip (shadcn's `Badge`) — hugging caption text in a
/// tinted, rounded-md shell. Defaults to the `info` tint; chain a
/// status modifier (`.success()`, `.warning()`, …) for other variants,
/// or `.outline()` for shadcn's `variant="outline"` (transparent fill,
/// `border` stroke, foreground label).
#[track_caller]
pub fn badge(label: impl Into<String>) -> El {
    El::new(Kind::Badge)
        .at_loc(Location::caller())
        .style_profile(StyleProfile::Tinted)
        .metrics_role(MetricsRole::Badge)
        .text(label)
        .text_align(TextAlign::Center)
        .caption()
        .font_weight(FontWeight::Medium)
        .text_color(tokens::INFO)
        .fill(tokens::INFO.with_alpha_u8(38))
        .stroke(tokens::INFO.with_alpha_u8(120))
        .default_radius(BADGE_RADIUS)
        .width(Size::Hug)
        .default_height(Size::Fixed(20.0))
        .default_padding(Sides::xy(tokens::SPACE_2, 0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_defaults_to_the_info_tint() {
        let b = badge("Online");
        assert_eq!(b.kind, Kind::Badge);
        assert_eq!(b.style_profile, StyleProfile::Tinted);
        assert_eq!(b.text.as_deref(), Some("Online"));
        assert_eq!(b.text_role, TextRole::Caption);
        assert_eq!(b.font_weight, FontWeight::Medium);
        assert_eq!(b.fill, Some(tokens::INFO.with_alpha_u8(38)));
        assert_eq!(b.stroke, Some(tokens::INFO.with_alpha_u8(120)));
        assert_eq!(b.text_color, Some(tokens::INFO));
        assert_eq!(b.width, Size::Hug);
    }

    #[test]
    fn badge_outline_is_transparent_with_a_border_stroke() {
        // shadcn `Badge` variant="outline": `border-border
        // text-foreground`, no background.
        let b = badge("v2").outline();
        assert!(b.fill.is_none(), "outline badge paints no fill");
        assert_eq!(b.stroke, Some(tokens::BORDER));
        assert_eq!(b.stroke_width, 1.0);
        assert_eq!(b.text_color, Some(tokens::FOREGROUND));
        // Geometry is the shared badge recipe — outline changes color
        // only.
        assert_eq!(b.radius, Corners::all(BADGE_RADIUS));
        assert_eq!(b.height, Size::Fixed(20.0));
        assert_eq!(b.font_weight, FontWeight::Medium);
    }

    #[test]
    fn badge_outline_stroke_is_border_not_input() {
        // Regression guard for the profile-aware `.outline()` split:
        // chips take shadcn's `border`, controls take `input`. The two
        // tokens happen to carry the same rgb in the stock dark
        // palette, so only the token name distinguishes them — and it
        // is the token that a palette swap re-resolves.
        let chip = badge("v2").outline();
        assert_eq!(chip.stroke.and_then(|c| c.token), Some("border"));

        let control = crate::button("Outline").outline();
        assert_eq!(control.stroke.and_then(|c| c.token), Some("input"));
    }

    #[test]
    fn badge_tinted_variants_are_unchanged_by_the_outline_split() {
        for (b, tint) in [
            (badge("ok").success(), tokens::SUCCESS),
            (badge("warn").warning(), tokens::WARNING),
            (badge("err").destructive(), tokens::DESTRUCTIVE),
            (badge("info").info(), tokens::INFO),
            (badge("brand").primary(), tokens::PRIMARY),
        ] {
            assert_eq!(b.fill, Some(tint.with_alpha_u8(38)));
            assert_eq!(b.stroke, Some(tint.with_alpha_u8(120)));
            assert_eq!(b.stroke_width, 1.0);
            assert_eq!(b.text_color, Some(tint));
        }

        // `.muted()` still swaps to the neutral surface, not a tint.
        let m = badge("draft").muted();
        assert_eq!(m.fill, Some(tokens::MUTED));
        assert_eq!(m.stroke, Some(tokens::BORDER));
        assert_eq!(m.text_color, Some(tokens::MUTED_FOREGROUND));
    }
}
