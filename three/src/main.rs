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
    let data = read_data(&opts.input)?;
    let vdata = data.iter().collect();
    let (gamma, epsilon) = rates(&vdata);
    let gdec = bin_to_dec(gamma);
    let edec = bin_to_dec(epsilon);
    println!("Power Consumption: {}", gdec * edec);
    let (o2, co2) = o2_co2(&vdata);
    let o2dec = bin_to_dec(o2);
    let co2dec = bin_to_dec(co2);
    println!("Life Support Rating: {}", o2dec * co2dec);
    Ok(())
}

fn bin_to_dec(b: Vec<u8>) -> u64 {
    let mut pow = 0;
    let mut dec = 0;
    for d in b.into_iter().rev() {
        dec += 2_u64.pow(pow) * d as u64;
        pow += 1;
    }
    dec
}

fn rates(nums: &Vec<&Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
    let mut i = 0;
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    while i < nums[0].len() {
        match mcb(&nums, i) {
            MCB::One | MCB::Equal => {
                gamma.push(1);
                epsilon.push(0);
            }
            MCB::Zero => {
                gamma.push(0);
                epsilon.push(1);
            }
        }
        i += 1
    }
    (gamma, epsilon)
}

fn o2_co2(nums: &Vec<&Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
    let mut o2 = nums.to_vec();
    let mut co2 = nums.to_vec();
    let mut i = 0;
    while i < nums[0].len() {
        match mcb(&o2, i) {
            MCB::One | MCB::Equal => {
                if o2.len() > 1 {
                    o2 = o2.into_iter().filter(|x| x[i] == 1).collect();
                }
            }
            MCB::Zero => {
                if o2.len() > 1 {
                    o2 = o2.into_iter().filter(|x| x[i] == 0).collect();
                }
            }
        }
        match mcb(&co2, i) {
            MCB::One | MCB::Equal => {
                if co2.len() > 1 {
                    co2 = co2.into_iter().filter(|x| x[i] == 0).collect();
                }
            }
            MCB::Zero => {
                if co2.len() > 1 {
                    co2 = co2.into_iter().filter(|x| x[i] == 1).collect();
                }
            }
        }

        i += 1
    }
    (
        o2.into_iter().next().unwrap().to_vec(),
        co2.into_iter().next().unwrap().to_vec(),
    )
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MCB {
    One,
    Zero,
    Equal,
}

fn mcb(nums: &Vec<&Vec<u8>>, index: usize) -> MCB {
    let count: usize = nums.into_iter().map(|num| num[index] as usize).sum();
    let zeros = nums.len() - count;
    if count == zeros {
        MCB::Equal
    } else if count > zeros {
        MCB::One
    } else {
        MCB::Zero
    }
}

fn read_data(path: &Path) -> Result<Vec<Vec<u8>>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|x| !x.trim().is_empty())
                .map(|x| x.parse::<u8>().unwrap())
                .collect()
        })
        .collect())
}
