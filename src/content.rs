//! Load service descriptions from the embedded `content/` tree.

mod service_entry;
pub use service_entry::ServiceEntry;

use std::collections::BTreeMap;
use std::sync::OnceLock;

use include_dir::{Dir, include_dir};
use sigma_theme::content::{markdown_to_html, split_front_matter};

static CONTENT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content");

static SERVICES: OnceLock<BTreeMap<String, ServiceEntry>> = OnceLock::new();

/// The static service registry, keyed by slug.
pub fn services() -> &'static BTreeMap<String, ServiceEntry> {
    SERVICES.get_or_init(load_services)
}

/// Registry entries in display order.
pub fn sorted_entries() -> Vec<&'static ServiceEntry> {
    let mut entries: Vec<_> = services().values().collect();
    entries.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.title.cmp(&b.title)));
    entries
}

/// Look up a service by slug.
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
