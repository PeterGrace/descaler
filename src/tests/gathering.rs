#[cfg(test)]
use httpmock::{Mock, MockServer, Method::{GET, POST}};


#[tokio::test]
async fn test_send_hook() {
    let mock_server = MockServer::start();
    let search_mock = mock_server.mock(|when, then| {
        when.method(GET)
            .path("/mockpost");
        then.status(200)
        .body(String::from("---"));

    });

    // let response = send_hook(
    //     mock_server.url("/mockpost").to_string(),
    //     String::from("POST"),
    //     String::from("{}"),
    // )
    //     .await
    //     .unwrap();
    //assert_eq!(response.status(), 200);
    //assert_eq!(search_mock.hits(), 1);
}