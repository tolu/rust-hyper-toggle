use hyper::header::USER_AGENT;
use hyper::{body::Incoming, service::Service, Request};

#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S,
}
impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}
type Req = Request<Incoming>;

impl<S> Service<Req> for Logger<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    fn call(&self, req: Req) -> Self::Future {
        println!(
            "\nprocessing request: {} {}\n{}",
            req.method(),
            req.uri().path(),
            format!("{}{:?}", "User-Agent: ", req.headers()[USER_AGENT])
        );
        self.inner.call(req)
    }
}
