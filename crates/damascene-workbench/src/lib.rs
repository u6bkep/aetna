//! VS Code workbench vocabulary for Damascene.
//!
//! damascene-core speaks **shadcn/Tailwind**, which is a landing-page
//! and dashboard system. This crate is the *application shell* opinion:
//! the VS Code workbench's token vocabulary, its density, and its
//! layered surface model, applied to the stock widget set.
//!
//! Stock damascene stays shadcn. Nothing here replaces or patches the
//! default vocabulary — a `button()` still behaves exactly as it does
//! anywhere else, it is simply 28px, 1.4px-cornered, and painted from
//! Dark Modern values. Forms, dialogs, buttons and controls keep their
//! shadcn anatomy; this crate resizes and squares them and adds the four
//! chrome widgets core lacks.
//!
//! The ratified design record is `docs/WORKBENCH_VISION.md` — including
//! the rejected alternatives (Carbon, Blender-as-skin, the
//! Tailwind-zinc devtool genre) with their reasons.
//!
//! # Why this exists
//!
//! Three real applications built carefully on stock damascene — an ECAD
//! workbench, a voice client, a slicer — consistently read as demos
//! rather than tools, without fighting the theme anywhere. The
//! diagnosis was four structural signals that landing-page design
//! maximizes and professional tools minimize:
//!
//! | signal | shadcn | workbench |
//! |---|---|---|
//! | control scale | 36px `Md` controls, 14px type | 28px `Xs` controls, 13px type |
//! | radius + shadow | 7–12px, floating shadows | ~2px, flat chrome (overlays keep `SHADOW_MD`) |
//! | surfaces | one bg, outlined cards floating on it | layered fills, 1px separators |
//! | whitespace | spent to look calm | spent on information |
//!
//! All four run through core theme knobs (`with_default_component_size`,
//! `with_radius_scale`, `with_shadow_scale`, `with_type_scale`, the
//! palette) — the crate contributes values and chrome recipes, no
//! mechanisms.
//!
//! # Why VS Code specifically
//!
//! Damascene's premise is vocabulary parity with the LLM training
//! distribution. VS Code's theme keys are the most heavily
//! corpus-represented dense-app token system in existence — hundreds of
//! thousands of published theme files name `sideBar.background`,
//! `statusBar.background`, `focusBorder` — and the product is the most
//! used developer tool on earth. An agent told "make it look like VS
//! Code" has deeper priors here than for any other tool aesthetic.
//!
//! Not binding: VS Code's DOM, its widget internals, or its widget
//! shapes where shadcn already has an anatomy. A select is still a
//! shadcn select, at workbench size and radius.
//!
//! # Usage
//!
//! ```no_run
//! use damascene_core::prelude::*;
//! use damascene_workbench::{chrome::*, theme, tokens as vs};
//!
//! struct Shell;
//!
//! impl App for Shell {
//!     fn build(&self, _cx: &BuildCx) -> El {
//!         column([
//!             row([
//!                 column([pane_header("EXPLORER", Vec::<El>::new())])
//!                     .width(Size::Fixed(220.0))
//!                     .height(Size::Fill(1.0))
//!                     .fill(vs::SIDE_BAR_BG)
//!                     .border_r()
//!                     .border_color(vs::SIDE_BAR_BORDER),
//!                 column([text("editor")])
//!                     .width(Size::Fill(1.0))
//!                     .height(Size::Fill(1.0))
//!                     .fill(vs::EDITOR_BG),
//!             ])
//!             .height(Size::Fill(1.0))
//!             .align(Align::Stretch),
//!             status_bar([text("main").caption()], [chip("2")]),
//!         ])
//!         .align(Align::Stretch)
//!     }
//!
//!     // Without this, stock controls render in shadcn zinc at 36px.
//!     fn theme(&self) -> Theme {
//!         theme::theme()
//!     }
//! }
//! ```
//!
//! Run the full example with
//! `cargo run -p damascene-workbench --example shell`.
//!
//! # Calibration
//!
//! **Color is calibrated; metrics are not.**
//! `references/vscode-calibration/` vendors VS Code's own Dark Modern
//! theme chain verbatim from a pinned upstream commit, and every color
//! in [`tokens`] cites its key and value. The metric constants
//! (bar heights, chip height, the 2px radius target) are the indicative
//! figures from `docs/WORKBENCH_VISION.md` and remain hypotheses until
//! the screenshot-measurement step of that document's calibration plan
//! lands. They are marked individually in [`tokens`].
//!
//! # Status
//!
//! First cut. Present: the palette, the dense metrics profile, and the
//! chrome widgets core lacks. Deferred deliberately (see
//! `docs/WORKBENCH_VISION.md`, "Non-goals for the first cut"): a shared
//! split-tree, a stock tree view, and `data_table`.

#![warn(missing_docs)]

pub mod chrome;
pub mod theme;
pub mod tokens;
