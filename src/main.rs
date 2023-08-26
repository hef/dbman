use clap::Parser;
use controller::State;
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[clap(long, env)]
    host: String,
    #[clap(short, long, env="POSTGRES_USER")]
    username: String,
    #[clap(short, long, env)]
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), controller::Error> {
    env_logger::init();

    let args = Args::parse();
    info!("Starting up");

    let state = State {
        conn_string: format!(
            "host={} user={} password={}",
            args.host, args.username, args.password
        ),
    };


    controller::run(state).await
}
