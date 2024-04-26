use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::body::Frame;
use hyper::{body::Body, Method, Request, Response, StatusCode};

// Create a request handling function, hooked up in main.rs to a service in a running hyper server
pub async fn toggle(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    // Match on incoming request method and path
    match (req.method(), req.uri().path()) {
        // Match GET /
        (&Method::GET, "/") => Ok(Response::new(full("Hello, Toggle! on slash "))),
        // Match POST /uppercase
        (&Method::POST, "/echo/uppercase") => {
            // Map this body's frame to a different type
            let frame_stream = req.into_body().map_frame(|frame| {
                let frame = if let Ok(data) = frame.into_data() {
                    // Convert every byte in every Data frame to uppercase
                    data.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Bytes>()
                } else {
                    Bytes::new()
                };

                Frame::data(frame)
            });

            Ok(Response::new(frame_stream.boxed()))
        }
        // Match POST /reversed
        (&Method::POST, "/echo/reversed") => {
            // To protect our server, reject requests with bodies larger than
            // 64kbs of data.
            let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);
            if upper > 1024 * 64 {
                let mut resp = Response::new(full("Body too big"));
                *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
                return Ok(resp);
            }

            // Await the whole body to be collected into a single `Bytes`...
            let whole_body = req.collect().await?.to_bytes();

            // Iterate the whole body in reverse order and collect into a new Vector
            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();

            Ok(Response::new(full(reversed_body)))
        }
        // Match POST /echo
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),
        // Default matcher: 404
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// We create some utility functions to make Empty and Full bodies
// fit our broadened Response body type.
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
