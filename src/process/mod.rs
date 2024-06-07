use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

//使用serde的序列化、反序列化
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Item {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut ret = Vec::new();
    let mut reader = Reader::from_path(input)?;
    for i in reader.deserialize() {
        let record: Item = i?;
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;

    Ok(())
}
