use druid::WidgetExt;
use druid::{widget::Label, widget::TextBox, Widget, Lens};

use super::data::*;

pub fn build_ui() -> impl Widget<AppState> {
    TextBox::new()
        // .with_placeholder("pub struct Repl")
        // .expand_width()
        .lens(AppState::body)
}
