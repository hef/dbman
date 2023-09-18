use actix_web::{HttpServer, App, middleware::Logger, web::{self, Data}, get, HttpRequest, HttpResponse, Responder};
use clap::Parser;
use controller::State;
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    
}

#[get("/healthz")]
async fn healthz(_c: Data<State>, _req: HttpRequest ) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[get("/readyz")]
async fn readyz(_c: Data<State>, _req: HttpRequest ) -> impl Responder {
    HttpResponse::Ok().json("ready")
}

#[get("/livez")]
async fn livez(_c: Data<State>, _req: HttpRequest ) -> impl Responder {
    HttpResponse::Ok().json("live")
}

#[get("/")]
async fn index(c: Data<State>, _req: HttpRequest ) -> impl Responder {
    let d = c.diagnostics().await;
    HttpResponse::Ok().json(&d)
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
            .wrap(Logger::default().exclude("/healthz"))
            .service(index)
            .service(healthz)
            .service(readyz)
            .service(livez)
    })
    .bind("0.0.0.0:8080")?;

    tokio::join!(controller, server.run()).1?;
    Ok(())

}
