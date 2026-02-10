use std::sync::atomic::{AtomicU64, Ordering};
use url::Url;

static NEXT_MARKDOWN_ROOT_ID: AtomicU64 = AtomicU64::new(1);

pub fn next_markdown_root_id() -> String {
    let id = NEXT_MARKDOWN_ROOT_ID.fetch_add(1, Ordering::Relaxed);
    format!("common-markdown-{id}")
}

pub fn resolve_href(raw: &str, link_base_url: &str) -> String {
    resolve_with_base(raw, link_base_url, true)
}

pub fn resolve_src(raw: &str, image_base_url: &str, link_base_url: &str) -> String {
    if !image_base_url.is_empty() {
        return resolve_with_base(raw, image_base_url, false);
    }
    resolve_with_base(raw, link_base_url, false)
}

fn resolve_with_base(raw: &str, base: &str, allow_anchor: bool) -> String {
    if raw.is_empty() || base.is_empty() {
        return raw.to_string();
    }

    if is_absolute(raw) {
        return raw.to_string();
    }

    if raw.starts_with('#') && !allow_anchor {
        return raw.to_string();
    }

    let base_url = match Url::parse(base) {
        Ok(url) => url,
        Err(_) => return raw.to_string(),
    };

    match base_url.join(raw) {
        Ok(url) => url.into(),
        Err(_) => raw.to_string(),
    }
}

fn is_absolute(raw: &str) -> bool {
    raw.starts_with("//") || Url::parse(raw).is_ok()
}
