use druid::{AppLauncher, WindowDesc};

use super::data;
use data::AppState;

use super::view;
use view::build_ui;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Ember editor")
        .window_size((400.0, 400.0));

    let initial_state = AppState { body: "pub struct Repl {}".into()};

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
