use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self,BufRead, BufReader};

type MyResult<T> = Result<T, Box< dyn Error>>;

#[derive(Debug)]
pub struct Config {

    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {

    let mut prev_status = false;
    for filename in &config.files {

        match open(&filename) {
            Err(err) => {
                eprintln!("{}: {}", filename, err);
                prev_status = false;
            }
            Ok(reader) => {

            match config.bytes {
            
            None => {
                if prev_status {
                    println!();
                }
                prev_status = true;
                if (&filename[..] != "-" && &config.files.len() > &1 ) {
                    println!("==> {} <==", filename);
                }
                for (line_num, line) in reader.lines().enumerate() {

                    if line_num == config.lines {
                        println!();
                        break;
                    }
                    println!("{}", line.unwrap());
                }
            },
            Some (n) => {

                if prev_status {
                    println!();
                }
                prev_status = true;
                if (&filename[..] != "-" && &config.files.len() > &1 ) {
                    println!("==> {} <==", filename);
                }
                reader = reader as Box<BufReader<File>>;
                for (byte_num, exact_byte) in reader.bytes().enumerate() {

                    if byte_num == n {
                        println!();
                        break;
                    }
                    println!("{}", exact_byte);
                }
            }
            }
            }
        }
    }
    
    Ok(())
}

fn open(filename: &str) -> MyResult<Box <dyn BufRead>> {

    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
pub fn get_args() -> MyResult<Config> {

    let matches = App::new("headr")
                    .version("0.1.0")
                    .author("udayj")
                    .about("Rust head")
                    .arg(
                        Arg::with_name("lines")
                            .short("n")
                            .long("lines")
                            .help("Number of lines")
                            .takes_value(true)
                            .default_value("10")
                    )
                    .arg(
                        Arg::with_name("bytes")
                            .short("c")
                            .long("bytes")
                            .value_name("BYTES")
                            .help("Number of bytes")
                            .conflicts_with("lines")
                            .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("files")
                            .value_name("FILE")
                            .help("Input file(s)")
                            .multiple(true)
                            .default_value("-")
                    )
                    .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches.value_of("lines")
                                     .map(parse_positive_int)
                                     .transpose()
                                     .map_err(|e| format!("illegal line count -- {}",e ))?;
    
    let bytes = matches.value_of("bytes")
                                     .map(parse_positive_int)
                                     .transpose()
                                     .map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok (
        Config {
            files,
            lines: lines.unwrap(),
            bytes
        }
    )
}


fn parse_positive_int(val: &str) -> MyResult<usize> {
    
    let parsed_val = val.parse::<usize>();
    if parsed_val.is_ok() {
        let ret_val = parsed_val.unwrap();
        if ret_val > 0 {
            return Ok(ret_val);
        }
        else { 
            return Err(From::from(val));    
        }
    }
    else {
        return Err(From::from(val));
    }
    
}

#[test]
fn test_parse_positive_int() {

    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}