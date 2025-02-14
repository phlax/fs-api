use crate::config::Config;
use crate::path::FSPath;
use crate::resolver::Resolver;
use hyper::{Body, Request, Response, Server, header::CONTENT_TYPE};
use hyper::service::{make_service_fn, service_fn};
use log::{error, info};
use std::convert::Infallible;
use std::sync::Arc;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct HTTP;

impl HTTP {
    // Incoming HTTP request handler
    async fn request(req: Request<Body>, resolver: Arc<Resolver>) -> Result<Response<Body>, Infallible> {
        match resolver.resolve(&req).await {
            Some(FSPath::API(api)) => {
                let json = api.json().await;
                let response = Response::builder()
                    .header(CONTENT_TYPE, api.content_type())
                    .body(Body::from(json))
                    .unwrap();
                Ok(response)
            }
            Some(FSPath::File(file)) => {
                let content = file.read().await.unwrap_or_else(|_| Vec::new());
                let response = Response::builder()
                    .header(CONTENT_TYPE, file.mimetype())
                    .body(Body::from(content))
                    .unwrap();
                Ok(response)
            }
            Some(FSPath::Directory(directory)) => {
                let body = directory.listing().await;
                let response = Response::builder()
                    .header(CONTENT_TYPE, directory.content_type())
                    .body(Body::from(body))
                    .unwrap();
                Ok(response)
            }
            None => {
                Ok(Response::builder()
                    .status(404)
                    .body(Body::from("Not Found"))
                    .unwrap())
            }
        }
    }

    pub async fn server(listen: SocketAddr, config: Config) {
        let config = Arc::new(config);
        let resolver = Arc::new(Resolver::new(Arc::clone(&config)));
        let make_svc = make_service_fn(move |_conn| {
            let resolver = Arc::clone(&resolver);
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    HTTP::request(req, Arc::clone(&resolver))
                }))
            }
        });
        let server = Server::bind(&listen).serve(make_svc);
        info!("Listening on http://{}", listen);
        if let Err(e) = server.await {
            error!("Server error: {}", e);
        }
    }
}
