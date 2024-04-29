use std::fmt::Display;
use std::io::{ BufRead, BufReader, BufWriter, Write };
use std::path::PathBuf;

use clap::Parser;
use positions::POSITIONS;
use serde::{Deserialize, Serialize};
use simbelmyne_chess::{board::Board, movegen::moves::Move};
use simbelmyne_uci::client::UciClientMessage;
use simbelmyne_uci::engine::UciEngineMessage;
use simbelmyne_uci::search_info::SearchInfo;
use simbelmyne_uci::time_control::TimeControl;
use std::process::{Child, ChildStdin, ChildStdout, Command};
use std::process::Stdio;
use anyhow::anyhow;
use colored::Colorize;

mod positions;

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
        // Diff instead: use the fens from the snapshot
        let snapshot: Vec<SearchResult> = serde_json::from_reader(BufReader::new(file))?;

        for snapshot_result in snapshot {
            let board = snapshot_result.position.parse()?;
            let result = engine.search(board, depth)?;

            println!("{}", diff(&snapshot_result, &result));
            results.push(result);

        }
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

// We want to deserialize this from the json at some point, right?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SearchResult {
    position: String,
    depth: usize,
    best_move: String,
    nodes: u32,
    time: u64,
    branching_factor: f32,
    nps: u32,
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

// TODO: Implement a Diff struct that does a bunch of these pre-processing steps,
// and have this be part of the Display logic, 
fn diff(result1: &SearchResult, result2: &SearchResult) -> String {
        let fen = format!("{:<72}", result1.position).blue();

        let nodes1 = format!("{:>12}", result1.nodes).black();
        let nodes2 = if result1.nodes > result2.nodes {
            format!("{:>12}", result2.nodes).green()
        } else if result1.nodes < result2.nodes { 
            format!("{:>12}", result2.nodes).red()
        } else {
            format!("{:>12}", result2.nodes).black()
        };

        let node_rel = if result1.nodes > result2.nodes {
            format!("{:>+9.2}%", 100.0*(result2.nodes as f32 - result1.nodes as f32) / result1.nodes as f32).green()
        } else if result1.nodes < result2.nodes {
            format!("{:>+9.2}%", 100.0*(result2.nodes as f32 - result1.nodes as f32) / result1.nodes as f32).red()
        } else {
            format!("{:>+9.2}%", 100.0*(result2.nodes as f32 - result1.nodes as f32) / result1.nodes as f32).black()
        };

        let time1 = format!("{:>6}ms", result1.time).black();
        let time2 = if result1.time > result2.time {
            format!("{:>6}ms", result2.time).green()
        } else if result1.time < result2.time { 
            format!("{:>6}ms", result2.time).red()
        } else {
            format!("{:>6}ms", result2.time).black()
        };

        let time_rel = if result1.nodes > result2.nodes {
            format!("{:>+9.2}%", 100.0*(result2.time as f32 - result1.time as f32) / result1.time as f32).green()
        } else if result1.time < result2.time {
            format!("{:>+9.2}%", 100.0*(result2.time as f32 - result1.time as f32) / result1.time as f32).red()
        } else {
            format!("{:>+9.2}%", 100.0*(result2.time as f32 - result1.time as f32) / result1.time as f32).black()
        };

        let nps1 = format!("{:>8}knps", result1.nps).black();
        let nps2 = if result1.nps > result2.nps {
            format!("{:>8}knps", result2.nps).green()
        } else if result1.nps < result2.nps { 
            format!("{:>8}knps", result2.nps).red()
        } else {
            format!("{:>8}knps", result2.nps).black()
        };

        let bf1 = format!("{:>6.2}", result1.branching_factor).black();
        let bf2 = if result1.branching_factor > result2.branching_factor {
            format!("{:>6.2}", result2.branching_factor).green()
        } else if result1.branching_factor < result2.branching_factor { 
            format!("{:>6.2}", result2.branching_factor).red()
        } else {
            format!("{:>6.2}", result2.branching_factor).black()
        };

        let mv1 = format!("{:>6}", result1.best_move).black();
        let mv2 = if result1.best_move == result2.best_move {
            format!("{:>6}", result2.best_move).black()
        } else {
            format!("{:>6}", result2.best_move).red()
        };

    format!(
    "{fen} {nodes1} {nodes2} ({node_rel}) {time1} {time2} ({time_rel}) {nps1} {nps2} {bf1} {bf2} {mv1} {mv2}",
    )
}

#[allow(dead_code)]
struct Engine {
    path: PathBuf,
    process: Child,
    stdin: UciWriter,
    stdout: UciReader,
}

impl Engine {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let mut process = Command::new(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let stdin = process.stdin.take()
            .ok_or_else(|| anyhow!("Failed to attach to stdin"))?;

        let stdout = process.stdout.take()
            .ok_or_else(|| anyhow!("Failed to attach to stdout"))?;

        let writer = UciWriter::new(stdin);
        let reader = UciReader::new(stdout);

        let mut engine = Self { path, process, stdin: writer, stdout: reader };

        // Start the engine in UCI mode
        engine.send(UciClientMessage::Uci)?;

        for msg in &mut engine.stdout {
            if let UciEngineMessage::UciOk = msg {
                break;
            }
        }

        Ok(engine)
    }

    pub fn send(&mut self, msg: UciClientMessage) -> anyhow::Result<()> {
        self.stdin.write(msg)
    }

    pub fn set_position(&mut self, board: Board) -> anyhow::Result<()> {
        self.send(UciClientMessage::UciNewGame)?;
        self.send(UciClientMessage::Position(board, Vec::new()))?;
        Ok(())

    }

    pub fn search(&mut self, board: Board, depth: usize) -> anyhow::Result<SearchResult> {
        let mut best_move: Option<Move> = None;
        let mut latest_info: Option<SearchInfo> = None;

        self.set_position(board)?;
        self.send(UciClientMessage::Go(TimeControl::Depth(depth)))?;

        for msg in &mut self.stdout {
            match msg {
                UciEngineMessage::Info(info) => {
                    latest_info = Some(info);
                },

                UciEngineMessage::BestMove(mv) => {
                    best_move = Some(mv);
                    break;
                },

                _ => {}
            }
        }

        let best_move = best_move.unwrap();
        let latest_info = latest_info.unwrap();

        Ok(SearchResult::new(
            board, 
            best_move, 
            latest_info.nodes.unwrap(), 
            latest_info.time.unwrap(), 
            depth
        ))
    }
}

struct UciWriter {
    writer: BufWriter<ChildStdin>
}

impl UciWriter {
    pub fn new(stdin: ChildStdin) -> Self {
        Self { writer: BufWriter::new(stdin) }
    }

    pub fn write(&mut self, msg: UciClientMessage) -> anyhow::Result<()> {
        self.writer.write(format!("{}\n", msg.to_string()).as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
}

struct UciReader {
    reader: BufReader<ChildStdout>
}

impl UciReader {
    pub fn new(stdout: ChildStdout) -> Self {
        Self { reader: BufReader::new(stdout) }
    }
}

impl Iterator for UciReader {
    type Item = UciEngineMessage;

    fn next(&mut self) -> Option<Self::Item> {
        (&mut self.reader)
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| line.parse().ok())
            .next()
    }
}
