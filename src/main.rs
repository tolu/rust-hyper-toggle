use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tower::ServiceBuilder;

// import our logger middleware
mod logger;
use logger::Logger;

// import api request handler
mod api;
use api::toggle;

// Hook up the Service into a running hyper server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Create service from request-handelr function
            let svc = service_fn(toggle);
            // Create a logger that uses the request service
            let svc = ServiceBuilder::new().layer_fn(Logger::new).service(svc);

            // Finally, we bind the incoming connection to our service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, svc)
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
