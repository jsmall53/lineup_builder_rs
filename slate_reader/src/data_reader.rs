use std::error::Error;

use csv;
use serde::{ Deserialize, Serialize };
use serde::de;
#[macro_use]
use serde_derive;

// TODO: figure out how to bound T by the Deserialize trait
pub struct SlateDataReader<T: Deserialize<'de> + Serialize> {
    file_path: String,
    data: Vec<T>
}

impl<T: Deserialize<'de> + Serialize> SlateDataReader<T> {
    pub fn new(path: &str) -> SlateDataReader<T> {
        SlateDataReader {
            data: Vec::<T>::new(),
            file_path: String::from(path)
        }
    }

    pub fn read(&mut self) -> Result<(), Box<Error>> {
        let mut reader = csv::Reader::from_path(&self.file_path).unwrap();
        for result in reader.deserialize() {
            let record: T = result?;
            self.data.push(record);
        }
        Ok(())
    }
}
