use crate::domain::context_file::validate_path;
use crate::domain::error::AppError;
use crate::ports::{Clipboard, ContextFileStore, SnippetCatalog};
use std::borrow::Cow;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CopyOutcome {
    pub snippet: String,
    pub relative_path: String,
    pub absolute_path: std::path::PathBuf,
}

pub fn execute(
    snippet: &str,
    catalog: &dyn SnippetCatalog,
    clipboard: &dyn Clipboard,
    workspace_store: Option<&dyn ContextFileStore>,
) -> Result<CopyOutcome, AppError> {
    let snippet_entry = catalog.resolve_snippet(snippet)?;
    let content = fs::read_to_string(&snippet_entry.absolute_path)?;
    let expanded = expand_placeholders(&content, workspace_store);
    clipboard.copy(expanded.as_ref())?;

    Ok(CopyOutcome {
        snippet: snippet_entry.key,
        relative_path: snippet_entry.relative_path,
        absolute_path: snippet_entry.absolute_path,
    })
}

fn expand_placeholders<'a>(
    content: &'a str,
    workspace_store: Option<&dyn ContextFileStore>,
) -> Cow<'a, str> {
    let Some(store) = workspace_store else {
        return Cow::Borrowed(content);
    };

    if !content.contains("{{") {
        return Cow::Borrowed(content);
    }

    let mut remainder = content;
    let mut output = String::with_capacity(content.len());

    while let Some(start) = remainder.find("{{") {
        output.push_str(&remainder[..start]);
        let tail = &remainder[start + 2..];

        match tail.find("}}") {
            Some(end) => {
                let token = &tail[..end];
                output.push_str(&render_placeholder(token, store));
                remainder = &tail[end + 2..];
            }
            None => {
                output.push_str(&remainder[start..]);
                return Cow::Owned(output);
            }
        }
    }

    output.push_str(remainder);
    Cow::Owned(output)
}

fn render_placeholder(raw_token: &str, workspace_store: &dyn ContextFileStore) -> String {
    let trimmed = raw_token.trim();
    if trimmed.is_empty() {
        return format!("{{{{{raw_token}}}}}");
    }

    if let Err(err) = validate_path(trimmed, Path::new(trimmed)) {
        return format!("[mx error: {}]", err);
    }

    match workspace_store.read_workspace_file(Path::new(trimmed)) {
        Ok(contents) => contents,
        Err(err) => format!("[mx missing: {trimmed} ({})]", err.kind()),
    }
}
