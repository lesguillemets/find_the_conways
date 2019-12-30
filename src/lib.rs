pub mod base;
pub mod plot_utils;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub use crate::base::*;

pub type PopulationSeries = Vec<u32>;

pub fn read_serieses(fname: &str) -> HashMap<String, PopulationSeries> {
    let mut map = HashMap::new();
    let f = File::open(fname).expect("unable to open file");
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.expect("unable to read line");
        let mut words = line.split_whitespace();
        let header = words.next().expect("empty line?").to_owned();
        let ys: Vec<u32> = words
            .map(|n| n.parse::<u32>().unwrap_or_else(|_| panic!("{}", n)))
            .collect();
        map.entry(header).or_insert(ys);
    }
    map
}
