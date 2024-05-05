use std::iter::repeat;

const SEP_WIDTH: usize = 3;

/// Helper struct that lets up print tabulated data in a sane way
pub struct Tabulator {
    cols: usize,
    widths: Vec<usize>,
    names: Vec<String>,
}

/// Creation/builder methods
impl Tabulator {
    pub fn new() -> Self {
        Self {
            cols: 0,
            widths: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn cols(mut self, cols: usize) -> Self {
        self.cols = cols;
        self
    }

    pub fn widths(mut self, widths: &[usize]) -> Self {
        self.widths = widths.iter()
            .cloned()
            .cycle()
            .take(self.cols)
            .collect();
        self
    }

    pub fn headings(mut self, headings: &[&str]) -> Self {
        self.names = headings.iter()
            .map(|s| s.to_string())
            .chain(repeat(String::from("")))
            .take(self.cols)
            .collect();
        self
    }
}

/// Tabulating logic
impl Tabulator {
    /// Return the table header as a string
    pub fn header(&self) -> String {
        let mut row = String::new();

        // Top line
        row.push_str(&format!("┌"));
        for (i, &width) in self.widths.iter().enumerate() {
            row.push_str(&format!("{}", "─".repeat(width + SEP_WIDTH/2 + 1)));

            if i < self.cols - 1 {
                row.push_str(&format!("┬"));
            }
        }
        row.push_str(&format!("┐"));
        row.push_str(&format!("\n"));

        // Heading names
        row.push_str(&format!("{:<1$}", "│", SEP_WIDTH/2 + 1));
        for (i, (name, width)) in self.names.iter().zip(self.widths.iter()).enumerate() {
            if i > 0 {
                let sep = format!("{:^1$}", "│", SEP_WIDTH);
                row.push_str(&sep);
            }

            let cell = format!("{:^1$}", name, width);

            row.push_str(&cell);
        }

        row.push_str(&format!("{:>1$}", "│", SEP_WIDTH/2 + 1));
        row.push_str(&format!("\n"));

        // Bottom line
        row.push_str(&self.row_separator());

        row
    }

    /// Return the table footer as a string
    pub fn footer(&self) -> String {
        let mut row = String::new();

        // Top line
        row.push_str(&format!("└"));
        for (i, &width) in self.widths.iter().enumerate() {
            row.push_str(&format!("{}", "─".repeat(width + SEP_WIDTH/2 + 1)));

            if i < self.cols - 1 {
                row.push_str(&format!("┴"));
            }
        }
        row.push_str(&format!("┘"));
        row.push_str(&format!("\n"));

        row

    }

    /// Given a slice of row entries, return the row as a string
    pub fn row(&self, values: &[String]) -> String {
        let mut row = format!("{:<1$}", "│", SEP_WIDTH/2 + 1);

        for (i, (value, width)) in values.iter().zip(self.widths.iter()).enumerate() {
            // Gotta figure out the "visual" length (ignoring color codes) so 
            // we can padd the cell correctly
            let stripped = strip_ansi_escapes::strip_str(value);
            let delta = value.len() - stripped.len();

            if i > 0 {
                let sep = format!("{:^1$}", "│", SEP_WIDTH);
                row.push_str(&sep);
            }

            // TODO: Make the alignment configurable
            let cell = if i == 0 {
                format!("{:<1$}", value, width + delta)
            } else {
                format!("{:>1$}", value, width + delta)
            };

            row.push_str(&cell);
        }

        row.push_str(&format!("{:>1$}", "│", SEP_WIDTH/2 + 1));

        row
    }

    pub fn row_separator(&self) -> String {
        let mut row = String::new();
        row.push_str(&format!("├"));
        for (i, &width) in self.widths.iter().enumerate() {
            row.push_str(&format!("{}", "─".repeat(width + SEP_WIDTH/2 + 1)));

            if i < self.cols - 1 {
                row.push_str(&format!("┼"));
            }
        }
        row.push_str(&format!("┤"));

        row
    }

    fn width(&self) -> usize {
        self.widths.iter().sum::<usize>() 
            + self.cols
            + self.cols * SEP_WIDTH
    }
}
