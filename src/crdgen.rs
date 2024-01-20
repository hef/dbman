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
        let pre = "{{- if .Values.crds.enabled }}";
        let post = "{{- end }}";
        print!(
            "{pre}\n{}\n{post}\n---\n{pre}\n{}\n{post}",
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
