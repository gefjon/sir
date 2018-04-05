extern crate gnuplot;
extern crate num;

const X_LABEL: &str = "Time elapsed (days)";
const Y_LABEL: &str = "Number of people";
const TITLE: &str = "Write a title for this graph";
const TOTAL_POP: SirNumeric = 10.0;
const INFECTED: SirNumeric = 1.0;
const BETA: SirNumeric = 0.5;
const GAMMA: SirNumeric = 0.5;
const NUMBER_OF_DAYS: SirInteger = 10;

const SUSCEPTIBLE_CAPTION: &str = "Number susceptible";
const INFECTED_CAPTION: &str = "Number infected";
const REMOVED_CAPTION: &str = "Number removed";

const OUTFILE: &str = "sir-out";

use gnuplot::{AxesCommon, Figure, PlotOption};
use std::{fs::File, io::prelude::*};

mod calculations;
use calculations::{SirInteger, SirIterator, SirNumeric, SirStep};

fn main() {
    let data: Vec<SirStep> =
        SirIterator::from_total_pop(BETA, GAMMA, TOTAL_POP, INFECTED, NUMBER_OF_DAYS).collect();

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
    fg.set_terminal("png", &format!("{}.png", OUTFILE)).show();

    let mut f = File::create(format!("{}.csv", OUTFILE)).unwrap();
    for info in data {
        writeln!(
            f,
            "{}, {}, {}, {}",
            info.day, info.susceptible, info.infected, info.removed,
        ).unwrap();
    }
}
