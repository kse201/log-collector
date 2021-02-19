use actix_web::http::Method;
use actix_web::App;

mod handlers;

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

// Write Routing
pub fn app(server: Server) -> App<Server> {
    use crate::handlers::*;

    let app: App<Server> = App::with_state(server)
        .route("/logs", Method::POST, handle_post_logs)
        .route("/logs", Method::GET, handle_get_logs)
        .route("/csv", Method::POST, handle_post_csv)
        .route("/csv", Method::GET, handle_get_csv);

    app
}

fn main() {
    env_logger::init();

    let server = Server::new();
    ::actix_web::server::new(move || app(server.clone()))
        .bind("localhost:3000")
        .expect("can not bind to port 3000")
        .run();
}
