mod common;





// this just sets up the k8s env.
// run this test first to avoid timeouts
#[tokio::test]
async fn test_prepare() {
    let client: kube::Client = common::get_kube_client().await;
    // setup cpng needs to come before install crds, as crds waits for cpng's crds to be ready
    common::setup_cnpg(&client).await;
    common::instal_crds(&client).await;
}
