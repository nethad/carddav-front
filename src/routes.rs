use actix_web::{
    http::Method,
    web::{self, Bytes, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Route,
};
use tera::{Context, Tera};

use crate::{formatting::prettify_xml, AppData};

const XML_CONTENT_TYPE: &str = "application/xml; charset=utf-8";

pub fn propfind_method() -> Method {
    Method::from_bytes(b"PROPFIND").unwrap()
}

pub fn report_method() -> Method {
    Method::from_bytes(b"REPORT").unwrap()
}

pub fn propfind_route() -> Route {
    web::method(propfind_method())
}

fn dav_route(method: Method) -> Route {
    web::method(method)
}

fn build_response(rendered: String) -> HttpResponse {
    println!("~~~~~~~~~~~~~~");
    println!("response:");
    println!("{}", rendered);
    println!("~~~~~~~~~~~~~~");
    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(rendered)
}

fn render_template(tera: &Tera, ctx: &Context, file_name: &str) -> String {
    tera.render(file_name, ctx).unwrap()
}

fn string_from_request_bytes(bytes: &Bytes) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn debug_body(context: &str, bytes: &Bytes) {
    let body_string = string_from_request_bytes(bytes);
    let xml_body = prettify_xml(&body_string);
    println!("--> {}:", context);
    println!("---");
    println!("{}", xml_body);
    println!("---");
}

async fn carddav(_req: HttpRequest, data: web::Data<AppData>, bytes: Bytes) -> impl Responder {
    debug_body("carddav", &bytes);
    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = render_template(&data.templates, &ctx, "root.xml.tera");

    build_response(rendered)
}

async fn principal(_req: HttpRequest, data: web::Data<AppData>, bytes: Bytes) -> impl Responder {
    debug_body("principal", &bytes);

    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    let rendered = render_template(&data.templates, &ctx, "principal.xml.tera");

    build_response(rendered)
}

async fn addressbooks(_req: HttpRequest, data: web::Data<AppData>, bytes: Bytes) -> impl Responder {
    debug_body("addressbooks", &bytes);
    let body = string_from_request_bytes(&bytes);

    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");

    let template = if body.contains("supported") {
        "addressbooks-data-support.xml.tera"
    } else {
        "addressbooks.xml.tera"
    };

    println!("addressbooks, template --> {}", template);
    let rendered = render_template(&data.templates, &ctx, template);

    build_response(rendered)
}

async fn addressbook_data(
    req: HttpRequest,
    data: web::Data<AppData>,
    bytes: Bytes,
) -> impl Responder {
    debug_body("addressbook-data", &bytes);
    let body = string_from_request_bytes(&bytes);

    let mut ctx = Context::new();
    ctx.insert("user", "rendered@example.org");
    ctx.insert("contact_id", "d7684a02-795e-4a2e-b8ce-a805cf7c26ed");

    let template = if req.method().eq(&report_method()) {
        "addressbook-data.xml.tera"
    } else if body.contains("supported") {
        "addressbooks-data-support.xml.tera"
    } else {
        "addressbook-data-contenttype.xml.tera"
    };

    println!("addressbook_data, template --> {}", template);
    let rendered = render_template(&data.templates, &ctx, template);

    build_response(rendered)
}

async fn well_known(_req: HttpRequest) -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("Location", "/carddav")
        .finish()
}

pub fn routing_configuration(cfg: &mut ServiceConfig) {
    cfg.route("/.well-known/carddav", web::get().to(well_known))
        .route(
            "/.well-known/carddav",
            dav_route(propfind_method()).to(well_known),
        )
        .route(
            "/carddav/addressbooks/users/{user}/contacts",
            dav_route(report_method()).to(addressbook_data),
        )
        .route(
            "/carddav/addressbooks/users/{user}/contacts",
            dav_route(propfind_method()).to(addressbook_data),
        )
        .route(
            "/carddav/addressbooks/users/{user}",
            propfind_route().to(addressbooks),
        )
        .route(
            "/carddav/principals/users/{user}",
            propfind_route().to(principal),
        )
        .route("/carddav", propfind_route().to(carddav));
}
