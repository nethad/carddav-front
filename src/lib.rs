use actix_web::{
    http::Method,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Route,
};

const XML_CONTENT_TYPE: &str = "application/xml; charset=utf-8";

pub fn propfind_method() -> Method {
    Method::from_bytes(b"PROPFIND").unwrap()
}

fn propfind_route() -> Route {
    web::method(propfind_method())
}

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
