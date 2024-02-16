use kuchikiki::{traits::TendrilSink, NodeRef};

#[derive(Debug)]
pub struct Story {
    pub headline: String,
    pub link: String,
    pub content: Option<String>,
}

impl Story {
    #[must_use]
    pub fn new(headline: String, link: &str) -> Story {
        let link = format!("https://www.oxfordmail.co.uk{link}");
        Story {
            headline,
            link,
            content: None,
        }
    }
    #[must_use]
    pub fn markdown(&self) -> String {
        format!("[{}]({})", self.headline, self.link)
    }

    pub fn get_page(&self) -> NodeRef {
        let response = reqwest::blocking::get(&self.link).unwrap().text().unwrap();
        let _ = std::fs::write("example.html", &response);
        kuchikiki::parse_html().one(response)
    }

    fn download_content(&self) -> Result<String, ()> {
        let document = self.get_page();
        let article_node = document.select_first(".article-body")?;

        let text_nodes = article_node.as_node().select("p").unwrap();
        let text: Vec<_> = text_nodes
            .map(|node| {
                let node_text = node
                    .as_node()
                    .first_child()
                    .unwrap()
                    .as_text()
                    .unwrap()
                    .borrow()
                    .to_string();
                node_text.trim().to_string()
            })
            .collect();
        // text.dedup();
        Ok(text.join("\n"))
    }

    pub fn get_content(&mut self) -> String {
        if self.content == None {
            self.content = Some(self.download_content().unwrap());
        }

        self.content.clone().unwrap()
    }
}
