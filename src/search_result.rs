use std::iter::Sum;
use std::ops::Add;
use std::ops::Div;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use simbelmyne_chess::board::Board;

use crate::{diff::{BFactor, Nodes, Nps, Score, Time}, fields::{Extract, Fields}};

////////////////////////////////////////////////////////////////////////////////
///
/// SearchResult
///
////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SearchResult {
    pub position: String,
    pub depth: usize,
    pub nodes: Nodes,
    pub time: Time,
    pub nps: Nps,
    pub score: Score,
    pub branching_factor: BFactor,
}

impl SearchResult {
    pub fn new(board: Board, nodes: u32, time: u64, score: i32, depth: usize) -> Self {
        let nps = nodes / time as u32;
        let branching_factor = f32::powf(nodes as f32, 1.0 / depth as f32);

        Self {
            position: board.to_fen(),
            depth,
            nodes: Nodes(nodes),
            time: Time(time),
            nps: Nps(nps),
            branching_factor: BFactor(branching_factor),
            score: Score(score),
        }
    }
}

impl Extract for SearchResult {
    fn extract(&self, fields: &Fields) -> Vec<String> {
        let mut values = Vec::new();

        values.push(format!("{}", self.position.to_string().blue()));

        if fields.nodes {
            values.push(self.nodes.to_string())
        }

        if fields.time {
            values.push(self.time.to_string())
        }

        if fields.nps {
            values.push(self.nps.to_string())
        }

        if fields.branching {
            values.push(self.branching_factor.to_string())
        }

        if fields.score {
            values.push(self.score.to_string())
        }


        values
    }
}

impl Add for SearchResult {
    type Output = SearchResult;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            position: String::new(),
            depth: self.depth,
            nodes: self.nodes + rhs.nodes,
            time: self.time + rhs.time,
            nps: self.nps + rhs.nps,
            score: self.score + rhs.score,
            branching_factor: self.branching_factor + rhs.branching_factor,
        }
    }
}

impl Div<usize> for SearchResult {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            position: self.position,
            depth: self.depth,
            nodes: self.nodes / rhs,
            time: self.time / rhs,
            nps: self.nps / rhs,
            score: self.score / rhs,
            branching_factor: self.branching_factor / rhs,
        }
    }
}

impl Sum for SearchResult {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, val| acc + val)
    }
}
