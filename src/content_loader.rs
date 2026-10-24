use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvRow { pub values: HashMap<String, String> }

pub fn parse_csv(input: &str) -> Result<Vec<CsvRow>, String> {
    let mut lines = input.lines().filter(|line| !line.trim().is_empty());
    let header_line = lines.next().ok_or_else(|| "CSV is empty".to_string())?;
    let headers: Vec<String> = header_line.split(',').map(|s| s.trim().to_string()).collect();
    if headers.iter().any(String::is_empty) { return Err("CSV contains an empty header".into()); }

    let mut rows = Vec::new();
    for (line_number, line) in lines.enumerate() {
        let values: Vec<&str> = line.split(',').map(str::trim).collect();
        if values.len() != headers.len() {
            return Err(format!("CSV line {} has {} columns; expected {}", line_number + 2, values.len(), headers.len()));
        }
        let values = headers.iter().cloned().zip(values.into_iter().map(ToOwned::to_owned)).collect();
        rows.push(CsvRow { values });
    }
    Ok(rows)
}

pub fn require_column<'a>(row: &'a CsvRow, name: &str) -> Result<&'a str, String> {
    row.values.get(name).map(String::as_str).ok_or_else(|| format!("missing CSV column: {name}"))
}
