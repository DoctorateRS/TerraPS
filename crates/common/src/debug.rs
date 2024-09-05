use anyhow::Result;
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

pub fn print_json<T: Serialize>(value: T) -> Result<String> {
    let mut buf = Vec::new();
    let fmtr = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, fmtr);

    value.serialize(&mut ser)?;

    let res = String::from_utf8(buf)?;
    println!("{}", &res);
    Ok(res)
}
