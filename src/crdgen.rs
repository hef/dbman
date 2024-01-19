pub use controller::Database;
pub use controller::DatabaseServer;
use kube::CustomResourceExt;

fn main() {
    print!(
        "{}\n---\n{}",
        serde_yaml::to_string(&controller::Database::crd()).unwrap(),
        serde_yaml::to_string(&controller::DatabaseServer::crd()).unwrap()
    )
}
