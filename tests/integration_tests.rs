use actix_web::{test, App};
use carddav_front::{propfind_method, routing_configuration};

#[actix_rt::test]
async fn test_carddav() {
    let mut app = test::init_service(App::new().configure(routing_configuration)).await;
    let req = test::TestRequest::with_uri("/carddav")
        .method(propfind_method())
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(207, resp.status().as_u16());
    assert_eq!(
        "application/xml; charset=utf-8",
        resp.headers().get("content-type").unwrap()
    );
}
