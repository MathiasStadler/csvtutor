// FROM HERE
// https://docs.rs/csv/latest/csv/tutorial/index.html

//tutorial-read-02.rs
// use std::{env, error::Error, ffi::OsString, fs::File, process};
use std::{env, error::Error, ffi::OsString, io, process};

// fn run() -> Result<(), Box<dyn Error>> {

//     // /w file
//     // let file_path = get_first_arg()?;
//     // let file = File::open(file_path)?;
//     // let mut rdr = csv::Reader::from_reader(file);

//     // shorter
//     let file_path = get_first_arg()?;
//     let mut rdr = csv::Reader::from_path(file_path)?;

//     for result in rdr.records() {
//         let record = result?;
//         println!("{:?}", record);
//     }
//     Ok(())
// }

// fn run() -> Result<(), Box<dyn Error>> {
//     let mut rdr = csv::ReaderBuilder::new()
//         .has_headers(false)
//         .from_reader(io::stdin());
//     for result in rdr.records() {
//         let record = result?;
//         println!("{:?}", record);
//     }
//     Ok(())
// }

// /w headers
// fn run() -> Result<(), Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     {
//         // We nest this call in its own scope because of lifetimes.
//         let headers = rdr.headers()?;
//         println!("{:?}", headers);
//     }
//     for result in rdr.records() {
//         let record = result?;
//         println!("{:?}", record);
//     }
//     // We can ask for the headers at any time. There's no need to nest this
//     // call in its own scope because we never try to borrow the reader again.
//     let headers = rdr.headers()?;
//     println!("{:?}", headers);
//     Ok(())
// }

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    {
        let s = "Hello";
        let i = 42;

        // FROM HERE
        // https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
        // print_type_of(&s); // &str
        // print_type_of(&i); // i32
        // print_type_of(&main); // playground::main
        // print_type_of(&print_type_of::<i32>); // playground::print_type_of<i32>
        // print_type_of(&{ || "Hi!" }); // playground::main::{{closure}}

        // We nest this call in its own scope because of lifetimes.
        // let headers = rdr.headers()?;
        // print!("{:} => "," headers");
        // println!("{:?}", headers);

        for result in rdr.headers() {
            print!("Header => {:?} ", result);
            print_type_of(&result);
            let record = result;
            for field in record {
                print!("field => {:?} ", field);
                print_type_of(&field);
            }
        }
    }

    // for result in rdr.records() {
    //     let record = result?;
    //     println!("{:?}", record);
    // }

    // FROM HERE get String from StringRecord
    // https://users.rust-lang.org/t/transform-stringrecord-to-string/54195/2
    for result in rdr.records() {
        let record = result?;

        println!("{:?} : {:} items ", "new line =>", record.len());
        for field in &record {
            print!("field => {:?} ", field);
            print_type_of(&field);
        }
        println!("");
    }

    // We can ask for the headers at any time. There's no need to nest this
    // call in its own scope because we never try to borrow the reader again.

    // double
    // let headers = rdr.headers()?;
    // println!("{:?}", headers);
    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("type => {}", std::any::type_name::<T>())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
#[allow(dead_code)]
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    } else {
        process::exit(0);
    }
}

/*
cargo build --example tutorial-read-headers-02
./target/debug/examples/tutorial-read-headers-02 <valid.csv
*/
