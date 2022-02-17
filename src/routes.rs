use actix_web::{
    http::Method,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Route,
};
use tera::Context;

use crate::AppData;

const XML_CONTENT_TYPE: &str = "application/xml; charset=utf-8";

pub fn propfind_method() -> Method {
    Method::from_bytes(b"PROPFIND").unwrap()
}

fn propfind_route() -> Route {
    web::method(propfind_method())
}

async fn carddav(_req: HttpRequest, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = data.templates.render("root.xml.tera", &ctx).unwrap();

    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(rendered)
}

async fn principal(_req: HttpRequest, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = data.templates.render("principal.xml.tera", &ctx).unwrap();

    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(rendered)
}

async fn addressbooks(_req: HttpRequest, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = data
        .templates
        .render("addressbooks.xml.tera", &ctx)
        .unwrap();

    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(rendered)
}

pub fn routing_configuration(cfg: &mut ServiceConfig) {
    cfg.route(
        "/carddav/addressbooks/users/{user}/",
        propfind_route().to(addressbooks),
    )
    .route(
        "/carddav/principals/users/{user}/",
        propfind_route().to(principal),
    )
    .route("/carddav", propfind_route().to(carddav));
}
