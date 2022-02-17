use actix_web::{
    http::Method,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Route,
};
use tera::{Context, Tera};

use crate::AppData;

const XML_CONTENT_TYPE: &str = "application/xml; charset=utf-8";

pub fn propfind_method() -> Method {
    Method::from_bytes(b"PROPFIND").unwrap()
}

fn propfind_route() -> Route {
    web::method(propfind_method())
}

fn build_response(rendered: String) -> HttpResponse {
    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(rendered)
}

fn render_template(tera: &Tera, ctx: &Context, file_name: &str) -> String {
    tera.render(file_name, ctx).unwrap()
}

async fn carddav(_req: HttpRequest, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = render_template(&data.templates, &ctx, "root.xml.tera");

    build_response(rendered)
}

async fn principal(_req: HttpRequest, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = render_template(&data.templates, &ctx, "principal.xml.tera");

    build_response(rendered)
}

async fn addressbooks(_req: HttpRequest, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = render_template(&data.templates, &ctx, "addressbooks.xml.tera");

    build_response(rendered)
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
