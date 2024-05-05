use std::fmt::Display;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Div;
use colored::Color;
use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;
use crate::search_result::SearchResult;

////////////////////////////////////////////////////////////////////////////////
/// 
/// Diff
///
////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct Diff {
    pub position: String,
    pub depth: usize,
    pub nodes: NodeDiff,
    pub time: TimeDiff,
    pub nps: NpsDiff,
    pub score: ScoreDiff,
    pub branching_factor: BFactorDiff,
}

impl Diff {
    pub fn new(first: &SearchResult, second: &SearchResult) -> Self {
        Self {
            position: first.position.clone(),
            depth: first.depth,
            nodes: NodeDiff::new(first.nodes, second.nodes),
            time: TimeDiff::new(first.time, second.time),
            nps: NpsDiff::new(first.nps, second.nps),
            score: ScoreDiff::new(first.score, second.score),
            branching_factor: BFactorDiff::new(first.branching_factor, second.branching_factor)
        }
    }
}

impl Add for Diff {
    type Output = Diff;

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

impl Div<usize> for Diff {
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

impl Sum for Diff {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, val| acc + val)
    }
}

////////////////////////////////////////////////////////////////////////////////
/// 
/// Nodes
///
////////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Eq, Serialize, Deserialize, Copy, Clone, Default)]
pub struct Nodes(pub u32);

impl PartialOrd for Nodes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl Ord for Nodes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl Add for Nodes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Nodes {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u32)
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} nodes", self.0)
    }
}

#[derive(Default)]
pub struct NodeDiff {
    first: Nodes,
    second: Nodes,
    relative: f32,
}

impl Add for NodeDiff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
            relative: self.relative + rhs.relative,
        }
    }
}

impl Div<usize> for NodeDiff {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            first: self.first / rhs,
            second: self.second / rhs,
            relative: self.relative / rhs as f32,
        }
    }
}

impl NodeDiff {
    pub fn new(first: Nodes, second: Nodes) -> Self {

        let relative = (second.0 as f32 - first.0 as f32) / first.0 as f32;
        Self { first, second, relative }
    }
}

impl Display for NodeDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: Custom definition of >/< !!!
        let color = if self.second > self.first {
            Color::Green
        } else if self.second < self.first {
            Color::Red
        } else {
            Color::Black
        };

        let first = format!("{}", self.first).color(Color::Black);
        let second = format!("{}", self.second).color(color);
        let relative = format!(
            "({})", 
            format!("{:>+.2}%", 100.0 * self.relative).color(color)
        );

        write!(f, "{:>15} {:>15} {:>20}", first, second, relative)
    }
}

////////////////////////////////////////////////////////////////////////////////
/// 
/// Time
///
////////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Eq, Serialize, Deserialize, Copy, Clone, Default)]
pub struct Time(pub u64);

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ms", self.0)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Time {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u64)
    }
}

#[derive(Default)]
pub struct TimeDiff {
    first: Time,
    second: Time,
    relative: f32,
}

impl TimeDiff {
    pub fn new(first: Time, second: Time) -> Self {
        let relative = (second.0 as f32 - first.0 as f32) / first.0 as f32;
        Self { first, second, relative }
    }
}

impl Display for TimeDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: Custom definition of >/< !!!
        let color = if self.second > self.first {
            Color::Green
        } else if self.second < self.first {
            Color::Red
        } else {
            Color::Black
        };

        let first = format!("{}", self.first).color(Color::Black);
        let second = format!("{}", self.second).color(color);
        let relative = format!(
            "({})", 
            format!("{:>+.2}%", 100.0 * self.relative).color(color)
        );

        write!(f, "{:>7} {:>7} {:>20}", first, second, relative)
    }
}

impl Add for TimeDiff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
            relative: self.relative + rhs.relative,
        }
    }
}

impl Div<usize> for TimeDiff {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            first: self.first / rhs,
            second: self.second / rhs,
            relative: self.relative / rhs as f32,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
/// 
/// Nps
///
////////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Copy, Clone, Default)]
pub struct Nps(pub u32);

impl Display for Nps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}knps", self.0)
    }
}

impl Add for Nps {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[derive(Default)]
pub struct NpsDiff {
    first: Nps,
    second: Nps,
    relative: f32,
}

impl NpsDiff {
    pub fn new(first: Nps, second: Nps) -> Self {
        let relative = (second.0 as f32 - first.0 as f32) / first.0 as f32;
        Self { first, second, relative }
    }
}

impl Display for NpsDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: Custom definition of >/< !!!
        let color = if self.second > self.first {
            Color::Green
        } else if self.second < self.first {
            Color::Red
        } else {
            Color::Black
        };

        let first = format!("{}", self.first).color(Color::Black);
        let second = format!("{}", self.second).color(color);
        let relative = format!(
            "({})", 
            format!("{:>+.2}%", 100.0 * self.relative).color(color)
        );

        write!(f, "{:>8} {:>8} {:>20}", first, second, relative)
    }
}

impl Add for NpsDiff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
            relative: self.relative + rhs.relative,
        }
    }
}

impl Div<usize> for NpsDiff {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            first: self.first / rhs,
            second: self.second / rhs,
            relative: self.relative / rhs as f32,
        }
    }
}

impl Div<usize> for Nps {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as u32)
    }
}

////////////////////////////////////////////////////////////////////////////////
/// 
/// Branching factor
///
////////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, PartialOrd, Serialize, Deserialize, Copy, Clone, Default)]
pub struct BFactor(pub f32);

impl Display for BFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl Add for BFactor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<usize> for BFactor {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as f32)
    }
}

#[derive(Default)]
pub struct BFactorDiff {
    first: BFactor,
    second: BFactor,
    relative: f32,
}

impl BFactorDiff {
    pub fn new(first: BFactor, second: BFactor) -> Self {
        let relative = (second.0 - first.0) / first.0;
        Self { first, second, relative }
    }
}

impl Display for BFactorDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: Custom definition of >/< !!!
        let color = if self.second < self.first {
            Color::Green
        } else if self.second > self.first {
            Color::Red
        } else {
            Color::Black
        };

        let first = format!("{}", self.first).color(Color::Black);
        let second = format!("{}", self.second).color(color);
        let relative = format!(
            "({})", 
            format!("{:>+.2}%", 100.0 * self.relative).color(color)
        );

        write!(f, "{:>5} {:>5} {:>20}", first, second, relative)
    }
}

impl Add for BFactorDiff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
            relative: self.relative + rhs.relative,
        }
    }
}

impl Div<usize> for BFactorDiff {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            first: self.first / rhs,
            second: self.second / rhs,
            relative: self.relative / rhs as f32,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
/// 
/// Score
///
////////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Copy, Clone, Default)]
pub struct Score(pub i32);

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+.2}", self.0 as f32/ 100.0)
    }
}

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<usize> for Score {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as i32)
    }
}


#[derive(Default)]
pub struct ScoreDiff {
    first: Score,
    second: Score,
    relative: f32,
}

impl ScoreDiff {
    pub fn new(first: Score, second: Score) -> Self {
        let relative = (second.0 as f32 - first.0 as f32) / first.0 as f32;
        Self { first, second, relative }
    }
}

impl Display for ScoreDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: Custom definition of >/< !!!
        let color = if self.second > self.first {
            Color::Green
        } else if self.second < self.first {
            Color::Red
        } else {
            Color::Black
        };

        let first = format!("{}", self.first).color(Color::Black);
        let second = format!("{}", self.second).color(color);

        write!(f, "{:>6} {:>6}", first, second)
    }
}

impl Add for ScoreDiff {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
            relative: self.relative + rhs.relative,
        }
    }
}

impl Div<usize> for ScoreDiff {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            first: self.first / rhs,
            second: self.second / rhs,
            relative: self.relative / rhs as f32,
        }
    }
}
