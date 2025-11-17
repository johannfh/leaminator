#[macro_use]
extern crate rocket;

mod ws {
    #[get("/commands")]
    pub fn commands(ws: rocket_ws::WebSocket) -> &'static str {
        _ = ws;
        "Commands endpoint"
    }
}

mod api {
    #[get("/ping")]
    pub fn ping() -> &'static str {
        "pong"
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // initialize rocket
    let r = rocket::build();

    // mount http routes for static files (release only)
    #[cfg(not(debug_assertions))]
    let r = r.mount("/", rocket::fs::FileServer::from("static/"));

    let r = r.mount("/api", routes![api::ping]);

    // mount websocket server routes
    let r = r.mount("/ws", routes![ws::commands]);

    // launch the server
    let _r = r.launch().await?;

    Ok(())
}
