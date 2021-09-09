mod obst;
use obst::Obst;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;

const DATA: &str = "./node.txt";
const GRAPG_NAME: &str = "obst";

fn to_ints(s: &str) -> Vec<i32> {
    s.split(' ')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn read_config(f: File) -> Obst {
    let mut obst = Obst::new();
    let mut f = BufReader::new(f);
    loop {
        let mut line = String::new();
        let len = f.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }
        let line = line.trim();
        if line.len() == 0 ||
            line.as_bytes()[0] == b'#' {
            continue;
        }
        let ints = to_ints(line);
        match ints.len() {
            1 => obst.insert_fail(ints[0] as usize),
            2 => obst.insert_node(ints[0], ints[1] as usize),
            _ => {},
        }
    }
    obst
}

fn main() -> io::Result<()> {
    let f = File::open(DATA)?;
    let obst = read_config(f);
    let bst = obst.obst()
                .map(|c| c.construct())
                .unwrap();
    let graph = format!("digraph tree{{\n{}}}", bst.graph().unwrap());
    let dot_file = format!("{}.dot", GRAPG_NAME);
    let mut dot = File::create(dot_file.clone())?;
    dot.write(graph.as_bytes())?;
    let jpg_file = format!("{}.jpg", GRAPG_NAME);
    Command::new("dot")
        .arg("-Tjpg")
        .arg(&dot_file)
        .arg("-o")
        .arg(&jpg_file)
        .output()
        .unwrap();
    Ok(())
}
