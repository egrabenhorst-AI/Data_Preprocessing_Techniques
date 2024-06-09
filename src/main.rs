extern crate ndarray;
extern crate plotly;
extern crate statrs;

fn main() {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";

    match fetch_dataset(url) {
        Ok(csv_data) => {
            match load_dataset(&csv_data) {
                Ok(dataset) => {
                    // The dataset is ready for processing
                    println!("Loaded {} records", dataset.len());
                    let parsed_data = filter_and_convert(&dataset);
                    println!("Filtered and converted data: {:?}", parsed_data);
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
    Ok(content)
}

use std::error::Error;
fn load_dataset(csv_data: &str) -> Result<Vec<SalaryRecord>, Box<dyn Error>> {
    let items: Vec<&str> = csv_data.split("\n").collect();
    let mut records = Vec::new();
    for item in &items[1..] {
        let item_elements: Vec<&str> = item.split(",").collect();
        if item_elements.len() > 1 {
            records.push(
                SalaryRecord {
                    work_year: item_elements[0].to_string().parse::<i32>()?,
                    experience_level: item_elements[1].to_string(),
                    employment_type: item_elements[2].to_string(),
                    job_title: item_elements[3].to_string(),
                    salary: item_elements[4].to_string().parse::<f64>()?,
                    salary_currency: item_elements[5].to_string(),
                    salary_in_usd: item_elements[6].to_string().parse::<f64>()?,
                    employee_residence: item_elements[7].to_string(),
                    remote_ratio:  item_elements[8].to_string().parse::<f64>()?,
                    company_location: item_elements[9].to_string(),
                    company_size: item_elements[10].to_string(),
                }
            );
        }
    }
    Ok(records)
}


fn filter_and_convert(dataset: &[SalaryRecord]) -> Vec<(i32, String, f64)> {
    dataset
    .iter()
    .filter(|record| record.experience_level == "SE")
    .map(|record| {
        let salary_in_usd_rounded = record.salary_in_usd.round();
        (
        record.work_year,
        record.job_title.clone(),
        salary_in_usd_rounded,
        )
    })
    .collect()
}


#[derive(Debug)]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salary_in_usd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: String,
}