mod email_address;
mod employee;
mod phone_number;

use employee::Employee;

use csv::{Reader, Writer};
use itertools::Itertools;
use std::error::Error;
use std::{env, process};

fn deduplicate_employee_file(
    filter: &str,
    input_filename: &str,
    output_filename: &str,
) -> Result<(), Box<dyn Error>> {
    let employees = read_employee_file(input_filename)?;

    let emp_cnt: usize = employees.len();
    println!("{} employees found...", emp_cnt);

    let dedup: Vec<Employee> = dedup_employees_by_filter(employees, filter)?;
    let dup_cnt: usize = emp_cnt - dedup.len();

    if dup_cnt == 1 {
        println!("1 duplicate employee merged...");
    } else if dup_cnt > 1 {
        println!("{} duplicate employees merged...", dup_cnt);
    } else {
        println!("No duplicate employees found...")
    }

    write_employee_file(dedup, output_filename)
}

fn parse_args(args: &[String]) -> Result<(&str, &str, &str), String> {
    let args_cnt: usize = args.len();
    if args_cnt != 4 {
        return Err(format!(
            "kev takes 3 arguments but {} were supplied",
            args_cnt - 1
        ));
    }
    let filter = &args[1];
    let input_filename = &args[2];
    let output_filename = &args[3];
    Ok((filter, input_filename, output_filename))
}

fn read_employee_file(filepath: &str) -> Result<Vec<Employee>, csv::Error> {
    let mut rdr = Reader::from_path(filepath)?;
    let mut employees: Vec<Employee> = Vec::new();
    for result in rdr.deserialize() {
        let emp: Employee = result?;
        employees.push(emp);
    }
    Ok(employees)
}

fn write_employee_file(
    employees: Vec<Employee>,
    output_filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(output_filename)?;

    for employee in employees {
        wtr.serialize(employee)?;
    }
    wtr.flush()?;
    Ok(())
}

fn dedup_employees_by_filter(
    employees: Vec<Employee>,
    filter: &str,
) -> Result<Vec<Employee>, String> {
    match filter {
        "email" => Ok(dedup_by_email(employees)),
        "phone" => Ok(dedup_by_phone(employees)),
        "email_or_phone" => {
            let email_deduped: Vec<Employee> = dedup_by_email(employees);
            Ok(dedup_by_phone(email_deduped))
        }
        unimplemented_filter => Err(format!(
            "\"{}\" filter not implemented",
            unimplemented_filter
        )),
    }
}

fn dedup_by_email(employees: Vec<Employee>) -> Vec<Employee> {
    let (no_email, email): (Vec<Employee>, Vec<Employee>) =
        employees.into_iter().partition(|e| e.email.is_none());
    let dedup: Vec<Employee> = email
        .into_iter()
        .unique_by(|e| e.email.clone())
        .collect::<Vec<_>>();
    no_email.into_iter().chain(dedup).collect::<Vec<_>>()
}

fn dedup_by_phone(employees: Vec<Employee>) -> Vec<Employee> {
    let (no_phone, phone): (Vec<Employee>, Vec<Employee>) =
        employees.into_iter().partition(|e| e.phone.is_none());
    let dedup: Vec<Employee> = phone
        .into_iter()
        .unique_by(|e| e.phone.clone())
        .collect::<Vec<_>>();
    no_phone.into_iter().chain(dedup).collect::<Vec<_>>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (filter, input_filename, output_filename): (&str, &str, &str) = parse_args(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    match deduplicate_employee_file(filter, input_filename, output_filename) {
        Ok(()) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("Problem deduplicating employee file: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_filter() {
        assert_eq!(
            dedup_employees_by_filter(Vec::new(), "email")
                .unwrap()
                .len(),
            0
        )
    }

    #[test]
    fn invalid_filter() {
        assert!(dedup_employees_by_filter(Vec::new(), "not_a_valid_filter").is_err())
    }
}
