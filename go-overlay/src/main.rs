mod board;
mod gui;

use board::{Board, Stone};
use hyper::{service::{make_service_fn, service_fn}, Body, Request, Response, Server, StatusCode};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use std::thread;

static BOARD: once_cell::sync::Lazy<Arc<Mutex<Board>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(Board::new())));

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Same handle_request logic as before
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");
    if path == "/click" {
        let mut row = None;
        let mut col = None;
        for param in query.split('&') {
            if param.starts_with("row=") { row = param[4..].parse::<usize>().ok(); }
            if param.starts_with("col=") { col = param[4..].parse::<usize>().ok(); }
        }
        if let (Some(r), Some(c)) = (row, col) {
            let mut board = BOARD.lock().unwrap();
            board.place_stone(r, c, Stone::Black);
            println!("Placed black stone at row {}, col {}", r, c);
            board.print_board();
            Ok(Response::new(Body::from("OK")))
        } else {
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Missing row or col"))
                .unwrap())
        }
    } else if path.ends_with("index.html") {
        let html = std::fs::read_to_string("src/static/index.html").unwrap();
        Ok(Response::new(Body::from(html)))
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("404 Not Found"))
            .unwrap())
    }
}

fn main() {
    let port = 3030;

    // Start server on a separate thread using tokio runtime
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle_request)) });
            let addr = ([127, 0, 0, 1], port).into();
            let server = Server::bind(&addr).serve(make_svc);

            println!("HTTP server running on http://localhost:{}", port);
            server.await.unwrap();
        });
    });

    // GUI runs on main thread (this fixes the error!)
    gui::start_gui(port);
}
