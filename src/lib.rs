mod md_utils;

pub fn to_html(markdown: &str) -> String {
    md_utils::to_html(markdown)
}
