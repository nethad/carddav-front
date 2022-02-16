use actix_web::{
    http::{self, Method},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Route,
};

const XML_CONTENT_TYPE: &str = "application/xml; charset=utf-8";

async fn carddav(_req: HttpRequest) -> impl Responder {
    let response = r##"
        <?xml version="1.0"?>
        <d:multistatus xmlns:d="DAV:">
            <d:response>
                <d:href>/</d:href>
                <d:propstat>
                    <d:prop>
                        <d:current-user-principal>
                            <d:href>/carddav/principals/users/user@example.org/</d:href>
                        </d:current-user-principal>
                    </d:prop>
                    <d:status>HTTP/1.1 200 OK</d:status>
                </d:propstat>
            </d:response>
        </d:multistatus>
    "##;

    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(response)
}

async fn principal(_req: HttpRequest) -> impl Responder {
    let response = r##"
        <?xml version="1.0"?>
        <d:multistatus xmlns:d="DAV:" xmlns:card="urn:ietf:params:xml:ns:carddav">
            <d:response>
                <d:href>/</d:href>
                <d:propstat>
                    <d:prop>
                        <card:addressbook-home-set>
                            <d:href>/carddav/addressbooks/users/user@example.org/</d:href>
                        </card:addressbook-home-set>
                    </d:prop>
                    <d:status>HTTP/1.1 200 OK</d:status>
                </d:propstat>
            </d:response>
        </d:multistatus>
    "##;

    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(response)
}

async fn addressbooks(_req: HttpRequest) -> impl Responder {
    let response = r##"
    <d:multistatus xmlns:d="DAV:" xmlns:s="http://sabredav.org/ns" xmlns:card="urn:ietf:params:xml:ns:carddav" xmlns:oc="http://owncloud.org/ns" xmlns:nc="http://nextcloud.org/ns">
        <d:response>
            <d:href>/carddav/addressbooks/users/user@example.org/</d:href>
            <d:propstat>
            <d:prop>
                <d:resourcetype>
                <d:collection />
                </d:resourcetype>
            </d:prop>
            <d:status>HTTP/1.1 200 OK</d:status>
            </d:propstat>
            <d:propstat>
            <d:prop>
                <d:displayname />
                <x1:getctag xmlns:x1="http://calendarserver.org/ns/" />
            </d:prop>
            <d:status>HTTP/1.1 404 Not Found</d:status>
            </d:propstat>
        </d:response>
        <d:response>
            <d:href>/carddav/addressbooks/users/user@example.org/contacts/</d:href>
            <d:propstat>
            <d:prop>
                <d:resourcetype>
                <d:collection />
                <card:addressbook />
                </d:resourcetype>
                <d:displayname>Kontakte</d:displayname>
                <x1:getctag xmlns:x1="http://calendarserver.org/ns/">92</x1:getctag>
            </d:prop>
            <d:status>HTTP/1.1 200 OK</d:status>
            </d:propstat>
        </d:response>
    </d:multistatus>
    "##;

    HttpResponse::MultiStatus()
        .content_type(XML_CONTENT_TYPE)
        .body(response)
}

fn propfind_method() -> Method {
    http::Method::from_bytes(b"PROPFIND").unwrap()
}

fn propfind_route() -> Route {
    web::method(propfind_method())
}

fn routing_configuration(cfg: &mut web::ServiceConfig) {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routing_configuration))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_index_get() {
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
}
