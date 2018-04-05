extern crate clap;
extern crate gnuplot;
extern crate num;

const X_LABEL: &str = "Time elapsed (days)";
const Y_LABEL: &str = "Number of people";
const TITLE: &str = "Write a title for this graph";

const SUSCEPTIBLE_CAPTION: &str = "Number susceptible";
const INFECTED_CAPTION: &str = "Number infected";
const REMOVED_CAPTION: &str = "Number removed";

use gnuplot::{AxesCommon, Figure, PlotOption};
use std::{fs::File, io::prelude::*};

pub mod calculations;
mod cli;
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
    } = cli::parse_args();

    let data = if let Some(total_pop) = total_pop {
        SirIterator::from_total_pop(beta, gamma, total_pop, infected, n_days)
    } else if let Some(susceptible) = susceptible {
        SirIterator::from_susceptible(beta, gamma, susceptible, infected, n_days)
    } else {
        panic!("Neither total-pop nor susceptible was supplied!")
    };

    let data: Vec<SirStep> = data.collect();

    let mut fg = Figure::new();

    fg.axes2d()
        .set_x_label(X_LABEL, &[])
        .set_y_label(Y_LABEL, &[])
        .set_title(TITLE, &[])
        .lines(
            data.iter().map(|i| i.day),
            data.iter().map(|i| i.susceptible),
            &[
                PlotOption::PointSymbol('.'),
                PlotOption::Caption(SUSCEPTIBLE_CAPTION),
                PlotOption::Color("green"),
            ],
        )
        .lines(
            data.iter().map(|i| i.day),
            data.iter().map(|i| i.infected),
            &[
                PlotOption::PointSymbol('x'),
                PlotOption::Caption(INFECTED_CAPTION),
                PlotOption::Color("red"),
            ],
        )
        .lines(
            data.iter().map(|i| i.day),
            data.iter().map(|i| i.removed),
            &[
                PlotOption::PointSymbol('+'),
                PlotOption::Caption(REMOVED_CAPTION),
                PlotOption::Color("black"),
            ],
        );
    fg.set_terminal("png", &format!("{}.png", filename)).show();

    let mut f = File::create(format!("{}.csv", filename)).unwrap();
    for info in data {
        writeln!(
            f,
            "{}, {}, {}, {}",
            info.day, info.susceptible, info.infected, info.removed,
        ).unwrap();
    }
}
