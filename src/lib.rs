use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Employee {
    department: String,
    name: String,
    age: u8,
    salary: f64,
}

#[derive(Debug)]
pub enum FileError {
    NameIsEmpty,
    InvalidExtension,
    NotExists,
}

pub fn touch_default_file(filepath: &str) -> Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(b"Department,Name,Age,Salary\n")?;
    Ok(())
}

pub fn check_file_exists(filepath: &str) -> Result<bool, FileError> {
    let path = Path::new(&filepath);
    match path.extension() {
        None => {
            if filepath.is_empty() {
                return Err(FileError::NameIsEmpty);
            } else {
                return Err(FileError::InvalidExtension);
            }
        }
        Some(_) => {
            if path.exists() {
                return Ok(true);
            } else {
                return Err(FileError::NotExists);
            }
        }
    }
}

pub fn add_new(filepath: &str, department: &String, name: &String, age: u8, salary: f64) -> Result<()> {
    let path = Path::new(&filepath);
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    //let mut wtr = csv::Writer::from_writer(file);
    //
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    wtr.serialize(Employee {
        department: department.to_string(),
        name: name.to_string(),
        age,
        salary,
    })?;

    wtr.flush()?;
    Ok(())
}

// Finds employees by name
pub fn find_employees(filepath: &str, name: &str) -> Result<()> {
    let path = Path::new(&filepath);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut count = 0;
    for result in rdr.deserialize() {
        let empl: Employee = result?;
        if empl.name == name {
            println!("{:?}", empl);
            count += 1;
        }
    }
    if count == 0 {
        println!("No employees found with name {}", name);
    }

    Ok(())
}

// Lists employees. Prints either all employees or filtered by department
pub fn list_employees(filepath: &str, department: Option<&str>) -> Result<()> {
    let path = Path::new(&filepath);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    match department {
        None => {
            for result in rdr.deserialize() {
                let empl: Employee = result?;
                println!("{:?}", empl);
            }
        }
        Some(dep) => {
            for result in rdr.deserialize() {
                let empl: Employee = result?;
                if empl.department == dep {
                    println!("{:?}", empl);
                }
            }
        }
    }
    Ok(())
}
