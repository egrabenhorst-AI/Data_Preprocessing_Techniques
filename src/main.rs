extern crate ndarray;
extern crate statrs;
extern crate plotly;


fn main() {
    let url = "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";

    match fetch_dataset(url) {
        Ok(csv_data) => {
        match load_dataset(&csv_data) {
            Ok(dataset) => {
                // The dataset is ready for processing
                println!("Loaded {} records", dataset.len());
                }
                Err(error) => {
                    eprintln!("Error loading dataset: {}", error);
                }
            }
        }
        Err(error) => {
            eprintln!("Error fetching dataset: {}", error);
        }
    }
}


use reqwest::blocking::get;
use std::io::Read;
fn fetch_dataset(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = get(url)?;
    let mut content = String::new();
    response.read_to_string(&mut content)?;
    println!("{:?}", &content[0..165]);
    Ok(content)
}

use csv::ReaderBuilder;
use std::error::Error;
fn load_dataset(csv_data: &str) -> Result<Vec<SalaryRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().delimiter(b'\n').from_reader(csv_data.as_bytes());
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: SalaryRecord = result?;
        records.push(record);
    }
    Ok(records)
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salaryinusd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: i32,
}



