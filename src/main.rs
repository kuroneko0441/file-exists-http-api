use tiny_http::{Server, Method, Response, StatusCode};
use chrono::Local;
use std::{sync::Arc, sync::atomic::{AtomicBool, Ordering}};

fn log(msg: &str) {
    println!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

fn main() {
    let addr = "0.0.0.0:3000";
    let server = Arc::new(Server::http(addr).unwrap());
    log(&format!("Server started: {}", addr));

    let running = Arc::new(AtomicBool::new(true));

    {
        let srv = Arc::clone(&server);
        let running_flag = Arc::clone(&running);
        ctrlc::set_handler(move || {
            log("Received SIGINT/SIGTERM, shutting down...");
            running_flag.store(false, Ordering::SeqCst);
            srv.unblock();
        }).expect("signal handler error");
    }

    for req in server.incoming_requests() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        if req.method() != &Method::Head {
            log(&format!("{} {} -> 405", req.method(), req.url()));
            let _ = req.respond(Response::empty(StatusCode(405)));
            continue;
        }

        let path = req.url();
        let status = if std::fs::metadata(path).is_ok() {
            StatusCode(200)
        } else {
            StatusCode(404)
        };

        log(&format!("HEAD {} -> {}", path, status.0));
        let _ = req.respond(Response::empty(status));
    }

    log("Server closed.");
}
