//! Thread-safe visit / match recording for parallel traversals.

use shared::{AlgorithmKind, AlgorithmResult, TraversalStep};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

/// Shared state for parallel BFS/DFS: step log, match list, visit count, and early-exit flag.
pub(crate) struct Recorder {
    matched_count: AtomicUsize,
    visited: AtomicUsize,
    steps: Mutex<Vec<TraversalStep>>,
    target: Option<usize>,
    done: AtomicBool,
    next_step: AtomicUsize,
}

impl Recorder {
    /// Creates a recorder; `top_n` caps how many matches are kept and triggers [`Self::is_done`].
    pub(crate) fn new(top_n: Option<usize>) -> Self {
        Self {
            matched_count: AtomicUsize::new(0),
            visited: AtomicUsize::new(0),
            steps: Mutex::new(Vec::new()),
            target: top_n,
            done: AtomicBool::new(false),
            next_step: AtomicUsize::new(0),
        }
    }

    /// Returns whether traversal should stop (enough matches or explicit shutdown).
    pub(crate) fn is_done(&self) -> bool {
        self.done.load(Ordering::Relaxed)
    }

    /// Records one visited node. Step indices are unique but not ordered by traversal order.
    pub(crate) fn record_visit(
        &self,
        node_index: usize,
        from_index: Option<usize>,
        is_match: bool,
    ) {
        let step_no = self.next_step.fetch_add(1, Ordering::Relaxed);
        self.visited.fetch_add(1, Ordering::Relaxed);
        let mut guard = self.steps.lock().expect("steps mutex poisoned");
        guard.push(TraversalStep {
            step: step_no,
            node_index,
            from_index,
            is_match,
        });
        drop(guard);

        if is_match {
            if let Some(cap) = self.target {
                let count = self.matched_count.fetch_add(1, Ordering::Relaxed) + 1;
                if count >= cap {
                    self.done.store(true, Ordering::Relaxed);
                }
            } else {
                self.matched_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    /// Consumes the recorder and builds a sorted [`AlgorithmResult`] for the given algorithm.
    pub(crate) fn into_algorithm_result(
        self,
        kind: AlgorithmKind,
        top_n: Option<usize>,
        t0: Instant,
    ) -> AlgorithmResult {
        let mut steps = self.steps.into_inner().expect("steps mutex poisoned");
        steps.sort_by_key(|s| s.step);
        let mut matched: Vec<usize> = steps
            .iter()
            .filter(|step| step.is_match)
            .map(|step| step.node_index)
            .collect();
        if let Some(cap) = top_n {
            matched.truncate(cap);
        }
        let visited = self.visited.load(Ordering::Relaxed);
        AlgorithmResult {
            algorithm: kind,
            matched_indices: matched,
            visited_count: visited,
            steps,
            duration_ms: t0.elapsed().as_secs_f64() * 1000.0,
            top_n,
        }
    }
}
