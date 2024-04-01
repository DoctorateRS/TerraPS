use reqwest::get;
use serde::Serialize;
use serde_json::{from_reader, ser::PrettyFormatter, to_writer_pretty, Result as SerdeJsonResult, Serializer, Value};
use std::{fs::File, io::BufReader};

pub(crate) fn say_hi() {
    println!("hi")
}