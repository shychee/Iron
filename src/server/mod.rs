mod connection;

use std::sync::Arc;
use tokio::net::TcpListener;
use log::{info, error};
use crate::router::{Router, RouterTrait};
use crate::http::{RequestTrait, ResponseTrait};

pub async fn run<Req, Res>(router: Router<Req, Res>, addr: &str)
where
    Req: RequestTrait + 'static,
    Res: ResponseTrait + Send + 'static,
    Router<Req, Res>: RouterTrait<Req, Res>
{
    let router = Arc::new(router);
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let router = router.clone();
                tokio::spawn(async move {
                    connection::handle_connection(stream, router).await;
                });
            }
            Err(e) => {
                error!("Error accepting connection: {}", e);
            }
        }
    }
}