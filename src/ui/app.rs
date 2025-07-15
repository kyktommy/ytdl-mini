use gpui::{
    actions, div, prelude::*, px, rgb, App as GpuiApp, ClickEvent, Context, Entity, FocusHandle,
    Focusable, IntoElement, Render, Subscription, Window,
};

use gpui_component::{
    button::{Button, ButtonVariants},
    input::{InputState, TextInput},
    Sizable,
};

use crate::core::{AppState, DownloadItem, DownloadStatus};

// Define actions for the app
actions!(
    ytdl_mini,
    [
        AddDownload,
        ShowSettings,
        ToggleSettings,
        Backspace,
        SubmitUrl,
    ]
);

/// Main application struct
pub struct App {
    app_state: AppState,
    url_input_state: Option<Entity<InputState>>,
    download_path_state: Option<Entity<InputState>>,
    show_settings: bool,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl App {
    /// Create a new app instance
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            app_state: AppState::new(),
            url_input_state: None,
            download_path_state: None,
            show_settings: false,
            focus_handle: cx.focus_handle(),
            _subscriptions: Vec::new(),
        }
    }

    /// Initialize the input states with window access
    pub fn init_input_states(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.url_input_state.is_none() {
            self.url_input_state = Some(cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter YouTube URL here...")
                    .default_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
            }));
        }

        if self.download_path_state.is_none() {
            self.download_path_state = Some(cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Download path...")
                    .default_value("~/Downloads/ytdl-mini")
            }));
        }
    }

    /// Handle adding a download
    fn handle_add_download(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(url_input_state) = &self.url_input_state {
            let url = url_input_state.read(cx).value();
            if !url.trim().is_empty() {
                if crate::utils::is_valid_youtube_url(&url) {
                    // For now, just clear the input and show a message
                    log::info!("Adding download for: {}", url);
                    // Clear the input
                    url_input_state.update(cx, |state, cx| {
                        state.set_value("", window, cx);
                    });
                } else {
                    log::warn!("Invalid YouTube URL: {}", url);
                }
            }
        }
    }

    /// Handle browse button click for download path
    fn handle_browse_download_path(
        &mut self,
        _: &ClickEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        // This would open a file picker dialog
        // For now, just a placeholder
        log::info!("Browse for download path");
    }

    /// Handle toggling settings
    fn handle_toggle_settings(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_settings = !self.show_settings;
        cx.notify();
    }

    /// Render the URL input section
    fn render_url_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_3()
            .p_4()
            .bg(rgb(0x2d2d2d))
            .border_b_1()
            .border_color(rgb(0x404040))
            .child(if let Some(url_input_state) = &self.url_input_state {
                TextInput::new(url_input_state)
                    .cleanable()
                    .flex_1()
                    .into_any_element()
            } else {
                div().flex_1().into_any_element()
            })
            .child(
                Button::new("download")
                    .primary()
                    .on_click(cx.listener(Self::handle_add_download))
                    .child("Download"),
            )
            .child(
                Button::new("settings")
                    .ghost()
                    .on_click(cx.listener(Self::handle_toggle_settings))
                    .child("Settings"),
            )
    }

    /// Render the downloads table
    fn render_downloads_table(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .bg(rgb(0x1e1e1e))
            .border_1()
            .border_color(rgb(0x404040))
            .rounded_lg()
            .m_4()
            .child(
                // Table header
                div()
                    .flex()
                    .flex_row()
                    .p_3()
                    .bg(rgb(0x2d2d2d))
                    .border_b_1()
                    .border_color(rgb(0x404040))
                    .text_color(rgb(0xffffff))
                    .font_weight(gpui::FontWeight::BOLD)
                    .child(div().flex_1().child("Status"))
                    .child(div().flex().w(px(300.0)).child("Title"))
                    .child(div().flex().w(px(200.0)).child("Created")),
            )
            .child(
                // Table content
                div().flex().flex_col().child(
                    // Empty state
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .p_8()
                        .text_color(rgb(0x888888))
                        .child(div().text_xl().mb_2().child("📥"))
                        .child(div().mb_1().child("No downloads yet"))
                        .child(
                            div()
                                .text_sm()
                                .child("Add a YouTube URL above to start downloading"),
                        ),
                ),
            )
    }

    /// Render the settings panel
    fn render_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.show_settings {
            return div();
        }

        div()
            .absolute()
            .top(px(0.0))
            .right(px(0.0))
            .w(px(300.0))
            .h_full()
            .bg(rgb(0x1e1e1e))
            .border_l_1()
            .border_color(rgb(0x404040))
            .shadow_lg()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xffffff))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Settings"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0xcccccc))
                                    .text_sm()
                                    .child("Default Resolution:"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x2d2d2d))
                                            .border_1()
                                            .border_color(rgb(0x0066cc))
                                            .rounded_lg()
                                            .text_color(rgb(0xffffff))
                                            .child("1920x1080 (1080p)"),
                                    )
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1e1e1e))
                                            .border_1()
                                            .border_color(rgb(0x404040))
                                            .rounded_lg()
                                            .text_color(rgb(0xcccccc))
                                            .hover(|style| style.bg(rgb(0x2d2d2d)))
                                            .child("1280x720 (720p)"),
                                    )
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1e1e1e))
                                            .border_1()
                                            .border_color(rgb(0x404040))
                                            .rounded_lg()
                                            .text_color(rgb(0xcccccc))
                                            .hover(|style| style.bg(rgb(0x2d2d2d)))
                                            .child("2560x1440 (1440p)"),
                                    )
                                    .child(
                                        div()
                                            .p_2()
                                            .bg(rgb(0x1e1e1e))
                                            .border_1()
                                            .border_color(rgb(0x404040))
                                            .rounded_lg()
                                            .text_color(rgb(0xcccccc))
                                            .hover(|style| style.bg(rgb(0x2d2d2d)))
                                            .child("3840x2160 (4K)"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0xcccccc))
                                    .text_sm()
                                    .child("Download Path:"),
                            )
                            .child(
                                if let Some(download_path_state) = &self.download_path_state {
                                    TextInput::new(download_path_state)
                                        .suffix(
                                            Button::new("browse")
                                                .ghost()
                                                .xsmall()
                                                .on_click(
                                                    cx.listener(Self::handle_browse_download_path),
                                                )
                                                .child("Browse"),
                                        )
                                        .into_any_element()
                                } else {
                                    div().into_any_element()
                                },
                            ),
                    ),
            )
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Initialize input states if they haven't been initialized yet
        self.init_input_states(window, cx);
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1a1a1a))
            .relative()
            .track_focus(&self.focus_handle)
            .child(
                // Header with URL input
                self.render_url_input(cx),
            )
            .child(
                // Main content area
                div().flex().flex_1().child(self.render_downloads_table()),
            )
            .child(
                // Settings overlay
                self.render_settings(cx),
            )
    }
}

impl Focusable for App {
    fn focus_handle(&self, _cx: &GpuiApp) -> FocusHandle {
        self.focus_handle.clone()
    }
}
