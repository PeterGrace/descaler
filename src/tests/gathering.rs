use crate::status_management::fetch_url;
#[cfg(test)]
use httpmock::{
    Method::{GET, POST},
    MockServer,
};

#[tokio::test]
async fn test_send_hook() {
    let mock_server = MockServer::start();
    let search_mock = mock_server.mock(|when, then| {
        when.method(GET).path("/descaler.yaml");
        then.status(200).body(String::from("---"));
    });
    assert_eq!(
        "---",
        fetch_url(mock_server.url("/descaler.yaml").as_str())
            .await
            .unwrap()
    );
}
