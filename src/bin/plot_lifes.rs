use gnuplot;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;

const SAVE_WIDTH: u32 = 800;
const SAVE_HEIGHT: u32 = 600;
const MAX_PLOT_SEQS: u32 = 100;

fn main() {
    let mut fg = plot_tsv("./results/whole_simple_lifes.tsv", &[]);
    fg.save_to_png(
        &format!(
            "plots/whole_simple_lifes_{}.png",
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ),
        1600,
        1200,
    )
    .unwrap();
}

type PopulationSeries = Vec<u32>;
fn plot_tsv(fname: &str) -> gnuplot::Figure {
    let f = File::open(fname).expect("unable to open file");
    let reader = BufReader::new(f);
    let mut fg = gnuplot::Figure::new();
    let axes = fg.axes2d();
    for line in reader.lines() {
        let line = line.expect("unable to read line");
        let mut words = line.split_whitespace();
        let header = words.next().expect("empty line?");
        println!("{}", header);
        let ys: Vec<u32> = words
            .map(|n| n.parse::<u32>().expect(&format!("{}", n)))
            .collect();
        axes.lines(0..ys.len() as u32, &ys, &[]);
    }
    fg
}

fn normalise(s: &str) -> String {
    std::path::Path::new(s)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from)
        .unwrap_or_else(|| String::from(""))
}
