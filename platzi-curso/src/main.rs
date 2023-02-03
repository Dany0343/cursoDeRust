use csv::{ReaderBuilder, StringRecord}; // Esto permite leer un String que tenga forma de CSV
use std::{fs}; // Para poder traernos el archivo al código

const FILENAME: &str = "history.csv";
fn main() {

    let content = fs::read_to_string(FILENAME).unwrap();
    println!("{}", content);

}