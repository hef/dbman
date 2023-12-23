mod condition;

use actix_web::{
    get, middleware::Logger, web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use controller::State;
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    owner: String,
}

#[get("/healthz")]
async fn healthz(_c: Data<State>, _req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[get("/readyz")]
async fn readyz(_c: Data<State>, _req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("ready")
}

#[get("/")]
async fn index(c: Data<State>, _req: HttpRequest) -> impl Responder {
    let d = c.diagnostics();
    match d {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let _args = Args::parse();
    info!("Starting up");

    let state = State::default();

    let controller = controller::run(state.clone());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(Logger::default().exclude("/healthz").exclude("/readyz"))
            .service(index)
            .service(healthz)
            .service(readyz)
    })
    .bind("0.0.0.0:8080")?;

    tokio::join!(controller, server.run()).1?;
    Ok(())
}
