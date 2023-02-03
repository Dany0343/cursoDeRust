use csv::{ReaderBuilder, StringRecord}; // Esto permite leer un String que tenga forma de CSV
use std::{fs}; // Para poder traernos el archivo al código

const FILENAME: &str = "./history.csv";
fn main() {

    let content = fs::read_to_string(FILENAME).unwrap();
    println!("{}", content);
    // leer datos tabulares como lo es en un CSV
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
    // la 'b' significa que se pasarán los datos como bytes y no como strings

    for result in rdr.records() {
        println!("{:?}", result);
    }

}