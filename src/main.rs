use Iron::{Router, http::{RequestTrait, ResponseTrait, HttpMethod, Request, Response}, server};
use Iron::router::RouterTrait;
use Iron::handlers::{hello_world, get_user, create_user, update_user, delete_user};
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut router = Router::<Request, Response>::new();

    // Add global logging middleware
    router.add_middleware(|request| async move {
        info!("Received request: {:?} {}", request.method(), request.path());
        Ok(request)
    });

    // Add public routes
    router.add_route(HttpMethod::GET, "/", hello_world);

    // Create an authenticated group
    let auth_group = router.group("/api");
    
    // Add authentication middleware to the group
    auth_group.add_middleware(|request| async move {
        if let Some(_auth_header) = request.headers().get("Authorization") {
            // TODO: Perform authentication logic here
            Ok(request)
        } else {
            let mut response = Response::new("Unauthorized");
            response.set_status(401);
            Err(response)
        }
    });

    // Add authenticated routes
    auth_group.add_route(HttpMethod::GET, "/users/:id", get_user);
    auth_group.add_route(HttpMethod::POST, "/users", create_user);
    auth_group.add_route(HttpMethod::PUT, "/users/:id", update_user);
    auth_group.add_route(HttpMethod::DELETE, "/users/:id", delete_user);

    // Add another public route
    router.add_route(HttpMethod::GET, "/files/*path", |request| async move {
        Response::new(format!("Accessing file: {}", request.params().get("path").unwrap_or(&"unknown".to_string())))
    });

    server::run(router, "127.0.0.1:7878").await;
}