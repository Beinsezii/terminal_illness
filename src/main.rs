use clap::Parser;

pub mod cells;
mod tui;

use cells::{Game, CellOpts};
use tui::TuiOpts;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    /// Use numbers 0-9 instead of blocks '█'
    numeric: bool,

    #[clap(short, long)]
    /// Use term colors instead of heatmap
    monochrome: bool,

    #[clap(short = 'c', long)]
    /// Only count 'true' neighbors
    no_corners: bool,

    #[clap(short, long, multiple_values = true, value_parser = clap::value_parser!(u8).range(0..=8))]
    /// #s of neighbors that cause growth
    grow: Vec<u8>,

    #[clap(short, long, multiple_values = true, value_parser = clap::value_parser!(u8).range(0..=8))]
    /// #s of neighbors that cause death
    die: Vec<u8>,

    #[clap(short, long)]
    /// Maximum life of a cell
    life: u8,
}

impl Args {
    pub fn cellopts(&self) -> CellOpts {
        let mut result = CellOpts {
            corners: !self.no_corners,
            life: self.life,
            grow: [false; 9],
            die: [false; 9],
        };

        for n in self.grow.iter() {
            result.grow[*n as usize] = true
        }
        for n in self.die.iter() {
            result.die[*n as usize] = true
        }

        result
    }

    pub fn tuiopts(&self) -> TuiOpts {
        TuiOpts { numeric: self.numeric, monochrome: self.monochrome }
    }
}

fn main() {
    let args = Args::parse();

    tui::run(Game::new(args.cellopts()), args.tuiopts());

    println!("{:?}", args.cellopts())
}
