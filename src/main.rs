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

use crate::fields::Extract;
use crate::fields::Fields;

mod positions;
mod search_result;
mod diff;
mod report;
mod engine;
mod tabulator;
mod fields;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Cli {
    /// The location of the engine binary
    engine: PathBuf,

    /// The depth to which to search each position. Ignored when comparing 
    /// diffs
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
    #[arg(short = 'S', long)]
    save: bool,

    /// Output all of the available metrics at once
    #[arg(short, long)]
    all: bool,

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

        let fields = Fields::from(self);

        let mut table = Tabulator::new();

        table.add_col("FEN", 72);

        if fields.nodes {
            table.add_col("Nodes", 45);
        }

        if fields.time {
            table.add_col("Time", 30);
        }

        if fields.nps {
            table.add_col("Nps", 30);
        }

        if fields.branching {
            table.add_col("Branching Factor", 25);
        }

        if fields.score {
            table.add_col("Score", 15);
        }

        println!("{}", table.header());

        for snapshot_result in snapshot {
            let board = snapshot_result.position.parse()?;
            let result = engine.search(board, snapshot_result.depth)?;
            let diff = Diff::new(snapshot_result, &result);

            // Print the diff in a table
            let row = diff.extract(&fields);
            println!("{}", table.row(&row));

            // Store the result
            results.push(result);
            diffs.push(diff);
        }

        // Print averages, potentially behind a flag
        println!("{}", table.row_separator());
        let averages = diffs.into_iter().sum::<Diff>() / results.len();
        let averages = averages.extract(&fields);

        println!("{}", table.row(&averages));

        // Print footer line
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

        let fields = Fields::from(self);

        let mut table = Tabulator::new();

        table.add_col("FEN", 72);

        if fields.nodes {
            table.add_col("Nodes", 20);
        }

        if fields.time {
            table.add_col("Time", 10);
        }

        if fields.nps {
            table.add_col("Nps", 10);
        }

        if fields.branching {
            table.add_col("Branching", 10);
        }

        if fields.score {
            table.add_col("Score", 10);
        }

        println!("{}", table.header());

        for fen in suite {
            let board = fen.parse()?;
            let result = engine.search(board, self.depth)?;

            let row = result.extract(&fields);
            println!("{}", table.row(&row));

            results.push(result);
        }

        // Print averages, potentially behind a flag
        println!("{}", table.row_separator());
        let averages = results.clone().into_iter().sum::<SearchResult>() / results.len();
        let averages = averages.extract(&fields);

        println!("{}", table.row(&averages));

        // Print footer line
        println!("{}", table.footer());

        Ok(results)
    }
}
