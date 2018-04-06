extern crate clap;
extern crate gnuplot;
extern crate num;

use std::{fs::File, io::prelude::*, path::Path};

pub mod calculations;
mod cli;
mod graphing;
use calculations::{SirIterator, SirStep};

fn main() {
    let cli::Args {
        beta,
        gamma,
        infected,
        n_days,
        filename,
        total_pop,
        susceptible,
        csv,
        terminals,
    } = cli::parse_args();

    let data = if let Some(total_pop) = total_pop {
        SirIterator::from_total_pop(beta, gamma, total_pop, infected, n_days)
    } else if let Some(susceptible) = susceptible {
        SirIterator::from_susceptible(beta, gamma, susceptible, infected, n_days)
    } else {
        panic!("Neither total-pop nor susceptible was supplied!")
    };

    let data: Vec<SirStep> = data.collect();
    
    if csv {
        write_csv(&data, &filename);
    }
    
    graphing::graph_all(&data, &filename, &terminals);
}

fn write_csv(data: &[SirStep], filename: &Path) {
    let mut f = File::create(&filename.with_extension("csv")).unwrap();
    for info in data {
        writeln!(
            f,
            "{}, {}, {}, {}",
            info.day, info.susceptible, info.infected, info.removed,
        ).unwrap();
    }
}
