use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(help = "Path to input file")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let data = read_data(&opts.input)?;
    let mut horiz = 0;
    let mut depth = 0;
    for (instr, scalar) in &data {
        match instr {
            Instruction::Down => depth += *scalar,
            Instruction::Up => depth -= *scalar,
            Instruction::Forward => horiz += *scalar,
        }
    }
    println!("Score: {}", horiz * depth);

    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (instr, scalar) in &data {
        match instr {
            Instruction::Down => aim += *scalar,
            Instruction::Up => aim -= *scalar,
            Instruction::Forward => {
                horiz += *scalar;
                depth += *scalar * aim
            }
        }
    }
    println!("Aim Score: {}", horiz * depth);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Up,
    Down,
    Forward,
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let out = match s {
            "up" => Instruction::Up,
            "down" => Instruction::Down,
            "forward" => Instruction::Forward,
            _ => return Err(anyhow!("Invalid data: {s}")),
        };
        Ok(out)
    }
}

fn read_data(path: &Path) -> Result<Vec<(Instruction, u64)>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input
        .lines()
        .map(|line| {
            if let [instruction, scalar] = &line.trim().split(" ").collect::<Vec<_>>()[..] {
                (
                    Instruction::from_str(instruction).unwrap(),
                    scalar.parse::<u64>().unwrap_or_else(|err| {
                        panic!("invalid u64: {err:?} {line}");
                    }),
                )
            } else {
                panic!("Invalid data: {line}");
            }
        })
        .collect())
}
