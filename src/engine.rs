use std::io::{ BufRead, BufReader, BufWriter, Write };
use std::path::PathBuf;

use crate::search_result::SearchResult;

use simbelmyne_chess::{board::Board, movegen::moves::Move};
use simbelmyne_uci::client::UciClientMessage;
use simbelmyne_uci::engine::UciEngineMessage;
use simbelmyne_uci::search_info::SearchInfo;
use simbelmyne_uci::time_control::TimeControl;
use std::process::{Child, ChildStdin, ChildStdout, Command};
use std::process::Stdio;
use anyhow::anyhow;

#[allow(dead_code)]
pub struct Engine {
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

