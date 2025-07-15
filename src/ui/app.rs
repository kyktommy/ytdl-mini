use gpui::{
    div, prelude::*, px, rgb, SharedString, Render, IntoElement, Window, Context,
    MouseButton, MouseDownEvent, FocusHandle, Focusable, actions, App as GpuiApp,
    KeyBinding, Keystroke, ClipboardItem, EntityInputHandler, UTF16Selection,
    KeyDownEvent, Modifiers,
};

use crate::core::{AppState, DownloadItem, DownloadStatus};

// Define actions for the app
actions!(ytdl_mini, [
    AddDownload,
    ShowSettings,
    ToggleSettings,
    Backspace,
    SubmitUrl,
]);

/// Main application struct
pub struct App {
    app_state: AppState,
    url_input: SharedString,
    show_settings: bool,
    focus_handle: FocusHandle,
    input_focused: bool,
}

impl App {
    /// Create a new app instance
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            app_state: AppState::new(),
            url_input: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".into(), // Demo URL - users can clear and type new ones
            show_settings: false,
            focus_handle: cx.focus_handle(),
            input_focused: false,
        }
    }

    /// Handle adding a download
    fn handle_add_download(&mut self, _event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let url = self.url_input.to_string();
        if !url.trim().is_empty() {
            if crate::utils::is_valid_youtube_url(&url) {
                // For now, just clear the input and show a message
                log::info!("Adding download for: {}", url);
                self.url_input = "".into();
                cx.notify();
            } else {
                log::warn!("Invalid YouTube URL: {}", url);
            }
        }
    }

    /// Handle toggling settings
    fn handle_toggle_settings(&mut self, _event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.show_settings = !self.show_settings;
        cx.notify();
    }

    /// Handle URL input click
    fn handle_url_input_click(&mut self, _event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.input_focused = true;
        cx.notify();
    }

    /// Handle backspace action
    fn handle_backspace(&mut self, _action: &Backspace, _window: &mut Window, cx: &mut Context<Self>) {
        let mut url = self.url_input.to_string();
        url.pop();
        self.url_input = url.into();
        cx.notify();
    }

    /// Handle submit URL action (Enter key)
    fn handle_submit_url(&mut self, _action: &SubmitUrl, _window: &mut Window, cx: &mut Context<Self>) {
        let url = self.url_input.to_string();
        if !url.trim().is_empty() {
            if crate::utils::is_valid_youtube_url(&url) {
                log::info!("Adding download for: {}", url);
                self.url_input = "".into();
                cx.notify();
            } else {
                log::warn!("Invalid YouTube URL: {}", url);
            }
        }
    }

    /// Handle text input (for typing characters)
    fn handle_text_input(&mut self, text: &str, cx: &mut Context<Self>) {
        let mut url = self.url_input.to_string();
        url.push_str(text);
        self.url_input = url.into();
        cx.notify();
    }

    /// Handle clear input button
    fn handle_clear_input(&mut self, _event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.url_input = "".into();
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
            .child(
                div()
                    .flex()
                    .flex_1()
                    .p_3()
                    .bg(rgb(0x1e1e1e))
                    .border_1()
                    .border_color(rgb(0x404040))
                    .rounded_lg()
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_url_input_click))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex_1()
                                    .text_color(if self.url_input.is_empty() {
                                        rgb(0x888888)
                                    } else {
                                        rgb(0xcccccc)
                                    })
                                    .child(if self.url_input.is_empty() {
                                        "Enter YouTube URL here...".to_string()
                                    } else {
                                        self.url_input.to_string()
                                    })
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(rgb(0x404040))
                                    .border_1()
                                    .border_color(rgb(0x606060))
                                    .rounded_md()
                                    .text_color(rgb(0xffffff))
                                    .text_xs()
                                    .hover(|style| style.bg(rgb(0x606060)))
                                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_clear_input))
                                    .child("Clear")
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x0066cc))
                    .border_1()
                    .border_color(rgb(0x0088ff))
                    .rounded_lg()
                    .text_color(rgb(0xffffff))
                    .hover(|style| style.bg(rgb(0x0088ff)))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_add_download))
                    .child("Download")
            )
            .child(
                div()
                    .flex()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x404040))
                    .border_1()
                    .border_color(rgb(0x606060))
                    .rounded_lg()
                    .text_color(rgb(0xffffff))
                    .hover(|style| style.bg(rgb(0x606060)))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_toggle_settings))
                    .child("Settings")
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
                    .child(
                        div()
                            .flex_1()
                            .child("Status")
                    )
                    .child(
                        div()
                            .flex()
                            .w(px(300.0))
                            .child("Title")
                    )
                    .child(
                        div()
                            .flex()
                            .w(px(200.0))
                            .child("Created")
                    )
            )
            .child(
                // Table content
                div()
                    .flex()
                    .flex_col()
                    .child(
                        // Empty state
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .p_8()
                            .text_color(rgb(0x888888))
                            .child(
                                div()
                                    .text_xl()
                                    .mb_2()
                                    .child("📥")
                            )
                            .child(
                                div()
                                    .mb_1()
                                    .child("No downloads yet")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .child("Add a YouTube URL above to start downloading")
                            )
                    )
            )
    }

    /// Render the settings panel
    fn render_settings(&self) -> impl IntoElement {
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
                            .child("Settings")
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
                                    .child("Default Resolution:")
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
                                            .child("1920x1080 (1080p)")
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
                                            .child("1280x720 (720p)")
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
                                            .child("2560x1440 (1440p)")
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
                                            .child("3840x2160 (4K)")
                                    )
                            )
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
                                    .child("Download Path:")
                            )
                            .child(
                                div()
                                    .p_2()
                                    .bg(rgb(0x2d2d2d))
                                    .border_1()
                                    .border_color(rgb(0x404040))
                                    .rounded_lg()
                                    .text_color(rgb(0xffffff))
                                    .child("~/Downloads/ytdl-mini")
                            )
                    )
            )
    }
}

impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1a1a1a))
            .relative()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::handle_backspace))
            .on_action(cx.listener(Self::handle_submit_url))
            .child(
                // Header with URL input
                self.render_url_input(cx)
            )
            .child(
                // Main content area
                div()
                    .flex()
                    .flex_1()
                    .child(self.render_downloads_table())
            )
            .child(
                // Settings overlay
                self.render_settings()
            )
    }
}

impl Focusable for App {
    fn focus_handle(&self, _cx: &GpuiApp) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EntityInputHandler for App {
    fn text_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
        _actual_range: &mut Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        Some(self.url_input.to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.url_input.len()..self.url_input.len(),
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<std::ops::Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        // No-op for now
    }

    fn replace_text_in_range(
        &mut self,
        _range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.url_input = SharedString::from(new_text.to_string());
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        _new_selected_range_utf16: Option<std::ops::Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.url_input = SharedString::from(new_text.to_string());
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
        _bounds: gpui::Bounds<gpui::Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<gpui::Bounds<gpui::Pixels>> {
        None
    }

    fn character_index_for_point(
        &mut self,
        _point: gpui::Point<gpui::Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.url_input.len())
    }
}
