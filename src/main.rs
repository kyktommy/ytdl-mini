mod core;
mod ui;
mod utils;

use log::info;
use gpui::{App, Application, Bounds, size, px, WindowOptions, WindowBounds, AppContext, KeyBinding};

fn main() {
    // Initialize logging
    env_logger::init();

    info!("Starting ytdl-mini application");

    // Create and run the GPUI application
    Application::new().run(|cx: &mut App| {
        // Set up key bindings for text input
        cx.bind_keys([
            KeyBinding::new("backspace", ui::Backspace, None),
            KeyBinding::new("enter", ui::SubmitUrl, None),
        ]);

        let bounds = Bounds::centered(None, size(px(800.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| ui::App::new(cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
