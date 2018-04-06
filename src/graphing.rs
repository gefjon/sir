use gnuplot::{AxesCommon, Figure, PlotOption};
use calculations::SirStep;
use std::{
    path::Path,
    ffi::OsStr,
};

const X_LABEL: &str = "Tid (dagar)";
const Y_LABEL: &str = "Antal personer";

const SUSCEPTIBLE_CAPTION: &str = "Antal mottagliga";
const INFECTED_CAPTION: &str = "Antal smittade";
const REMOVED_CAPTION: &str = "Antal borträknade";
const TOTAL_CASES_CAPTION: &str = "Antal fall";

pub fn make_figure(data: &[SirStep]) -> Figure {
    let mut fg = Figure::new();

    fg.axes2d()
        .set_x_label(X_LABEL, &[])
        .set_y_label(Y_LABEL, &[])
        .lines(
            data.iter().map(|i| i.day),
            data.iter().map(|i| i.susceptible),
            &[
                PlotOption::PointSymbol('.'),
                PlotOption::Caption(SUSCEPTIBLE_CAPTION),
                PlotOption::Color("black"),
            ],
        )
        .lines(
            data.iter().map(|i| i.day),
            data.iter().map(|i| i.infected),
            &[
                PlotOption::PointSymbol('x'),
                PlotOption::Caption(INFECTED_CAPTION),
                PlotOption::Color("blue"),
            ],
        )
        .lines(
            data.iter().map(|i| i.day),
            data.iter().map(|i| i.removed),
            &[
                PlotOption::PointSymbol('+'),
                PlotOption::Caption(REMOVED_CAPTION),
                PlotOption::Color("green"),
            ],
        )
        .lines(
            data.iter().map(|i| i.day),
            data.iter().map(|i| i.infected + i.removed),
            &[
                PlotOption::Caption(TOTAL_CASES_CAPTION),
                PlotOption::Color("red"),
            ],
        );

    fg
}

pub fn graph_all<S: AsRef<OsStr>>(
    data: &[SirStep],
    filename: &Path,
    terminals: &[S],
)
where
    S: AsRef<OsStr>
{
    if terminals.is_empty() {
        return;
    }
    
    let mut fg = make_figure(data);
    for term in terminals {
        fg.set_terminal(
            &term.as_ref().to_string_lossy(),
            &filename.with_extension(term).to_string_lossy()
        )
            .show();
    }
}
