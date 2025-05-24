use build_warren::index_manager::get_st_highest_index;
use mockito::Server;

#[test]
fn test_get_highest_index() {
    let mut server = Server::new();
    // Set up the mock server
    let _mock = server
        .mock("GET", "/build/")
        .with_status(200)
        .with_header("content-type", "text/html")
        .with_body(
            r#"
            <html>
            <body>
            <tbody><tr><td><a href="/build/193844/">Build 193844</a></td></tr></tbody>
            </body>
            </html>
            "#,
        )
        .create();

    // Call the function and assert the result
    let highest_index = get_st_highest_index();
    assert_eq!(
        highest_index, 193844,
        "Expected highest index to be 193844, got {}",
        highest_index
    );
}
