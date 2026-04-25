//! Error types returned when HTML cannot be turned into a [`shared::DomTree`].

use serde::{Deserialize, Serialize};
use std::fmt;

/// Failure while parsing HTML into a DOM tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseError {
    /// The input string contained no meaningful characters (only whitespace).
    EmptyInput,
    /// The tokenizer or IO layer reported a fatal problem.
    Tokenizer(
        /// Human-readable tokenizer message.
        String,
    ),
    /// An end tag did not match any open element (only in strict mode).
    UnmatchedEnd {
        /// Tag name from the closing token (ASCII-preserving).
        tag: String,
    },
    /// Generic structural problem (strict mode or internal invariant).
    InvalidStructure(
        /// Explanation suitable for logs or UI.
        String,
    ),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "HTML input is empty"),
            ParseError::Tokenizer(s) => write!(f, "tokenizer error: {s}"),
            ParseError::UnmatchedEnd { tag } => write!(f, "unmatched end tag </{tag}>"),
            ParseError::InvalidStructure(s) => write!(f, "invalid HTML structure: {s}"),
        }
    }
}

impl std::error::Error for ParseError {}
