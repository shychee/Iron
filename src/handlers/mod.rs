use crate::http::{RequestTrait, ResponseTrait};

pub async fn hello_world<Req: RequestTrait, Res: ResponseTrait>(_request: Req) -> Res {
    Res::new("Hello, World!".to_string())
}

pub async fn get_user<Req: RequestTrait, Res: ResponseTrait>(request: Req) -> Res {
    let user_id = request.params().get("id").map_or("unknown".to_string(), |s| s.to_string());
    Res::new(format!("Getting user with ID: {}", user_id))
}

pub async fn create_user<Req: RequestTrait, Res: ResponseTrait>(request: Req) -> Res {
    Res::new(format!("Creating user: {}", request.body()))
}

pub async fn update_user<Req: RequestTrait, Res: ResponseTrait>(request: Req) -> Res {
    let user_id = request.params().get("id").map_or("unknown".to_string(), |s| s.to_string());
    Res::new(format!("Updating user with ID: {}", user_id))
}

pub async fn delete_user<Req: RequestTrait, Res: ResponseTrait>(request: Req) -> Res {
    let user_id = request.params().get("id").map_or("unknown".to_string(), |s| s.to_string());
    Res::new(format!("Deleting user with ID: {}", user_id))
}