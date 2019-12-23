use gnuplot;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;

const SAVE_WIDTH: u32 = 800;
const SAVE_HEIGHT: u32 = 600;
const MAX_PLOT_SEQS: u32 = 100;

fn main() {
    if let Some(fname) = env::args().collect::<Vec<String>>().get(1) {
        println!("Plotting {}", fname);
        let mut fg = plot_tsv(fname);
        let out_name = format!(
            "./plots/plot_{}_{}.png",
            normalise(&fname),
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        println!("saving to ... {}", &out_name);
        fg.save_to_png(&out_name, SAVE_WIDTH, SAVE_HEIGHT).unwrap();
        println!("done.");
    } else {
        println!("specify a filename");
    }
}

fn plot_tsv(fname: &str) -> gnuplot::Figure {
    let f = File::open(fname).expect("unable to open file");
    let reader = BufReader::new(f);
    let mut fg = gnuplot::Figure::new();
    let axes = fg.axes2d();
    for line in reader.lines().take(MAX_PLOT_SEQS as usize) {
        let ys: Vec<u32> = line
            .expect("unable to read line")
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        axes.lines(0..ys.len() as u32, &ys, &[]);
    }
    fg.show().unwrap();
    fg
}

fn normalise(s: &str) -> String {
    std::path::Path::new(s)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from)
        .unwrap_or_else(|| String::from(""))
}
