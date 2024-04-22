use std::fs;

use crate::cli::csv::OutputFormat;

pub fn process_csv(input: &str, output: &str, fmt: OutputFormat) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let records = reader.records();
    let mut ret = Vec::with_capacity(128);
    for record in records {
        let record = record?;
        let value = headers
            .into_iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(value);
    }

    let content = match fmt {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
