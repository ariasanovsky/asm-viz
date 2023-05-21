use std::{path::PathBuf, env};
use asm_viz::parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: PathBuf = 
        args
        .get(1)
        .map_or("./samples/a.ptx".to_string().into(), 
    |arg| {
        arg.into()
    });

    let mut reader: PtxReader = file_path.try_into().unwrap();
    let foo = reader.outer_expression().unwrap();
}
