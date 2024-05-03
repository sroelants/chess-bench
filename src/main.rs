use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;
use engine::Engine;
use positions::POSITIONS;
use search_result::SearchResult;

use crate::diff::{DiffReport, ResultDiff};

mod positions;
mod search_result;
mod diff;
mod report;
mod engine;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
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
}

fn main() -> anyhow::Result<()> {
    let Args { engine, depth, fens, output, snapshot, save } = Args::parse();

    let mut engine = Engine::new(engine)?;

    let mut results: Vec<SearchResult> = Vec::new();

    if let Ok(file) = std::fs::File::open(snapshot) {
        // Diff instead: use the fens and depths from the snapshot
        let snapshot: Vec<SearchResult> = serde_json::from_reader(BufReader::new(file))?;
        let mut diffs: Vec<ResultDiff> = Vec::new();

        println!("{:^75} | {:^32} | {:^32} | {:^32} | {:^21} | {:^20}", 
            "FEN",
            "Nodes",
            "Time",
            "Nps",
            "Branching factor",
            "Best move",
        );

        println!("{:->225}", "");
        for snapshot_result in snapshot {
            let board = snapshot_result.position.parse()?;
            let result = engine.search(board, snapshot_result.depth)?;

            let diff = ResultDiff::new(snapshot_result, result.clone());

            println!("{}", diff);
            results.push(result);
            diffs.push(diff);
        }

        let report = DiffReport::new(&diffs);
        println!("");
        println!("{report}");
    } else {
        let suite: Vec<String> = if let Some(file) = fens {
            std::fs::read_to_string(file)
                .unwrap()
                .lines()
                .map(|st| st.to_owned())
                .collect()
        } else {
            POSITIONS.into_iter().map(|st| st.to_owned()).collect()
        };

        for fen in suite {
            let board = fen.parse()?;
            let result = engine.search(board, depth)?;

            println!("{result}");
            results.push(result);
        }
    }

    if save {
        std::fs::write(output, serde_json::to_string(&results)?)?;
    }

    Ok(())
}
