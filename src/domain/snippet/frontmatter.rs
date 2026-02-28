use serde::Deserialize;

/// Strips the YAML front matter from `content` and returns the body.
/// If no valid front matter exists, returns `content` unchanged.
pub fn strip_frontmatter(content: &str) -> &str {
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return content;
    }

    let open_len = if content.starts_with("---\r\n") { 5 } else { 4 };
    let after_open = &content[open_len..];

    if let Some(close_pos) = find_closing_fence(after_open) {
        let fence_line_len = 4; // "---\n"
        &after_open[close_pos + fence_line_len..]
    } else {
        content
    }
}

/// Returns the YAML string inside front matter, or `None` if absent / malformed.
pub fn parse_frontmatter(content: &str) -> Option<&str> {
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return None;
    }

    let open_len = if content.starts_with("---\r\n") { 5 } else { 4 };
    let after_open = &content[open_len..];
    let close_pos = find_closing_fence(after_open)?;
    Some(&after_open[..close_pos])
}

fn find_closing_fence(text: &str) -> Option<usize> {
    let mut pos = 0;
    for line in text.lines() {
        if line == "---" {
            return Some(pos);
        }
        pos += line.len() + 1; // +1 for '\n'
    }
    None
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SnippetFrontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub aliases: Option<Vec<String>>,
}

pub fn parse_frontmatter_metadata(content: &str) -> Option<SnippetFrontmatter> {
    let yaml = parse_frontmatter(content)?;
    serde_yaml::from_str(yaml).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_frontmatter_returning_body() {
        let content = "---\ntitle: Hello\n---\nbody content\n";
        assert_eq!(strip_frontmatter(content), "body content\n");
    }

    #[test]
    fn no_frontmatter_returns_input_unchanged() {
        let content = "just a body\n";
        assert_eq!(strip_frontmatter(content), "just a body\n");
    }

    #[test]
    fn unclosed_frontmatter_returns_input_unchanged() {
        let content = "---\ntitle: Hello\nbody without close";
        assert_eq!(strip_frontmatter(content), content);
    }

    #[test]
    fn empty_body_after_frontmatter() {
        let content = "---\ntitle: T\n---\n";
        assert_eq!(strip_frontmatter(content), "");
    }

    #[test]
    fn only_fence_markers() {
        let content = "---\n---\n";
        assert_eq!(strip_frontmatter(content), "");
    }

    #[test]
    fn parse_frontmatter_returns_yaml_string() {
        let content = "---\ntitle: My Title\ndescription: My desc\n---\nbody\n";
        let yaml = parse_frontmatter(content).unwrap();
        assert!(yaml.contains("title: My Title"));
    }

    #[test]
    fn parse_frontmatter_none_when_absent() {
        assert!(parse_frontmatter("no frontmatter").is_none());
    }

    #[test]
    fn parse_frontmatter_metadata_deserializes_fields() {
        let content = "---\ntitle: My Title\ndescription: My desc\n---\nbody\n";
        let fm = parse_frontmatter_metadata(content).unwrap();
        assert_eq!(fm.title.as_deref(), Some("My Title"));
        assert_eq!(fm.description.as_deref(), Some("My desc"));
    }

    #[test]
    fn parse_frontmatter_metadata_handles_missing_fields() {
        let content = "---\ntitle: Only Title\n---\nbody\n";
        let fm = parse_frontmatter_metadata(content).unwrap();
        assert_eq!(fm.title.as_deref(), Some("Only Title"));
        assert!(fm.description.is_none());
    }

    #[test]
    fn parse_frontmatter_metadata_none_when_no_frontmatter() {
        assert!(parse_frontmatter_metadata("no frontmatter").is_none());
    }
}
