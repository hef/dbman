use clap::command;
use clap::Parser;
use controller::v1alpha1;
use controller::v1alpha2;
use controller::v1alpha3;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
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
        println!("{{- if .Values.crds.enabled }}")
    }

    let crds: Vec<CustomResourceDefinition> = vec![
        // old
        v1alpha1::DatabaseServer::crd(),
        v1alpha2::Database::crd(),
        // current
        v1alpha3::Database::crd(),
        v1alpha2::DatabaseServer::crd(),
    ];
    crds.iter().for_each(|crd| {
        println!(
            "{}\n---\n",
            serde_yaml::to_string(&crd).expect("failed to serialize crd")
        )
    });

    if args.for_helm {
        println!("{{- end }}")
    }
}
