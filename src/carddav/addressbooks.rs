use simple_xml_builder::XMLElement;

const STATUS_OK: &str = "HTTP/1.1 200 OK";
// const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not Found";

const XMLNS_D: (&str, &str) = ("xmlns:d", "DAV:");
const XMLNS_CARD: (&str, &str) = ("xmlns:card", "urn:ietf:params:xml:ns:carddav");

pub fn build_addressbooks_response() -> String {
    let root = elem_with_children(
        elem_with_attrs("d:multistatus", &[XMLNS_D, XMLNS_CARD]),
        vec![elem_with_children(
            elem("d:response"),
            vec![
                text_node("d:href", "/"),
                elem_with_children(
                    elem("d:propstat"),
                    vec![
                        elem_with_children(
                            elem("d:prop"),
                            vec![elem_with_children(
                                elem("d:current-user-principal"),
                                vec![text_node(
                                    "d:href",
                                    "/carddav/principals/users/test@example.org/",
                                )],
                            )],
                        ),
                        text_node("d:status", STATUS_OK),
                    ],
                ),
            ],
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
    use super::*;

    #[test]
    fn test_build_addressbooks_response() {
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
        assert_eq!(expected, build_addressbooks_response())
    }
}
