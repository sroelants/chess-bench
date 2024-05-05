use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;
use diff::Diff;
use engine::Engine;
use positions::POSITIONS;
use search_result::SearchResult;
use tabulator::Tabulator;

use std::fs::File;
use std::fs::write;

use colored::Colorize;

mod positions;
mod search_result;
mod diff;
mod report;
mod engine;
mod tabulator;

const COL_SEP: &'static str = "│";
const COL_WIDTH: usize = 3;
const FEN_COL_WIDTH: usize = 72;
const NODES_COL_WIDTH: usize = 15;
const TIME_COL_WIDTH: usize = 8;
const NPS_COL_WIDTH: usize = 8;
const BF_COL_WIDTH: usize = 8;
const SCORE_COL_WIDTH: usize = 6;
const NODESDIFF_COL_WIDTH: usize = 50;
const TIMEDIFF_COL_WIDTH: usize = 45;
const NPSDIFF_COL_WIDTH: usize = 45;
const BFDIFF_COL_WIDTH: usize = 12;
const SCOREDIFF_COL_WIDTH: usize = 20;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Cli {
    /// The location of the engine binary
    #[arg(short, long)]
    engine: PathBuf,

    /// The depth to which to search each position
    #[arg(short, long, default_value = "10")]
    depth: usize,

    /// The file to write the snapshot to
    #[arg(short, long, default_value = "./bench_snapshot.json")]
    output: PathBuf,

    /// A suite of fens to use
    #[arg(short, long)]
    fens: Option<PathBuf>,

    /// An existing snapshot to compare against
    #[arg(short, long, default_value = "./bench_snapshot.json")]
    snapshot: PathBuf,

    /// Write snapshot to output file
    #[arg(short = 'S', long = "save")]
    save: bool,

    /// Whether or not to include node count in the output
    #[arg(short, long)]
    nodes: bool,

    /// Whether or not to include time in the output
    #[arg(short, long)]
    time: bool,

    /// Whether or not to include time in the output
    #[arg(short = 'N', long)]
    nps: bool,

    /// Whether or not to include the branching factor in the output
    #[arg(short, long)]
    branching: bool,

    /// Whether or not to include the score in the output
    #[arg(short = 'E', long)]
    score: bool,

    /// Whether or not to include the best move in the output
    #[arg(short = 'B', long)]
    best_move: bool,
}

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}

impl Cli {
    /// Run the program either in Snapshot mode or Suite mode, depending on the
    /// CLI arguments
    pub fn run(&self) -> anyhow::Result<()> {
        let results = if let Ok(file) = File::open(self.snapshot.as_path()) {
            let file = BufReader::new(file);
            let snapshot: Vec<SearchResult> = serde_json::from_reader(file)?;

            self.run_snapshot(&snapshot)
        } else {
            let suite: Vec<String> = if let Some(file) = &self.fens {
                std::fs::read_to_string(file)
                    .unwrap()
                    .lines()
                    .map(|st| st.to_owned())
                    .collect()
            } else {
                POSITIONS.into_iter().map(|st| st.to_owned()).collect()
            };

            self.run_suite(&suite)
        }?;

        // Save the results to the requested output file
        if self.save {
            write(self.output.as_path(), serde_json::to_string(&results)?)?;
        }

        Ok(())
    }

    /// Run the engine against a snapshot of SearchResults and return the
    /// Vec of new SearchResults. 
    ///
    /// Also responsible for reporting/printing the results as they come in.
    fn run_snapshot(&self, snapshot: &[SearchResult]) -> anyhow::Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let mut diffs = Vec::new();
        let mut engine = Engine::new(&self.engine)?;

        // TODO: These widths should be determined from the flags
        let table = Tabulator::new()
            .cols(6)
            .widths(&[72, 45, 30, 30, 25, 15])
            .headings(&["FEN", "Nodes", "Time", "Nps", "Branching Factor", "Score"]);

        println!("{}", table.header());

        for snapshot_result in snapshot {
            let board = snapshot_result.position.parse()?;
            let result = engine.search(board, snapshot_result.depth)?;
            let diff = Diff::new(snapshot_result, &result);

            // Print the diff in a table
            println!("{}", table.row(&self.diff_entries(&diff)));

            // Store the result
            results.push(result);
            diffs.push(diff);
        }

        // Print averages, potentially behind a flag
        println!("{}", table.row_separator());
        let averages = diffs.into_iter().sum::<Diff>() / results.len();
        println!("{}", table.row(&self.diff_averages_entries(&averages)));

        println!("{}", table.footer());

        Ok(results)
    }

    /// Run a suite of board positions through the engine, and return a Vec
    /// of SearchResult.
    ///
    /// Also responsible for reporting/printing the results as they come in.
    fn run_suite(&self, suite: &[String]) -> anyhow::Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let mut engine = Engine::new(&self.engine)?;

        // TODO: These widths should be determined from the flags
        let table = Tabulator::new()
            .cols(5)
            .widths(&vec![80, 45, 35, 50, 12]);

        for fen in suite {
            let board = fen.parse()?;
            let result = engine.search(board, self.depth)?;

            // self.print_result(&result);
            results.push(result);
        }

        Ok(results)
    }

    /// Given the CLI flags, return a Vec of the requested table entries for a 
    /// search result
    fn result_entries(&self, result: &SearchResult) -> Vec<String> {
        let mut entries = Vec::new();
        entries.push(format!("{}", result.position.to_string().blue()));

        if self.nodes {
            entries.push(result.nodes.to_string());
        }

        if self.time {
            entries.push(result.time.to_string());
        }

        if self.nps {
            entries.push(result.nps.to_string());
        }

        if self.branching {
            entries.push(result.branching_factor.to_string());
        }

        if self.score {
            entries.push(result.score.to_string());
        }

        entries
    }

    /// Given the CLI flags, return a Vec of the requested table entries for a 
    /// diff
    fn diff_entries(&self, diff: &Diff) -> Vec<String> {
        let mut entries = Vec::new();

        entries.push(format!("{}", diff.position.to_string().blue()));

        if self.nodes {
            entries.push(diff.nodes.to_string());
        }

        if self.time {
            entries.push(diff.time.to_string());
        }

        if self.nps {
            entries.push(diff.nps.to_string());
        }

        if self.branching {
            entries.push(diff.branching_factor.to_string());
        }

        if self.score {
            entries.push(diff.score.to_string());
        }

        entries
    }

    fn diff_averages_entries(&self, diff: &Diff) -> Vec<String> {
        let mut entries = Vec::new();

        entries.push(String::from("Averages"));

        if self.nodes {
            entries.push(diff.nodes.to_string());
        }

        if self.time {
            entries.push(diff.time.to_string());
        }

        if self.nps {
            entries.push(diff.nps.to_string());
        }

        if self.branching {
            entries.push(diff.branching_factor.to_string());
        }

        if self.score {
            entries.push(String::from(""));
        }

        entries
    }
}
