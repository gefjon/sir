use calculations::{SirInteger, SirNumeric};
use clap::{App, Arg, ArgGroup};
use std::path::PathBuf;

pub struct Args {
    pub beta: SirNumeric,
    pub gamma: SirNumeric,
    pub susceptible: Option<SirNumeric>,
    pub infected: SirNumeric,
    pub total_pop: Option<SirNumeric>,
    pub n_days: SirInteger,
    pub filename: PathBuf,
    pub csv: bool,
    pub terminals: Vec<String>,
}

pub fn parse_args() -> Args {
    let args = App::new("SIR")
        .version("0.1.0")
        .author("Arthur Goldman <arthur@goldman-tribe.org>")
        .about("A simple SIR calculator")
        .arg(Arg::with_name("beta")
             .short("b")
             .long("beta")
             .value_name("BETA")
             .required(true)
             .help("Rate of infection"))
        .arg(Arg::with_name("gamma")
             .short("g")
             .long("gamma")
             .value_name("GAMMA")
             .required(true)
             .help("Rate of recovery"))
        .arg(Arg::with_name("total-pop")
             .short("n")
             .long("total-pop")
             .value_name("N")
             .required_unless("susceptible")
             .conflicts_with("susceptible")
             .help("Total population"))
        .arg(Arg::with_name("susceptible")
             .short("s")
             .long("susceptible")
             .value_name("S")
             .required_unless("total-pop")
             .conflicts_with("total-pop")
             .help("Susceptible at t=0"))
        .arg(Arg::with_name("infected")
             .short("i")
             .long("infected")
             .value_name("I")
             .required(true)
             .help("Infected at t=0"))
        .arg(Arg::with_name("day-count")
             .short("t")
             .long("day-count")
             .value_name("T")
             .default_value("100")
             .help("Number of days to simulate"))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILENAME")
             .default_value("sir-out")
             .help("Filename to output")
             .long_help("Output filename. A path without extension; <FILE>.png and <FiLE>.csv will both be written. Overwrites if file exists."))
        .arg(Arg::with_name("csv")
             .short("c")
             .long("csv")
             .help("Produce <FILE>.png"))
        .arg(Arg::with_name("no-csv")
             .short("C")
             .long("no-csv")
             .help("Do not produce <FILE>.png"))
        .group(ArgGroup::with_name("should-produce-csv")
               .args(&["csv", "no-csv"]))
        .arg(Arg::with_name("terminals")
             .min_values(0)
             .long("terminals")
             .value_names(&["TERM"])
             .default_value("png")
             .help("Gnuplot terminals to output")
             .long_help("A list of Gnuplot terminals to output."))
        .get_matches();

    Args {
        beta: args.value_of("beta").unwrap().parse().unwrap(),
        gamma: args.value_of("gamma").unwrap().parse().unwrap(),
        infected: args.value_of("infected").unwrap().parse().unwrap(),
        n_days: args.value_of("day-count").unwrap().parse().unwrap(),
        filename: args.value_of("output").unwrap().into(),
        total_pop: args.value_of("total-pop").map(|s| s.parse().unwrap()),
        susceptible: args.value_of("susceptible").map(|s| s.parse().unwrap()),
        csv: !args.is_present("no-csv"),
        terminals: args.values_of("terminals").map(|i| i.map(|s| s.into()).collect()).unwrap_or_else(|| vec!("png".into())),
    }
}
