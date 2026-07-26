//! The whole-shell lint guard.
//!
//! The unit tests in `src/` check anatomy on unlaid-out trees. This one
//! runs a shell built from all four [`damascene_workbench::chrome`]
//! recipes *plus* stock widgets through the real pipeline — theme,
//! metrics, layout, draw ops, lint — because the interesting failures of
//! an opinion crate are cross-cutting: a chrome fill that fails contrast
//! against themed text, a raw (untokened) color, a full-bleed region the
//! window-padding lint objects to.
//!
//! It mirrors `examples/shell.rs`. When that example's anatomy changes,
//! this tree should follow.

use damascene_core::prelude::*;
use damascene_workbench::{chrome::*, theme, tokens as vs};

#[test]
fn shell_is_lint_clean() {
    let mut root = column([
        row([text("t").caption()])
            .fill(vs::TITLE_BAR_ACTIVE_BG)
            .border_b()
            .border_color(vs::TITLE_BAR_BORDER)
            .padding(Sides::x(tokens::SPACE_2))
            .height(Size::Fixed(vs::TITLE_BAR_HEIGHT))
            .width(Size::Fill(1.0))
            .align(Align::Center),
        row([
            column([
                pane_header("EXPLORER", [chip("3")]),
                column([sidebar_menu(vec![
                    sidebar_menu_button("main.rs", true).key("f:1"),
                    sidebar_menu_button("theme.rs", false).key("f:2"),
                ])])
                .padding(tokens::SPACE_2)
                .width(Size::Fill(1.0))
                .height(Size::Fill(1.0)),
            ])
            .fill(vs::SIDE_BAR_BG)
            .border_r()
            .border_color(vs::SIDE_BAR_BORDER)
            .width(Size::Fixed(220.0))
            .height(Size::Fill(1.0))
            .align(Align::Stretch),
            column([
                editor_tabs("editors", &"a", [("a", "main.rs"), ("b", "theme.rs")])
                    .fill(vs::EDITOR_GROUP_HEADER_TABS_BG)
                    .border_b()
                    .border_color(vs::EDITOR_GROUP_HEADER_TABS_BORDER)
                    .width(Size::Fill(1.0)),
                column([
                    row([
                        button("Run").key("run").primary(),
                        spacer(),
                        chip("modified"),
                    ])
                    .width(Size::Fill(1.0))
                    .align(Align::Center),
                    hairline(),
                    card([
                        card_header([card_title("Panel")]),
                        card_content([text("x").caption()]),
                    ])
                    .width(Size::Fill(1.0)),
                    spacer(),
                ])
                .gap(tokens::SPACE_3)
                .padding(tokens::SPACE_3)
                .fill(vs::EDITOR_BG)
                .width(Size::Fill(1.0))
                .height(Size::Fill(1.0))
                .align(Align::Stretch),
            ])
            .width(Size::Fill(1.0))
            .height(Size::Fill(1.0))
            .align(Align::Stretch),
        ])
        .height(Size::Fill(1.0))
        .width(Size::Fill(1.0))
        .align(Align::Stretch),
        status_bar(
            [text("main*").caption(), chip("6")],
            [text("Ln 1, Col 1").caption()],
        ),
    ])
    .align(Align::Stretch)
    .fill(vs::EDITOR_BG);

    let b = damascene_core::bundle::artifact::render_bundle_themed(
        &mut root,
        Rect::new(0.0, 0.0, 1040.0, 660.0),
        &theme::theme(),
    );
    for f in &b.lint.findings {
        eprintln!("{f:?}");
    }
    assert!(
        b.lint.findings.is_empty(),
        "{} findings",
        b.lint.findings.len()
    );
}
