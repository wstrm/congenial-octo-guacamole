use hyper::service::{make_service_fn, service_fn};
use hyper::{StatusCode, Method, Body, Request, Response, Server};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, Error>;

async fn page_index(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Welcome!")))
}

async fn page_hello(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello, world!")))
}

async fn page_not_found(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::builder()
       .status(StatusCode::NOT_FOUND)
       .body(Body::from("Dunno."))
       .unwrap())
}

async fn mux(
    req: Request<Body>,
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => page_index(req).await,
        (&Method::GET, "/hello") => page_hello(req).await,
        _ => page_not_found(req).await
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let service = make_service_fn(|_conn| async {
        Ok::<_, Error>(service_fn(move |req| {
            mux(req)
        }))
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
