use std::fmt::Display;

use serde::{Deserialize, Serialize};
use simbelmyne_chess::{board::Board, movegen::moves::Move};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub position: String,
    pub depth: usize,
    pub best_move: String,
    pub nodes: u32,
    pub time: u64,
    pub branching_factor: f32,
    pub nps: u32,
}

impl SearchResult {
    pub fn new(board: Board, best_move: Move, nodes: u32, time: u64, depth: usize) -> Self {
        let nps = nodes / time as u32;
        let branching_factor = f32::powf(nodes as f32, 1.0 / depth as f32);

        Self {
            position: board.to_fen(),
            depth,
            best_move: best_move.to_string(),
            nodes,
            time,
            branching_factor,
            nps,
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<72}:", self.position)?;
        write!(f, "{:>12} nodes", self.nodes)?;
        write!(f, "{:>6}ms", self.time)?;
        write!(f, "{:>8} knps", self.nps)?;
        write!(f, "{:>6.2}", self.branching_factor)?;
        write!(f, "{:>6}", self.best_move)?;

        Ok(())
    }
}
