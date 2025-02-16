/***********
 * Imports *
 ***********/
use std::fs;
use std::io::Error;

/*****************
 * Main Function *
 *****************/
fn main() {
    //let text = fs::read_to_string("logs.txt");

    //println!("{:#?}", text)

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong);
        }
    }

    match validate_email(String::from("test.test.com")) {
        Ok(..) => println!("email is valid"),
        Err(reason_this_failed_validation) => {
            println!("{}", reason_this_failed_validation)
        }
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(()) // Empty Tuple
    } else {
        Err(Error::other("emails must have an @ sign"))
    }
}

/*
 * Definition of 'Result' in Rust:
 *  enum Result<T, E> {
 *      Ok(T),
 *      Err(E)
 *  }
 */
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
