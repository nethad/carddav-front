use std::fs;

use actix_web::{test, App};
use carddav_front::{
    routes::{propfind_method, report_method, routing_configuration},
    setup_app_data,
};

#[actix_rt::test]
async fn test_carddav() {
    let mut app = test::init_service(
        App::new()
            .configure(routing_configuration)
            .data(setup_app_data()),
    )
    .await;

    let request_payload =
        fs::read_to_string("tests/fixtures/request/current-user-principal.xml").unwrap();
    let req = test::TestRequest::with_uri("/carddav")
        .method(propfind_method())
        .set_payload(request_payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(207, resp.status().as_u16());
    assert_eq!(
        "application/xml; charset=utf-8",
        resp.headers().get("content-type").unwrap()
    );

    let expected_response = fs::read_to_string("tests/fixtures/carddav_response.xml").unwrap();
    let body = test::read_body(resp).await;
    assert_eq!(actix_web::web::Bytes::from(expected_response), body)
}

#[actix_rt::test]
async fn test_principal() {
    let mut app = test::init_service(
        App::new()
            .configure(routing_configuration)
            .data(setup_app_data()),
    )
    .await;

    let request_payload =
        fs::read_to_string("tests/fixtures/request/addressbook-home-set.xml").unwrap();

    let req = test::TestRequest::with_uri("/carddav/principals/users/rendered@example.org")
        .method(propfind_method())
        .set_payload(request_payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(207, resp.status().as_u16());
    assert_eq!(
        "application/xml; charset=utf-8",
        resp.headers().get("content-type").unwrap()
    );

    let expected_response = fs::read_to_string("tests/fixtures/principal_response.xml").unwrap();
    let body = test::read_body(resp).await;
    assert_eq!(actix_web::web::Bytes::from(expected_response), body)
}

#[actix_rt::test]
async fn test_addressbooks_data_support() {
    let mut app = test::init_service(
        App::new()
            .configure(routing_configuration)
            .data(setup_app_data()),
    )
    .await;

    let request_payload =
        fs::read_to_string("tests/fixtures/request/addressbooks-supported-address-data.xml")
            .unwrap();
    let req = test::TestRequest::with_uri("/carddav/addressbooks/users/user@example.org")
        .method(propfind_method())
        .set_payload(request_payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    assert!(resp.status().is_success());
    assert_eq!(207, resp.status().as_u16());
    assert_eq!(
        "application/xml; charset=utf-8",
        resp.headers().get("content-type").unwrap()
    );

    let expected_response =
        fs::read_to_string("tests/fixtures/addressbooks_supported_address_data.xml").unwrap();
    let body = test::read_body(resp).await;
    assert_eq!(actix_web::web::Bytes::from(expected_response), body)
}

#[actix_rt::test]
async fn test_addressbooks_resourcetype_displayname() {
    let mut app = test::init_service(
        App::new()
            .configure(routing_configuration)
            .data(setup_app_data()),
    )
    .await;

    let request_payload =
        fs::read_to_string("tests/fixtures/request/addressbooks-resourcetype-displayname.xml")
            .unwrap();
    let req = test::TestRequest::with_uri("/carddav/addressbooks/users/user@example.org")
        .method(propfind_method())
        .set_payload(request_payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    println!("{:#?}", resp);
    assert!(resp.status().is_success());
    assert_eq!(207, resp.status().as_u16());
    assert_eq!(
        "application/xml; charset=utf-8",
        resp.headers().get("content-type").unwrap()
    );

    let expected_response =
        fs::read_to_string("tests/fixtures/addressbooks_resourcetype_displayname.xml").unwrap();
    let body = test::read_body(resp).await;
    assert_eq!(actix_web::web::Bytes::from(expected_response), body)
}

#[actix_rt::test]
async fn test_addressbooks_getcontenttype_getetag() {
    let mut app = test::init_service(
        App::new()
            .configure(routing_configuration)
            .data(setup_app_data()),
    )
    .await;

    let request_payload =
        fs::read_to_string("tests/fixtures/request/contacts-getcontenttype-getetag.xml").unwrap();
    let req = test::TestRequest::with_uri("/carddav/addressbooks/users/user@example.org/contacts")
        .method(propfind_method())
        .set_payload(request_payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(207, resp.status().as_u16());
    assert_eq!(
        "application/xml; charset=utf-8",
        resp.headers().get("content-type").unwrap()
    );

    let expected_response =
        fs::read_to_string("tests/fixtures/contacts_getcontenttype_getetag.xml").unwrap();
    let body = test::read_body(resp).await;
    assert_eq!(actix_web::web::Bytes::from(expected_response), body)
}

#[actix_rt::test]
async fn test_addressbooks_getetag_addressdata() {
    let mut app = test::init_service(
        App::new()
            .configure(routing_configuration)
            .data(setup_app_data()),
    )
    .await;

    let request_payload =
        fs::read_to_string("tests/fixtures/request/contacts-getetag-addressdata.xml").unwrap();
    let req = test::TestRequest::with_uri("/carddav/addressbooks/users/user@example.org/contacts")
        .method(report_method())
        .set_payload(request_payload)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(207, resp.status().as_u16());
    assert_eq!(
        "application/xml; charset=utf-8",
        resp.headers().get("content-type").unwrap()
    );

    let expected_response =
        fs::read_to_string("tests/fixtures/contacts_getetag_addressdata.xml").unwrap();
    let body = test::read_body(resp).await;
    assert_eq!(actix_web::web::Bytes::from(expected_response), body)
}
