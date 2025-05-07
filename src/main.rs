use tiny_http::{Header, Method, Response, Server, StatusCode};
use chrono::Local;
use std::{sync::Arc, sync::atomic::{AtomicBool, Ordering}};

fn log(msg: &str) {
    println!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

fn cors_headers() -> Vec<Header> {
    vec![
        Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap(),
        Header::from_bytes("Access-Control-Allow-Methods", "HEAD, OPTIONS").unwrap(),
        Header::from_bytes("Access-Control-Allow-Headers", "*").unwrap(),
    ]
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


        match *req.method() {
            Method::Head => {
                let path = req.url();
                let status = if std::fs::metadata(path).is_ok() {
                    StatusCode(200)
                } else {
                    StatusCode(404)
                };
                log(&format!("HEAD {} -> {}", path, status.0));
                let mut res = Response::empty(status);
                for h in cors_headers() {
                    res.add_header(h);
                }
                let _ = req.respond(res);
            }
            Method::Options => {
                log(&format!("OPTIONS {} -> 204", req.url()));
                let mut res = Response::empty(StatusCode(204));
                for h in cors_headers() {
                    res.add_header(h);
                }
                let _ = req.respond(res);
            }
            _ => {
                log(&format!("{} {} -> 405", req.method(), req.url()));
                let mut res = Response::empty(StatusCode(405));
                for h in cors_headers() {
                    res.add_header(h);
                }
                let _ = req.respond(res);
            }
        }
    }

    log("Server closed.");
}
