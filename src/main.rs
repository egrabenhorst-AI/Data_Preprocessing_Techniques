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
                    let mut salaries = get_salaries(&dataset);
                    let mean_salary = mean(&salaries);
                    println!("Mean salary: {:.2}", mean_salary.unwrap());
                    let median_salary = median(&mut salaries);
                    println!("Median salary: {:.2}", median_salary.unwrap());
                    // let mode_salary = mode(&salaries);
                    // println!("Mode salary: {:.2}", mode_salary.unwrap());
                    // let standardized_salaries = standardize_salaries(&dataset);
                    // println!("Standardized salaries: {:?}", standardized_salaries);
                    // let job_title_mapping = create_job_title_mapping(&dataset);
                    // println!("Job title mapping: {:?}", job_title_mapping);
                    // let one_hot_encoded_job_titles =
                    //     one_hot_encode_job_titles(&dataset, &job_title_mapping);
                    // println!(
                    //     "One-hot encoded job titles: {:?}",
                    //     one_hot_encoded_job_titles
                    // );
                    //let us_based_feature = create_us_based_feature(&dataset);
                    //println!("US-based feature: {:?}", us_based_feature);
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
            records.push(SalaryRecord {
                work_year: item_elements[0].to_string().parse::<i32>()?,
                experience_level: item_elements[1].to_string(),
                employment_type: item_elements[2].to_string(),
                job_title: item_elements[3].to_string(),
                salary: item_elements[4].to_string().parse::<f64>()?,
                salary_currency: item_elements[5].to_string(),
                salary_in_usd: item_elements[6].to_string().parse::<f64>()?,
                employee_residence: item_elements[7].to_string(),
                remote_ratio: item_elements[8].to_string().parse::<f64>()?,
                company_location: item_elements[9].to_string(),
                company_size: item_elements[10].to_string(),
            });
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

fn get_salaries(dataset: &[SalaryRecord]) -> Vec<f64> {
    dataset.iter().map(|record| record.salary_in_usd).collect()
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn median(data: &mut [f64]) -> Option<f64> {
    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let len = data.len();
    if len % 2 == 0 {
        let mid1 = data[(len / 2) - 1];
        let mid2 = data[len / 2];
        Some((mid1 + mid2) / 2.0)
    } else {
        Some(data[len / 2])
    }
}

// fn mode(data: &[f64]) -> Option<f64> {
//     use std::collections::HashMap;
//     let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
//         *freqs.entry(value).or_insert(0) += 1;
//         freqs
//     });

//     frequencies
//         .into_iter()
//         .max_by_key(|&(_, count)| count)
//         .map(|(value, _)| *value)
// }

fn range(data: &[f64]) -> f64 {
    let max_value = data
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_value = data
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    max_value - min_value
}

fn variance(data: &[f64]) -> f64 {
    let mean_value = mean(data).unwrap();
    let squared_diffs: Vec<f64> = data
        .iter()
        .map(|&value| (value - mean_value).powi(2))
        .collect();
    mean(&squared_diffs).unwrap()
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    Some(variance(data).sqrt())
}

fn standardize_salaries(dataset: &[SalaryRecord]) -> Vec<f64> {
    let mean_salary = mean(
        &dataset
            .iter()
            .map(|record| record.salary_in_usd)
            .collect::<Vec<f64>>(),
    );
    let std_dev_salary = std_deviation(
        &dataset
            .iter()
            .map(|record| record.salary_in_usd)
            .collect::<Vec<f64>>(),
    );
    dataset
        .iter()
        .map(|record| (record.salary_in_usd - mean_salary.unwrap()) / std_dev_salary.unwrap())
        .collect()
}

use std::collections::HashMap;
use std::collections::HashSet;
fn create_job_title_mapping(dataset: &[SalaryRecord]) -> HashMap<String, usize> {
    let mut job_title_set: HashSet<String> = dataset
        .iter()
        .map(|record| record.job_title.clone())
        .collect();
    let mut job_title_mapping: HashMap<String, usize> = HashMap::new();
    for (index, job_title) in job_title_set.drain().enumerate() {
        job_title_mapping.insert(job_title, index);
    }
    job_title_mapping
}

fn one_hot_encode_job_titles(
    dataset: &[SalaryRecord],
    mapping: &HashMap<String, usize>,
) -> Vec<Vec<u8>> {
    dataset
        .iter()
        .map(|record| {
            let mut encoding = vec![0u8; mapping.len()];
            let index = mapping[&record.job_title];
            encoding[index] = 1;
            encoding
        })
        .collect()
}

fn create_us_based_feature(dataset: &[SalaryRecord]) -> Vec<u8> {
    dataset
        .iter()
        .map(|record| {
            if record.company_location == "US" {
                1u8
            } else {
                0u8
            }
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
