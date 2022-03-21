use simple_xml_builder::XMLElement;

const STATUS_OK: &str = "HTTP/1.1 200 OK";
// const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not Found";

const XMLNS_D: (&str, &str) = ("xmlns:d", "DAV:");
const XMLNS_CARD: (&str, &str) = ("xmlns:card", "urn:ietf:params:xml:ns:carddav");

pub fn principal_path(identifier: &str) -> String {
    format!("/carddav/principals/users/{}/", identifier)
}

pub struct PropRequest {
    props: Vec<String>,
}

pub fn parse_carddav_request(xml_content: &str) -> PropRequest {
    let doc = roxmltree::Document::parse(xml_content).unwrap();
    let root_element = doc.root_element();
    if root_element.tag_name().name() == "propfind" {
        let prop = root_element.children().find(|n| n.has_tag_name("prop"));
        match prop {
            Some(node) => {
                let props = node
                    .children()
                    .filter(|n| n.is_element())
                    .map(|n| n.tag_name().name().to_string())
                    .collect::<Vec<String>>();
                PropRequest { props }
            }
            None => PropRequest { props: vec![] },
        }
    } else {
        PropRequest { props: vec![] }
    }
}

fn build_propstat_block(prop_request: PropRequest) -> XMLElement {
    let prop_blocks = prop_request
        .props
        .iter()
        .map(|prop| match prop.as_str() {
            "current-user-principal" => Some(elem_with_children(
                elem("d:current-user-principal"),
                vec![text_node("d:href", &principal_path("test@example.org"))],
            )),
            _ => None,
        })
        .filter_map(std::convert::identity)
        .collect::<Vec<_>>();

    elem_with_children(
        elem("d:propstat"),
        vec![
            elem_with_children(elem("d:prop"), prop_blocks),
            text_node("d:status", STATUS_OK),
        ],
    )
}

pub fn build_carddav_response(prop_request: PropRequest) -> String {
    let root = elem_with_children(
        elem_with_attrs("d:multistatus", &[XMLNS_D, XMLNS_CARD]),
        vec![elem_with_children(
            elem("d:response"),
            vec![text_node("d:href", "/"), build_propstat_block(prop_request)],
        )],
    );

    root.to_string()
}

fn text_node(tag: &str, text: &str) -> XMLElement {
    let mut text_node = elem(tag);
    text_node.add_text(text);
    text_node
}

fn elem_with_attrs(tag: &str, attributes: &[(&str, &str)]) -> XMLElement {
    let mut elem = elem(tag);
    for attrs in attributes {
        add_xmlns(&mut elem, attrs);
    }
    elem
}

fn elem_with_children(mut elem: XMLElement, children: Vec<XMLElement>) -> XMLElement {
    for child in children {
        elem.add_child(child);
    }
    elem
}

fn elem(tag: &str) -> XMLElement {
    XMLElement::new(tag)
}

fn add_xmlns(elem: &mut XMLElement, xmlns: &(&str, &str)) {
    elem.add_attribute(xmlns.0, xmlns.1);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_build_carddav_response() {
        let expected = "<?xml version = \"1.0\" encoding = \"UTF-8\"?>\n\
      <d:multistatus xmlns:d=\"DAV:\" xmlns:card=\"urn:ietf:params:xml:ns:carddav\">\n\
      \t<d:response>\n\
      \t\t<d:href>/</d:href>\n\
      \t\t<d:propstat>\n\
      \t\t\t<d:prop>\n\
      \t\t\t\t<d:current-user-principal>\n\
      \t\t\t\t\t<d:href>/carddav/principals/users/test@example.org/</d:href>\n\
      \t\t\t\t</d:current-user-principal>\n\
      \t\t\t</d:prop>\n\
      \t\t\t<d:status>HTTP/1.1 200 OK</d:status>\n\
      \t\t</d:propstat>\n\
      \t</d:response>\n\
      </d:multistatus>\n";

        let prop_request = PropRequest {
            props: vec!["current-user-principal".to_string()],
        };
        assert_eq!(expected, build_carddav_response(prop_request))
    }

    #[test]
    fn test_parse_carddav_request() {
        let content = fs::read_to_string("tests/fixtures/request/current-user-principal.xml")
            .expect("Read fixture failed");

        let request = parse_carddav_request(&content);
        assert_eq!(["current-user-principal"], request.props.as_slice())
    }
}
