use kuchikiki::{traits::TendrilSink, NodeRef};

use crate::story::Story;

#[must_use]
pub fn home_page_html() -> NodeRef {
    // let html = std::fs::read_to_string("mail.html").expect("Oops no file");
    let resp = reqwest::blocking::get("https://oxfordmail.co.uk").unwrap();
    kuchikiki::parse_html().one(resp.text().unwrap())
}

#[must_use]
pub fn article_links(document: &NodeRef) -> Vec<Story> {
    let css_selector = ".top-stories .omnicard__headline";

    document
        .select(css_selector)
        .unwrap()
        .map(|css_match| {
            let as_node = css_match.as_node();
            let text_node = as_node.first_child().unwrap();
            let text = text_node.as_text().unwrap().borrow();
            let text = text.trim();

            let link_node = as_node.parent().unwrap();
            let link_attrs = link_node.as_element().unwrap().attributes.borrow();
            let link = link_attrs.get("href").unwrap();

            Story::new(text.to_string(), link)
        })
        .collect()
}
