use std::process;

use clap::Parser;
#[allow(unused_imports)]
use department::{add_new, check_file_exists, list_employees, search_employee, touch_default_file, FileError};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file_path
    #[arg(short, long)]
    path: Option<String>,
    /// List employees from department
    #[arg(short, long, default_value_t = false)]
    list: bool,
    /// Find an employee by name
    #[arg(short, long, default_value_t = false)]
    find: bool,
    /// Add a new employee
    #[arg(long, default_value_t = false)]
    add: bool,
    /// Department of the employee
    #[arg(short, long)]
    department: Option<String>,
    /// Name of the employee
    #[arg(short, long)]
    name: Option<String>,
    /// Age of the employee
    #[arg(short, long)]
    age: Option<u8>,
    /// Salary of the employee
    #[arg(short, long)]
    salary: Option<f64>,
}

fn main() {
    let args = Args::parse();
    let filepath = match args.path {
        Some(path) => path,
        None => String::from("deps.csv"),
    };

    // Check for correct usage, if none of these options are selected, we do nothing
    if !args.list && !args.add && !args.find {
        process::exit(0);
    }

    // Check that file exists
    // If file doesn't exist, we create a default one
    match check_file_exists(&filepath) {
        Ok(_) => {}
        Err(error) => match error {
            FileError::NameIsEmpty => {
                println!("Name of departments file cannot be empty.");
                process::exit(1);
            }
            FileError::InvalidExtension => {
                println!("Invalid file extension. Not a CSV File?");
                process::exit(1);
            }
            // Create a new empty file if file doesn't exist
            FileError::NotExists => {
                println!(
                    "Departments file {} does not exist. Creating an empty one...",
                    filepath
                );
                match touch_default_file(&filepath) {
                    Err(error) => {
                        println!("Error creating new file: {error}");
                        process::exit(1);
                    }
                    Ok(_) => {
                        println!("File {} created succesfully.", filepath);
                    }
                }
            }
        },
    }

    if args.add {
        let department = match args.department {
            Some(dep) => dep,
            None => {
                println!("Error: Department name is required.");
                process::exit(1);
            }
        };
        let name = match args.name {
            Some(name) => name,
            None => {
                println!("Error: Employee name is required.");
                process::exit(1);
            }
        };
        let age = match args.age {
            Some(age) => age,
            None => {
                println!("Error: Age of employee is required.");
                process::exit(1);
            }
        };
        let salary = match args.salary {
            Some(sal) => sal,
            None => 0.0,
        };

        if let Err(error) = add_new(&filepath, &department, &name, age, salary) {
            println!("Error when adding new employee: {}", error);
            process::exit(1);
        }
        process::exit(0);
    }

    if args.list {
        match args.department {
            Some(dep) => {
                if let Err(error) = list_employees(&filepath, Some(dep)) {
                    println!("Error when listing employees: {error}");
                    process::exit(1);
                } else {
                    process::exit(0);
                }
            }
            None => {
                println!("No department specified. Printing all employees:");
                if let Err(error) = list_employees(&filepath, None) {
                    println!("Error when listing employees: {error}");
                    process::exit(1);
                } else {
                    process::exit(0);
                }
            }
        }
    }

    if args.find {
        match args.name {
            Some(name) => {
                if let Err(error) = search_employee(&filepath, name) {
                    println!("Error when listing employees: {error}");
                    process::exit(1);
                } else {
                    process::exit(0);
                }
            }
            None => {
                println!("No name provided for search.");
                process::exit(0);
            }
        }
    }
}
