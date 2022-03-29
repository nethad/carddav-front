use simple_xml_builder::XMLElement;

const STATUS_OK: &str = "HTTP/1.1 200 OK";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not Found";

const XMLNS_D: (&str, &str) = ("xmlns:d", "DAV:");
const XMLNS_CARD: (&str, &str) = ("xmlns:card", "urn:ietf:params:xml:ns:carddav");

pub fn principal_path(identifier: &str) -> String {
    format!("/carddav/principals/users/{}/", identifier)
}

struct PropResponse {
    found: bool,
    content: XMLElement,
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

fn build_prop_responses(prop_request: PropRequest) -> Vec<PropResponse> {
    prop_request
        .props
        .iter()
        .map(|prop| match prop.as_str() {
            "current-user-principal" => PropResponse {
                found: true,
                content: elem_with_children(
                    elem("d:current-user-principal"),
                    vec![text_node("d:href", &principal_path("test@example.org"))].into_iter(),
                ),
            },
            _ => PropResponse {
                found: false,
                content: elem(prop),
            },
        })
        .collect::<Vec<_>>()
}

fn build_propstat_block(
    prop_responses: impl Iterator<Item = XMLElement>,
    found: bool,
) -> Option<XMLElement> {
    let elements = prop_responses.collect::<Vec<_>>();
    if elements.is_empty() {
        None
    } else {
        let status = if found { STATUS_OK } else { STATUS_NOT_FOUND };
        Some(elem_with_children(
            elem("d:propstat"),
            vec![
                elem_with_children(elem("d:prop"), elements.into_iter()),
                text_node("d:status", status),
            ]
            .into_iter(),
        ))
    }
}

pub fn build_carddav_response(prop_request: PropRequest) -> String {
    let prop_responses = build_prop_responses(prop_request);

    let found_props = prop_responses
        .iter()
        .filter(|prop_block| prop_block.found)
        .map(|prop_block| prop_block.content.clone());
    let found_propstats = build_propstat_block(found_props, true);
    let not_found_props = prop_responses
        .iter()
        .filter(|prop_block| !prop_block.found)
        .map(|prop_block| prop_block.content.clone());
    let not_found_propstats = build_propstat_block(not_found_props, false);

    let response_elements = vec![
        Some(text_node("d:href", "/")),
        found_propstats,
        not_found_propstats,
    ];
    let elements = response_elements
        .into_iter()
        .filter_map(|pb| pb)
        .collect::<Vec<_>>();

    let root = elem_with_children(
        elem_with_attrs("d:multistatus", &[XMLNS_D, XMLNS_CARD]),
        vec![elem_with_children(elem("d:response"), elements.into_iter())].into_iter(),
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

fn elem_with_children(
    mut elem: XMLElement,
    children: impl Iterator<Item = XMLElement>,
) -> XMLElement {
    for child in children {
        elem.add_child(child);
    }
    elem
}

// fn elem_with_children2(
//     mut elem: XMLElement,
//     children: impl Iterator<Item = XMLElement>,
// ) -> XMLElement {
//     for child in children {
//         elem.add_child(child);
//     }
//     elem
// }

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
