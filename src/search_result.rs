use std::fmt::Display;

use serde::{Deserialize, Serialize};
use simbelmyne_chess::{board::Board, movegen::moves::Move};

use crate::diff::{BFactor, Nodes, Nps, Score, Time};

////////////////////////////////////////////////////////////////////////////////
///
/// SearchResult
///
////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub position: String,
    pub depth: usize,
    pub best_move: String,
    pub nodes: Nodes,
    pub time: Time,
    pub nps: Nps,
    pub score: Score,
    pub branching_factor: BFactor,
}

impl SearchResult {
    pub fn new(board: Board, best_move: Move, nodes: u32, time: u64, score: i32, depth: usize) -> Self {
        let nps = nodes / time as u32;
        let branching_factor = f32::powf(nodes as f32, 1.0 / depth as f32);

        Self {
            position: board.to_fen(),
            depth,
            best_move: best_move.to_string(),
            nodes: Nodes(nodes),
            time: Time(time),
            nps: Nps(nps),
            branching_factor: BFactor(branching_factor),
            score: Score(score),
        }
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<72}:", self.position)?;
        write!(f, "{}", self.nodes)?;
        write!(f, "{}", self.time)?;
        write!(f, "{}", self.nps)?;
        write!(f, "{}", self.branching_factor)?;
        write!(f, "{:>6}", self.best_move)?;

        Ok(())
    }
}
