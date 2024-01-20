use clap::command;
use clap::Parser;
pub use controller::Database;
pub use controller::DatabaseServer;
use kube::CustomResourceExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    for_helm: bool,
}

fn main() {
    let args = Args::parse();

    if args.for_helm {
        print!(
            "{{- if .Values.crds.enabled }}\n{}\n---\n{}\n{{- end }}",
            serde_yaml::to_string(&controller::Database::crd()).unwrap(),
            serde_yaml::to_string(&controller::DatabaseServer::crd()).unwrap()
        )
    } else {
        print!(
            "{}\n---\n{}",
            serde_yaml::to_string(&controller::Database::crd()).unwrap(),
            serde_yaml::to_string(&controller::DatabaseServer::crd()).unwrap()
        )
    }
}
