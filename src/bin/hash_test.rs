use jomini::TextTape;
use std::env;
use std::collections::BTreeMap;
use std::fs::*;
use std::io::Read;
use std::vec::Vec;
use std::mem::drop;
use std::error::*;
fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(env::args_os().nth(1).unwrap())?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    drop(file);
    let tape = TextTape::from_slice(data.as_slice())?;
    let mut reader = tape.windows1252_reader();
    for (key, op, val) in reader.fields() {
        println!("{} {:?} {:?}", key.read_str(), op, val);
    }
    Ok(())
}