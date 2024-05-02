use std::fmt::Display;

use colored::{ Color, Colorize };

use crate::search_result::SearchResult;

pub struct ResultDiff {
    position: String,
    nodes: Diff<u32>,
    time: Diff<u32>,
    nps: Diff<u32>,
    branching_factor: Diff<f32>,
    best_move: Diff<String>,
}

impl Display for ResultDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<75} | {:>30} | {:>30} | {:>30} | {:>30} | {:>30}", 
            format!("{}", self.position).blue(), 
            self.nodes, 
            self.time, 
            self.nps, 
            self.branching_factor, 
            self.best_move
        )
    }
}

impl ResultDiff {
    pub fn new(first: SearchResult, second: SearchResult) -> Self {
        Self {
            position: first.position,
            nodes: Diff::new(first.nodes, second.nodes),
            time: Diff::new(first.time as u32, second.time as u32),
            nps: Diff::new(first.nps, second.nps),
            branching_factor: Diff::new(first.branching_factor, second.branching_factor),
            best_move: Diff::new(first.best_move, second.best_move),
        }
    }
}

pub struct Diff<T> where T: Display {
    first: T,
    second: T,
}

impl<T> Diff<T> where T: Display {
    pub fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

impl Display for Diff<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = if self.first > self.second {
            Color::Green
        } else if self.first > self.second {
            Color::Red
        } else {
            Color::Black
        };

        let relative = (self.second as f32 - self.first as f32) / self.second as f32;

        let first = format!("{:>10.2}", self.first).black();
        let second = format!("{:>10.2}", self.second).color(color);
        let relative = format!("{:>+5.2}%", relative).color(color);

        write!(f, "{first} {second} ({relative:>8})")
    }
}

impl Display for Diff<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = if self.first > self.second {
            Color::Green
        } else if self.first > self.second {
            Color::Red
        } else {
            Color::Black
        };

        let relative = (self.second as f32 - self.first as f32) / self.second as f32;

        let first = format!("{:>6.2}", self.first).black();
        let second = format!("{:>6.2}", self.second).color(color);
        let relative = format!("{:>+.2}", relative).color(color);

        write!(f, "{first} {second} ({relative})")
    }
}

impl Display for Diff<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = if self.first != self.second {
            Color::Red
        } else {
            Color::Black
        };

        let first = format!("{:>8}", self.first).black();
        let second = format!("{:>8}", self.second).color(color);

        write!(f, "{first} {second}")
    }
}
