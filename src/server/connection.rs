use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, error};
use crate::router::{Router, RouterTrait};
use crate::http::{RequestTrait, ResponseTrait};

pub(crate) async fn handle_connection<Req: RequestTrait, Res: ResponseTrait>(
    mut stream: TcpStream,
    router: Arc<Router<Req, Res>>
) where
    Router<Req, Res>: RouterTrait<Req, Res>
{
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(_) => {
            let request_str = String::from_utf8_lossy(&buffer[..]);
            let request = Req::parse(&request_str);

            info!("Received request: {:?} {}", request.method(), request.path());

            let response = router.handle_request(request).await;

            if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
                error!("Failed to write to stream: {}", e);
            }
            if let Err(e) = stream.flush().await {
                error!("Failed to flush stream: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to read from stream: {}", e);
        }
    }
}