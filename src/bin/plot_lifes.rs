use gnuplot;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;

const SAVE_WIDTH: u32 = 800;
const SAVE_HEIGHT: u32 = 600;
const MAX_PLOT_SEQS: u32 = 100;
