mod core;
mod ui;
mod utils;

use gpui::{
    px, size, App, AppContext, Application, Bounds, KeyBinding, TitlebarOptions, WindowBounds,
    WindowOptions,
};
use log::info;

fn main() {
    // Initialize logging
    env_logger::init();

    info!("Starting ytdl-mini application");

    // Create and run the GPUI application
    Application::new().run(|cx: &mut App| {
        // Initialize gpui-component
        gpui_component::init(cx);

        // Set up key bindings for text input
        cx.bind_keys([
            KeyBinding::new("backspace", ui::Backspace, None),
            KeyBinding::new("enter", ui::SubmitUrl, None),
        ]);

        let bounds = Bounds::centered(None, size(px(800.), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("ytdl-mini".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                // Create the app view
                let app_view = cx.new(|cx| ui::App::new(cx));

                // Wrap it in a Root component
                cx.new(|cx| gpui_component::Root::new(app_view.into(), window, cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
