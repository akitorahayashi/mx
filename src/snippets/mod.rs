mod catalog_entry;
mod filesystem_catalog;
mod filesystem_store;
mod frontmatter;
mod query;
mod snippet_catalog;
mod snippet_checkout;
mod snippet_store;
mod symlink_checkout;

#[cfg(test)]
mod in_memory_catalog;
#[cfg(test)]
mod in_memory_checkout;
#[cfg(test)]
mod in_memory_snippet_store;
#[cfg(test)]
pub mod test_support;

pub use catalog_entry::SnippetEntry;
pub use filesystem_catalog::FilesystemSnippetCatalog;
pub use filesystem_store::FilesystemSnippetStore;
pub use frontmatter::{
    parse_frontmatter, parse_frontmatter_metadata, strip_frontmatter, SnippetFrontmatter,
};
pub use query::{candidate_key, normalize_query, path_to_string};
pub use snippet_catalog::SnippetCatalog;
pub use snippet_checkout::{CheckoutStatus, SnippetCheckout};
pub use snippet_store::SnippetStore;
pub use symlink_checkout::SymlinkCheckout;
