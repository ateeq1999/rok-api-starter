mod common;

#[tokio::test]
#[ignore = "requires running database"]
async fn register_and_login() {
    let _app = common::TestApp::boot().await;
    // TODO: implement auth integration test
}
