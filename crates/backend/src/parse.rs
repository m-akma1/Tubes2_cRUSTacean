use html_parser::{ParseError, ParseOptions};
use shared::{DomTree, ParseHtmlRequest, TreeStats};

use crate::error::ApiError;

pub fn map_parse_error(error: ParseError) -> ApiError {
    match error {
        ParseError::EmptyInput => ApiError::BadRequest(error.to_string()),
        ParseError::UnmatchedEnd { .. }
        | ParseError::InvalidStructure(_)
        | ParseError::Tokenizer(_) => ApiError::Unprocessable(error.to_string()),
    }
}

pub fn build_tree_stats(tree: &DomTree) -> TreeStats {
    let edge_count = tree.nodes.iter().map(|node| node.children.len()).sum();

    TreeStats {
        node_count: tree.node_count(),
        edge_count,
        max_depth: tree.max_depth(),
    }
}

pub fn options_from_query(strict: Option<bool>, include_doctype: Option<bool>) -> ParseOptions {
    ParseOptions {
        strict: strict.unwrap_or(false),
        include_doctype: include_doctype.unwrap_or(true),
    }
}

pub fn options_from_payload(payload: &ParseHtmlRequest) -> ParseOptions {
    let options = payload.options.clone().unwrap_or_default();
    ParseOptions {
        strict: options.strict,
        include_doctype: options.include_doctype,
    }
}