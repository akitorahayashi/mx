mod catalog_entry;
mod filesystem_catalog;
mod frontmatter;
mod query;
mod snippet_catalog;

#[cfg(test)]
mod in_memory_catalog;
#[cfg(test)]
pub use in_memory_catalog::InMemoryCatalog;

pub use catalog_entry::SnippetEntry;
pub use filesystem_catalog::FilesystemSnippetCatalog;
pub use frontmatter::{
    parse_frontmatter, parse_frontmatter_metadata, strip_frontmatter, SnippetFrontmatter,
};
pub use query::{candidate_key, normalize_query, path_to_string};
pub use snippet_catalog::SnippetCatalog;
