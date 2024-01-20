use clap::command;
use clap::Parser;
use controller::v1alpha1;
use controller::v1alpha2;
use controller::v1alpha3;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion;
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
        // escape `{` as `{{`.
        // go template syntax is `{{- if .Values.crds.enabled }}`
        println!("{{{{- if .Values.crds.enabled }}}}");
    }
    let mut database = v1alpha3::Database::crd();

    let versions = vec![
        v1alpha3::Database::crd().spec.versions[0].clone(),
        v1alpha2::Database::crd().spec.versions[0].clone(),
    ];
    database.spec.versions = versions;

    let mut database_server = v1alpha2::DatabaseServer::crd();
    let versions = vec![
        v1alpha2::DatabaseServer::crd().spec.versions[0].clone(),
        v1alpha1::DatabaseServer::crd().spec.versions[0].clone(),
    ];
    database_server.spec.versions = versions;

    println!("---");
    let crds: Vec<CustomResourceDefinition> = vec![
        // old
        //v1alpha1::DatabaseServer::crd(),
        //v1alpha2::Database::crd(),
        // current
        //v1alpha3::Database::crd(),
        //v1alpha2::DatabaseServer::crd(),
        database,
        database_server,
    ];
    crds.iter().for_each(|crd| {
        print!(
            "{}\n---\n",
            serde_yaml::to_string(&crd).expect("failed to serialize crd")
        )
    });

    if args.for_helm {
        println!("{{{{- end }}}}")
    }
}
