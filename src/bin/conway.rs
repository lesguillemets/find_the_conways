use find_the_conways::{random_world, Model, CONWAY};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;
const SIZE: u32 = HEIGHT * WIDTH;

fn main() {
    let file_name = format!(
        "./results/conways_{}.tsv",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let f = File::create(file_name).expect("unable to create");
    let mut writer = BufWriter::new(f);
    for trial in 0..1000 {
        println!("START {}", trial);
        let w = random_world(SIZE);
        let mut m = Model::from_world(w, WIDTH, HEIGHT, CONWAY);
        for _ in 0..1001 {
            writer
                .write_all(format!("{}\t", m.population).as_bytes())
                .expect("write_err");
            let _ = m.tick_and_report();
        }
        writer.write_all(b"\n").expect("write_err");
    }
}
