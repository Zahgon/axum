use std::net::SocketAddr;

use actix_web::{
    http::{header, StatusCode},
    test, App,
};
use example_testing::configure;
use serde_json::{json, Value};

#[actix_web::test]
async fn hello_world() {
    // `App` is turned into a `Service` we can drive directly, no need to run an
    // HTTP server.
    let app = test::init_service(App::new().configure(configure)).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let response = test::call_service(&app, req).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = test::read_body(response).await;
    assert_eq!(&body[..], b"Hello, World!");
}

#[actix_web::test]
async fn json() {
    let app = test::init_service(App::new().configure(configure)).await;

    let req = test::TestRequest::post()
        .uri("/json")
        .insert_header((header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref()))
        .set_payload(serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap())
        .to_request();
    let response = test::call_service(&app, req).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body: Value = test::read_body_json(response).await;
    assert_eq!(body, json!({ "data": [1, 2, 3, 4] }));
}

#[actix_web::test]
async fn not_found() {
    let app = test::init_service(App::new().configure(configure)).await;

    let req = test::TestRequest::get().uri("/does-not-exist").to_request();
    let response = test::call_service(&app, req).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = test::read_body(response).await;
    assert!(body.is_empty());
}

// You can also spawn a server and talk to it like any other HTTP server:
#[actix_web::test]
async fn the_real_deal() {
    let srv = actix_test::start(|| App::new().configure(configure));

    let mut response = srv.get("/").send().await.unwrap();

    let body = response.body().await.unwrap();
    assert_eq!(&body[..], b"Hello, World!");
}

// `call_service` borrows the service, so the same app can serve multiple
// requests without cloning it.
#[actix_web::test]
async fn multiple_request() {
    let app = test::init_service(App::new().configure(configure)).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let response = test::call_service(&app, req).await;
    assert_eq!(response.status(), StatusCode::OK);

    let req = test::TestRequest::get().uri("/").to_request();
    let response = test::call_service(&app, req).await;
    assert_eq!(response.status(), StatusCode::OK);
}

// Here we're calling `/requires-connect-info` which needs the peer address.
//
// That is normally provided by the connection itself, but a `TestRequest` is
// not backed by a socket. The solution is instead to set the peer address on
// the request during tests. The name is kept from the original suite so the
// test case maps one-to-one across the migration.
#[actix_web::test]
async fn with_into_make_service_with_connect_info() {
    let app = test::init_service(App::new().configure(configure)).await;

    let req = test::TestRequest::get()
        .uri("/requires-connect-info")
        .peer_addr(SocketAddr::from(([0, 0, 0, 0], 3000)))
        .to_request();
    let response = test::call_service(&app, req).await;

    assert_eq!(response.status(), StatusCode::OK);
}
