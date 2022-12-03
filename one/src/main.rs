use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(help = "Path to input file")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let depths = read_depths(&opts.input)?;
    let mut prev_depth = 0;
    let mut increases = 0;
    for depth in &depths {
        if prev_depth < *depth && prev_depth != 0 {
            increases += 1;
        }
        prev_depth = *depth;
    }
    println!("Depth Increases: {increases}");
    let mut prev_window_sum = 0;
    let mut sum_increases = 0;
    for i in 0..depths.len() - 2 {
        let sum: u64 = (&depths[i..i + 3]).into_iter().sum();
        if prev_window_sum < sum && prev_window_sum != 0 {
            sum_increases += 1;
        }
        prev_window_sum = sum;
    }
    println!("Window Increases: {sum_increases}");
    Ok(())
}

fn read_depths(path: &Path) -> Result<Vec<u64>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input
        .lines()
        .map(|line| {
            line.parse::<u64>().unwrap_or_else(|err| {
                panic!("invalid u64: {err:?} {line}");
            })
        })
        .collect())
}
