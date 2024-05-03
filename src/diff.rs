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

pub struct DiffReport {
    entries: usize,

    nodes_total: u32,
    nodes_average: u32,
    nodes_relative_average: f32,
    nodes_improvement: u32,

    time_total: u32,
    time_average: u32,
    time_relative_average: f32,
    time_improvement: u32,

    nps_average: u32,
    nps_relative_average: f32,
    nps_improvement: u32,

    branching_factor_average: f32,
    branching_factor_relative_average: f32,
    branching_factor_improvement: u32,
}

impl DiffReport {
    pub fn new(diffs: &[ResultDiff]) -> Self {
        let entries = diffs.len().max(1);

        // Nodes
        let nodes_total = diffs.iter()
            .map(|diff| diff.nodes.second)
            .sum::<u32>();

        let nodes_average = nodes_total / entries as u32;
        let nodes_relative_average = diffs.iter()
            .map(|diff| relative(diff.nodes.first as f32, diff.nodes.second as f32))
            .sum::<f32>() / entries as f32;

        let nodes_improvement = diffs.iter()
            .filter(|diff| diff.nodes.second < diff.nodes.first)
            .count() as u32;

        // Time
        let time_improvement = diffs.iter()
            .filter(|diff| diff.time.second < diff.time.first)
            .count() as u32;
        let time_total = diffs.iter()
            .map(|diff| diff.time.second)
            .sum::<u32>();
        let time_average = time_total / entries as u32;
        let time_relative_average = diffs.iter()
            .map(|diff| relative(diff.time.first as f32, diff.time.second as f32))
            .sum::<f32>() / entries as f32;

        //Nps
        let nps_improvement = diffs.iter()
            .filter(|diff| diff.nps.second > diff.nps.first)
            .count() as u32;
        let nps_total = diffs.iter()
            .map(|diff| diff.nps.second)
            .sum::<u32>();
        let nps_average = nps_total / entries as u32;
        let nps_relative_average = diffs.iter()
            .map(|diff| relative(diff.nps.first as f32, diff.nps.second as f32))
            .sum::<f32>() / entries as f32;

        // Branching factor
        let branching_factor_improvement = diffs.iter()
            .filter(|diff| diff.branching_factor.second < diff.branching_factor.first)
            .count() as u32;
        let branching_factor_total = diffs.iter()
            .map(|diff| diff.branching_factor.second)
            .sum::<f32>();

        let branching_factor_relative_average = diffs.iter()
            .map(|diff| relative(diff.branching_factor.first, diff.branching_factor.second))
            .sum::<f32>() / entries as f32;

        let branching_factor_average = branching_factor_total / entries as f32;

        Self {
            entries,
            nodes_total,
            nodes_average,
            nodes_relative_average,
            nodes_improvement,

            time_total,
            time_average,
            time_relative_average,
            time_improvement,

            nps_average,
            nps_relative_average,
            nps_improvement,

            branching_factor_average,
            branching_factor_relative_average,
            branching_factor_improvement
        }
    }
}

impl Display for DiffReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nodes_relative_color = if self.nodes_relative_average < 0.0 {
            Color::Green
        } else if self.nodes_relative_average > 0.0 {
            Color::Red
        } else {
            Color::Black
        };

        let nodes_improvement_color = if self.nodes_improvement > self.entries as u32 / 2 {
            Color::Green
        } else if self.nodes_improvement < self.entries as u32 / 2 {
            Color::Red
        } else {
            Color::Black
        };

        let time_relative_color = if self.time_relative_average < 0.0 {
            Color::Green
        } else if self.time_relative_average > 0.0 {
            Color::Red
        } else {
            Color::Black
        };

        let time_improvement_color = if self.time_improvement > self.entries as u32 / 2 {
            Color::Green
        } else if self.time_improvement < self.entries as u32 / 2 {
            Color::Red
        } else {
            Color::Black
        };

        let nps_relative_color = if self.nps_relative_average > 0.0 {
            Color::Green
        } else if self.nps_relative_average < 0.0 {
            Color::Red
        } else {
            Color::Black
        };

        let nps_improvement_color = if self.nps_improvement > self.entries as u32 / 2 {
            Color::Green
        } else if self.nps_improvement < self.entries as u32 / 2 {
            Color::Red
        } else {
            Color::Black
        };

        let branching_factor_relative_color = if self.branching_factor_relative_average < 0.0 {
            Color::Green
        } else if self.branching_factor_relative_average > 0.0 {
            Color::Red
        } else {
            Color::Black
        };

        let branching_factor_improvement_color = if self.branching_factor_improvement > self.entries as u32 / 2 {
            Color::Green
        } else if self.branching_factor_improvement < self.entries as u32 / 2 {
            Color::Red
        } else {
            Color::Black
        };

        writeln!(f, "Report")?;
        writeln!(f, "----------------------------------------------------------")?;

        writeln!(f, "Nodes")?;
        writeln!(f, "-----")?;
        writeln!(f, "  Total:       {:>9} nodes", self.nodes_total)?;
        writeln!(f, "  Average:     {:>9} nodes", self.nodes_average)?;
        writeln!(f, "  Relative:    {}", format!("{:>+9.2}%", 100.0*self.nodes_relative_average).color(nodes_relative_color))?;
        writeln!(f, "  Improvement: {}/{:>4}", format!("{:>4}", self.nodes_improvement).color(nodes_improvement_color), self.entries)?;
        writeln!(f, "")?;

        writeln!(f, "Time")?;
        writeln!(f, "----")?;
        writeln!(f, "  Total:       {:>9}ms", self.time_total)?;
        writeln!(f, "  Average:     {:>9}ms", self.time_average)?;
        writeln!(f, "  Relative:    {}", format!("{:>+9.2}%", 100.0*self.time_relative_average).color(time_relative_color))?;
        writeln!(f, "  Improvement: {}/{:>4}", format!("{:>4}", self.time_improvement).color(time_improvement_color), self.entries)?;
        writeln!(f, "")?;

        writeln!(f, "Nps")?;
        writeln!(f, "---")?;
        writeln!(f, "  Average:     {:>9}knps", self.nps_average)?;
        writeln!(f, "  Relative:    {}", format!("{:>+9.2}%", 100.0*self.nps_relative_average).color(nps_relative_color))?;
        writeln!(f, "  Improvement: {}/{:>4}", format!("{:>4}", self.nps_improvement).color(nps_improvement_color), self.entries)?;
        writeln!(f, "")?;

        writeln!(f, "Branching factor ")?;
        writeln!(f, "----------------")?;
        writeln!(f, "  Average:     {:>9.2}", self.branching_factor_average)?;
        writeln!(f, "  Relative:    {}", format!("{:>+9.2}%", 100.0*self.branching_factor_relative_average).color(branching_factor_relative_color))?;
        writeln!(f, "  Improvement: {}/{:>4}", format!("{:>4}", self.branching_factor_improvement).color(branching_factor_improvement_color), self.entries)?;

        Ok(())
    }
}

pub fn relative(first: f32, second: f32) -> f32 {
    (second - first) / second
}
