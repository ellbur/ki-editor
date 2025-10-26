use super::{DiagnosticStyles, HighlightName, Theme, UiStyles};
use crate::{
    style::{fg, Style},
    themes::{GitGutterStyles, SyntaxStyles},
};
use my_proc_macros::hex;

pub fn very_dark() -> Theme {
    Theme {
        name: "Very Dark".to_string(),
        syntax: SyntaxStyles::new({
            use HighlightName::*;
            &[
                (Variable, fg(hex!("#99dddd"))),
                (Keyword, fg(hex!("#9999dd"))),
                (KeywordModifier, fg(hex!("#99bbdd"))),
                (Function, fg(hex!("#99dd99"))),
                (Type, fg(hex!("#dd99dd"))),
                (TypeBuiltin, fg(hex!("#bb99dd"))),
                (String, fg(hex!("#bbddcc"))),
                (Comment, fg(hex!("#8fbcbc"))),
                (Tag, fg(hex!("#ecc6c6"))),
                (TagAttribute, fg(hex!("#ffdd99"))),
            ]
        }),
        ui: UiStyles {
            global_title: Style::new()
                .foreground_color(hex!("#cccccc"))
                .background_color(hex!("#000000")),
            window_title_focused: Style::new()
                .foreground_color(hex!("#333333"))
                .background_color(hex!("#111111")),
            window_title_unfocused: Style::new()
                .foreground_color(hex!("#aaaaaa"))
                .background_color(hex!("#111111")),
            parent_lines_background: hex!("#332233"),
            section_divider_background: hex!("#222255"),
            jump_mark_odd: Style::new()
                .background_color(hex!("#000099"))
                .foreground_color(hex!("#ffffff")),
            jump_mark_even: Style::new()
                .background_color(hex!("#009900"))
                .foreground_color(hex!("#ffffff")),
            background_color: hex!("#000000"),
            text_foreground: hex!("#cccccc"),
            primary_selection_background: hex!("#333333"),
            primary_selection_anchor_background: hex!("#333366"),
            primary_selection_secondary_cursor: Style::new()
                .background_color(hex!("#808080"))
                .foreground_color(hex!("#aaaaaa")),
            secondary_selection_background: hex!("#222222"),
            secondary_selection_anchor_background: hex!("#222255"),
            secondary_selection_primary_cursor: Style::new()
                .background_color(hex!("#552222"))
                .foreground_color(hex!("#dddddd")),
            secondary_selection_secondary_cursor: Style::new()
                .background_color(hex!("#777777"))
                .foreground_color(hex!("#cccccc")),
            line_number: Style::new().foreground_color(hex!("#666666")),
            border: Style::new()
                .background_color(hex!("#224422"))
                .foreground_color(hex!("#777777")),
            mark: Style::new().background_color(hex!("#990000")),
            possible_selection_background: hex!("#990033"),
            keymap_hint: Style::new().underline(hex!("#003366")),
            keymap_key: Style::new().bold().foreground_color(hex!("#4d2500")),
            keymap_arrow: Style::new().foreground_color(hex!("#808080")),
            fuzzy_matched_char: Style::new().foreground_color(hex!("#336699")),
        },
        diagnostic: DiagnosticStyles::default(),
        hunk: super::HunkStyles::dark(),
        git_gutter: GitGutterStyles::new(),
    }
}
