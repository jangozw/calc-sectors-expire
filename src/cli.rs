//use tracing::*;
use std::fs::File;
//use std::io::prelude::*;
//use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Result};
use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
#[clap(name = "calc-sectors-expire", author = "Django")]
pub struct CLI {
    /// Specify an optional subcommand.
    #[clap(subcommand)]
    commands: Option<Command>,
}

impl CLI {
    pub fn start(self) -> Result<()> {
        match self.commands {
            Some(command) => {
                println!("{}", command.parse()?);
                Ok(())
            }
            _ => {
                Ok(())
            }
        }
    }
}


#[derive(Debug, Parser)]
pub enum Command {
    #[clap(name = "run", about = "")]
    Run(Run),
}

impl Command {
    pub fn parse(self) -> Result<String> {
        match self {
            Self::Run(command) => command.parse(),
        }
    }
}


#[derive(Debug, Parser)]
pub struct Run {
    #[clap(long = "begin_epoch", help="int")]
    pub begin_epoch: u64,

    #[clap(long = "expect_exp_power", help ="eg: 1P,2T,10G")]
    pub expect_exp_power: String,

    #[clap(long = "files", help="filenames, eg: --files a.txt --files b.txt")]
    pub files: Vec<String>,
}

pub struct SectorInfo {
    // id:u64,
    expiration: u64,
    sector_type: u64,
}

fn get_expect_exp_power_g(text: String) -> Result<u64> {
    let reg = Regex::new(r"^(\d+)(P|G|T)$")?;
    for cap in reg.captures_iter(&text.to_ascii_uppercase()) {
        let power: u64 = cap.get(1).unwrap().as_str().parse::<u64>()?;
        if power <=0 {
            return Err(anyhow::Error::msg("parsed power is 0"));
        }
        let unit = cap.get(2).unwrap().as_str();
        // println!("{:?} {:?}", power, unit);
        if unit == "P" {
            return Ok(power * 1024 * 1024);
        } else if unit == "T"{
            return Ok(power * 1024);
        }
        return Ok(power);
    }
    Err(anyhow::Error::msg("couldn't parse expect power"))
}

impl Run {
    pub fn parse(self) -> Result<String> {
        println!("run expect_expire_power:{} begin_epoch:{}  all sector expiration files: {:?}", self.expect_exp_power, self.begin_epoch, self.files);
        if self.files.len() == 0 {
            bail!("files len is 0")
        }
        let expect_exp_power = get_expect_exp_power_g(self.expect_exp_power)?;
        println!("expect power: {}", g_power_display(expect_exp_power));

        let mut sectors = Vec::new();
        for (_, filename) in self.files.iter().enumerate() {
            let sector_type = if filename.contains("32G") {
                32
            } else if filename.contains("64G") {
                64
            } else {
                bail!("failed to read sector_type from filename: {}", filename);
            };
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            for (_, line) in reader.lines().enumerate() {
                let line = String::from(line.unwrap());
                if line.is_empty() {
                    continue
                }
                let line: Vec<u64> = line.split(":").map(|s| {
                    s.parse::<u64>().unwrap()
                }).collect();
                if line.len() != 2 {
                    continue
                }
                // let sector_id = line[0];
                let sector_expiration = line[1];
                if sector_expiration >= self.begin_epoch {
                    sectors.push(SectorInfo {
                        // id: sector_id,
                        expiration: sector_expiration,
                        sector_type: sector_type,
                    });
                }
            }
            //------------------------- 从小到大--------------
            sectors.sort_by(|a, b| {
                a.expiration.cmp(&b.expiration)
            });
        }
        let mut exp_power: u64 = 0;
        let mut last_expiration: u64 = 0;
        for (_, info) in sectors.iter().enumerate() {
            exp_power += info.sector_type;
            if last_expiration != info.expiration {
                println!("at expiration: {} ({}) exp_power:{}", info.expiration, util::lotus::mainnet_height_to_datetime(info.expiration as i64), g_power_display(exp_power));
            }
            if exp_power >= expect_exp_power && last_expiration != info.expiration {
                return Ok(format!("done! begin_epoch:{} expect_expire_power: {}  end_expiration: {} ({}) actual_exp: {}", self.begin_epoch,
                                  g_power_display(expect_exp_power), info.expiration, util::lotus::mainnet_height_to_datetime(info.expiration as i64), g_power_display(exp_power)).to_string());
            }
            last_expiration = info.expiration;
        }

        return Err(anyhow::Error::msg("couldn't reach expect-expire-power"));
    }
}


fn g_power_display(n: u64) -> String {
    let power = if n < 1024 {
        format!("{} GiB", n).to_string()
    } else if n < 1024 * 1024 {
        format!("{} TiB", n / 1024).to_string()
    } else {
        format!("{} PiB", n / 1024 / 1024).to_string()
    };
    power
}


#[test]
fn test_cmp() {
    let mut a = Vec::new();
    a.push(1);
    a.push(4);
    a.push(2);
    a.push(5);
    a.sort_by(|a, b| {
        b.cmp(&a)
    });

    for v in a.iter() {
        println!("{}", v);
    }
    println!("min {}", a.last().unwrap());
}


