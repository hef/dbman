use clap::Parser;
use controller::State;
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    
}

#[tokio::main]
async fn main() -> Result<(), controller::Error> {
    env_logger::init();

    let _args = Args::parse();
    info!("Starting up");

    let state = State {
        ..Default::default()
    };

    controller::run(state).await
}
