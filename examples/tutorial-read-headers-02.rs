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
        // We nest this call in its own scope because of lifetimes.
        // let headers = rdr.headers()?;
        // print!("{:} => "," headers");
        // println!("{:?}", headers);

        for result in rdr.headers() {

                println!("Header => {:?}", result);
                let record = result;
                for field in record {
                    println!(" Here => {:?}", field);
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
        
        println!("{:?}", "new line");
        for field in &record {
            print!("{:?}", field);
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
