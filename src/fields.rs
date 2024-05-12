use crate::Cli;

pub struct Fields {
    pub nodes: bool,
    pub time: bool,
    pub nps: bool,
    pub branching: bool,
    pub score: bool,
    pub best_move: bool
}

impl Default for Fields {
    fn default() -> Self {
        Self {
            nodes: true,
            time: true,
            nps: true,
            branching: true,
            score: true,
            best_move: true
        }
    }
}

pub trait Extract {
    fn extract<'a>(&self, fields: &'a Fields) -> Vec<String>;
}

impl<'a> From<&Cli> for Fields {
    fn from(value: &Cli) -> Self {
        Self {
            nodes: value.all || value.nodes,
            time: value.all || value.time,
            nps: value.all || value.nps,
            branching: value.all || value.branching,
            score: value.all || value.score,
            best_move: value.all || value.best_move,
        }
    }
}
