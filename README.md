# Introduction
Simple project to learn and practice writing CLI programs. This helped me practice a little more on error handling in Rust, as well as learn and practice a few Rust crates (mentioned below) and familiarize a little more with the documentation of the standard library.

## Crates or requirements
- Anyhow
- Clap
- CSV
- Serde

# Usage
This is a command line interface program, you do everything giving arguments to the program.
Display the help dialog by doing:

``` sh
departments --help
```

The available options are:
``` sh
Options:
  -p, --path <PATH>              file_path
  -l, --list                     List employees from department
  -f, --find <FIND>              Find an employee by name
      --add                      Add a new employee
  -d, --department <DEPARTMENT>  Department of the employee
  -n, --name <NAME>              Name of the employee
  -a, --age <AGE>                Age of the employee
  -s, --salary <SALARY>          Salary of the employee
  -h, --help                     Print help
  -V, --version                  Print version
```
## Listing employees
You can list employees like so:
``` sh
departments --list
```
Or you can search employees by department name by doing:
``` sh
departments --list -p <DEPARTMENT>
```
This will list all employees who are in the specified department.

## Finding an employee
You can search for a specific employee like so:
``` sh
departments --find -n <NAME>
```
This will list all employees with the specified name. This function can be improved by modifying the CSV structure and allowing full names in different fields, so that we could search for names or last names. But for now I think the example is sufficient.

## Adding a new employee
You can add a new employee like so:
``` sh
departments --add -d <DEPARTMENT> -n <NAME> -a <AGE> -s <SALARY>
```
