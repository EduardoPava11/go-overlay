use wry::{
    application::event_loop::EventLoop,
    application::window::WindowBuilder,
    webview::WebViewBuilder,
};

pub fn start_gui(port: u16) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Go Overlay")
        .with_transparent(true)
        .with_decorations(false)
        .with_inner_size(wry::application::dpi::LogicalSize::new(540.0, 540.0))
        .build(&event_loop)
        .unwrap();

    let index_url = format!("http://localhost:{}/index.html", port);
    let _webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url(&index_url)
        .unwrap()
        .with_transparent(true)
        .build()
        .unwrap();

    event_loop.run(move |_event, _, _| {});
}