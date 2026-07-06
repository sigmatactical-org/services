//! Load service descriptions from the embedded `content/` tree.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use include_dir::{Dir, include_dir};
use pulldown_cmark::{Options, Parser, html};

static CONTENT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content");

#[derive(Debug, Clone)]
pub struct ServiceEntry {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub order: i32,
    pub body_html: String,
}

static SERVICES: OnceLock<BTreeMap<String, ServiceEntry>> = OnceLock::new();

pub fn services() -> &'static BTreeMap<String, ServiceEntry> {
    SERVICES.get_or_init(load_services)
}

pub fn sorted_entries() -> Vec<&'static ServiceEntry> {
    let mut entries: Vec<_> = services().values().collect();
    entries.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.title.cmp(&b.title)));
    entries
}

pub fn get(slug: &str) -> Option<&'static ServiceEntry> {
    services().get(slug)
}

fn load_services() -> BTreeMap<String, ServiceEntry> {
    let mut map = BTreeMap::new();
    for file in CONTENT.files() {
        let Some(path) = file.path().to_str() else {
            continue;
        };
        if !path.ends_with(".md") {
            continue;
        }
        let slug = path.trim_end_matches(".md").to_string();
        let source = file.contents_utf8().unwrap_or("");
        let (meta, markdown) = split_front_matter(source);
        let title = meta
            .get("title")
            .cloned()
            .unwrap_or_else(|| slug.replace('-', " "));
        let summary = meta.get("summary").cloned().unwrap_or_default();
        let order = meta
            .get("order")
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);
        let body_html = markdown_to_html(markdown);
        map.insert(
            slug.clone(),
            ServiceEntry {
                slug,
                title,
                summary,
                order,
                body_html,
            },
        );
    }
    map
}

fn split_front_matter(source: &str) -> (BTreeMap<String, String>, &str) {
    let mut meta = BTreeMap::new();
    let Some(rest) = source.strip_prefix("---") else {
        return (meta, source);
    };
    let Some((header, body)) = rest.split_once("\n---") else {
        return (meta, source);
    };
    let body = body.trim_start_matches('\n');
    for line in header.lines() {
        if let Some((key, value)) = line.split_once(':') {
            meta.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    (meta, body)
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html_out = String::new();
    let parser = Parser::new_ext(
        markdown,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    );
    html::push_html(&mut html_out, parser);
    html_out
}
