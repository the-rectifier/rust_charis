use anyhow::{bail, Result};
use log::{error, info, warn};
use phf::phf_map;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Write};
use std::slice::SliceIndex;
use structopt::StructOpt;

static ISA_I_TYPE: phf::Map<&'static str, &'static str> = phf_map! {
    "li" => "111000",
    "lui" => "111001",
    "addi" => "110000",
    "nandi" => "110010",
    "ori" => "110011",
    "b" => "111111",
    "beq" => "000000",
    "bne" => "000001",
    "lb" => "000011",
    "sb" => "000111",
    "lw" => "001111",
    "sw" => "011111",
};

static ISA_R_TYPE: phf::Map<&'static str, &'static str> = phf_map! {
    "add" => "110000",
    "sub" => "110001",
    "and" => "110010",
    "or" => "110011",
    "not" => "110100",
    "nand" => "110101",
    "nor" => "110110",
    "sra" => "111000",
    "srl" => "111001",
    "sll" => "111010",
    "rol" => "111100",
    "ror" => "111101",
};

#[derive(StructOpt, Debug)]
pub struct Asm {
    #[structopt(long = "file", short = "f")]
    file: String,
}

#[derive(StructOpt, Debug)]
pub struct Disasm {
    #[structopt(long = "file", short = "f")]
    file: String,
}

fn handle_immed(immed: &str) -> Result<i16> {
    let data: i16;

    if immed.chars().nth(0) == Some('-') && immed.len() >= 3 && &immed[..3] == "-0x" {
        data = !(u16::from_str_radix(&immed[3..], 16)? as i16);
    } else if immed.len() >= 2 && &immed[..2] == "0x" {
        data = u16::from_str_radix(&immed[2..], 16)? as i16;
    } else {
        data = i16::from_str_radix(immed, 10)?;
    }

    Ok(data)
}

pub fn do_asm(args: Asm) -> Result<()> {
    let mut instructions: Vec<String> = Vec::new();
    let file = File::open(&args.file)?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    reader.read_to_string(&mut data)?;
    let data: Vec<&str> = data.split("\n").filter(|x| !x.is_empty()).collect();

    for instruction in data {
        let r_instruction: bool;
        let tokens: Vec<&str> = instruction
            .split(&[',', ' '][..])
            .filter(|x| !x.is_empty())
            .collect();

        println!("{:?}", tokens);
        if let Some(_) = ISA_R_TYPE.get(tokens[0]) {
            r_instruction = true;
        } else if let Some(_) = ISA_I_TYPE.get(tokens[0]) {
            r_instruction = false;
        } else {
            bail!("Undefined Instruction!!");
        }

        if r_instruction {
            // Assemble R Instructions with 3 and 4 tokens
            match tokens.len() {
                4 => {
                    let rd = tokens[1].split_at(1).1.parse::<u8>()?;
                    let rs = tokens[2].split_at(1).1.parse::<u8>()?;
                    let rt = tokens[3].split_at(1).1.parse::<u8>()?;
                    let func = ISA_R_TYPE.get(tokens[0]).unwrap();
                    let assembled = format!("100000{:05b}{:05b}{:05b}00000{}", rs, rd, rt, func);
                    assert_eq!(assembled.len(), 32);

                    instructions.push(assembled);
                }
                3 => {
                    let rd = tokens[1].split_at(1).1.parse::<u8>()?;
                    let rs = tokens[2].split_at(1).1.parse::<u8>()?;
                    let rt = 0;
                    let func = ISA_R_TYPE.get(tokens[0]).unwrap();
                    let assembled = format!("100000{:05b}{:05b}{:05b}00000{}", rs, rd, rt, func);
                    assert_eq!(assembled.len(), 32);

                    instructions.push(assembled);
                }
                _ => bail!("Undefined amount of tokens found!"),
            }
        } else {
            let rs: u8;
            let rd: u8;
            let immed: i16;
            let op = tokens[0];
            let opcode = *ISA_I_TYPE.get(tokens[0]).unwrap();
            match tokens.len() {
                2 => {
                    rs = 0;
                    rd = 0;
                    immed = handle_immed(tokens[1])?;
                }
                3 => {
                    if op == "sw" || op == "sb" || op == "lw" || op == "lb" {
                        // lw r4, 5(r6)
                        // tokens = [lw, r4, 5(r6)]
                        rd = tokens[1].split_at(1).1.parse::<u8>()?;
                        let offset: Vec<&str> = tokens[2]
                            .split(&['(', ')', 'r'][..])
                            .filter(|x| !x.is_empty())
                            .collect();
                        immed = handle_immed(offset[0])?;
                        rs = offset[1].parse::<u8>()?;
                    } else {
                        // li, r5, 0x4
                        rs = 0;
                        rd = tokens[1].split_at(1).1.parse::<u8>()?;
                        immed = handle_immed(tokens[2])?;
                    }
                }
                4 => {
                    // addi r4, r5, 139
                    rd = tokens[1].split_at(1).1.parse::<u8>()?;
                    rs = tokens[2].split_at(1).1.parse::<u8>()?;
                    immed = handle_immed(tokens[3])?;
                }
                _ => bail!("Undefined amount of tokens found!"),
            }
            let assembled = format!("{}{:05b}{:05b}{:016b}", opcode, rs, rd, immed);
            println!("{}", assembled);
            assert_eq!(assembled.len(), 32);
            instructions.push(assembled);
        }
    }
    Ok(())
}

pub fn do_disasm(args: Disasm) -> Result<()> {
    println!("DISASSEMBLY!!!");

    Ok(())
}
