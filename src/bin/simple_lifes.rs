use find_the_conways::{random_world, Cell, Model, Rule};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;
const SIZE: u32 = HEIGHT * WIDTH;

fn main() {
    let w = random_world(SIZE);
    for birth_min in 0..9 {
        for birth_max in birth_min..9 {
            for alive_min in 0..9 {
                for alive_max in alive_min..9 {
                    let rule = Rule {
                        birth_min,
                        birth_max,
                        alive_min,
                        alive_max,
                    };
                    try_rule(rule, &w);
                }
            }
        }
    }
}

fn try_rule(rule: Rule, world: &[Cell]) -> () {
    let file_name = format!(
        "./results/simple_life_{}_{}_{}_{}_{}.tsv",
        rule.birth_min,
        rule.birth_max,
        rule.alive_min,
        rule.alive_max,
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let f = File::create(file_name).expect("unable to create");
    let mut writer = BufWriter::new(f);
    let w = world.to_vec();
    println!("START {:?}", rule);
    let mut m = Model::from_world(w, WIDTH, HEIGHT, rule);
    for _ in 0..1001 {
        writer
            .write_all(format!("{}\t", m.population).as_bytes())
            .expect("write_err");
        let _ = m.tick_and_report();
    }
    writer.write_all(b"\n").expect("write_err");
}
