use crate::{mycli::csv::CsvOpts, CmdExcutor, OutputFormat};
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

pub fn process_csv(input: &str, output: Option<String>, format: OutputFormat) -> Result<()> {
    let output = if let Some(o) = output {
        o.clone()
    } else {
        format!("output.{}", format)
    };

    let mut ret = Vec::new();
    let mut reader = Reader::from_path(input)?;

    // csv crate的fn headers(&mut self) -> Result<&StringRecord>
    // 成功的话,可以从Result里得到第一行的数据的引用,然后clone
    let headers = reader.headers()?.clone();
    for i in reader.records() {
        let record = i?;

        // 由于headers里有多条信息, record里也有多条信息,但都是一一对应的
        // headers.iter() -> 使用 headers的迭代器,
        // record.iter() -> 使用 record的迭代器,
        // zip() -> 将两个迭代器合并为一个元组的**迭代器**,
        // 即 [(header, record), ..]
        // colloct() 则将元组的迭代器转换为最终目标
        // 大多数情况下colloct可以推断类型,不过对于复杂的还是要手动标注
        // **这里选择json val是因为json的语法要求每个val都有key对应,符合该迭代器组合**
        let val = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(val);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;

    Ok(())
}

impl CmdExcutor for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            format!("output.{}", self.format)
        };
        process_csv(&self.input, Some(output), self.format)
    }
}
