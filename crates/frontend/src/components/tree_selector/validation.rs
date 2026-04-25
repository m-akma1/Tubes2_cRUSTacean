use css_selector::parse;

use crate::app::RunMode;

use super::lca_controls::parse_lca_index;

pub(crate) fn selector_error(run_mode: RunMode, selector_text: &str) -> Option<String> {
    if run_mode != RunMode::Traversal {
        return None;
    }

    let trimmed = selector_text.trim();
    if trimmed.is_empty() {
        Some("Masukkan CSS selector sebelum menjalankan algoritma.".to_string())
    } else {
        parse(trimmed).err().map(|error| error.to_string())
    }
}

pub(crate) fn top_n_error(
    run_mode: RunMode,
    top_n_enabled: bool,
    top_n_text: &str,
) -> Option<String> {
    if run_mode != RunMode::Traversal || !top_n_enabled {
        return None;
    }

    match top_n_text.trim().parse::<usize>() {
        Ok(value) if value > 0 => None,
        _ => Some("Top N harus berupa bilangan bulat positif.".to_string()),
    }
}

pub(crate) fn lca_error(
    run_mode: RunMode,
    node_count: usize,
    node_a_text: &str,
    node_b_text: &str,
) -> Option<String> {
    if run_mode != RunMode::Lca {
        return None;
    }

    if node_count == 0 {
        return Some("Tree kosong, LCA tidak dapat dihitung.".to_string());
    }

    let Some(a) = parse_lca_index(node_a_text) else {
        return Some("Node A harus berupa indeks bilangan bulat.".to_string());
    };
    let Some(b) = parse_lca_index(node_b_text) else {
        return Some("Node B harus berupa indeks bilangan bulat.".to_string());
    };

    if a >= node_count || b >= node_count {
        return Some(format!(
            "Node A dan Node B harus berada pada rentang 0..{}",
            node_count - 1
        ));
    }

    None
}
