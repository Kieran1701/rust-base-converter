use std::process::exit;
use std::env;

fn print_help() {
    /* Print help message */
    println!{"USAGE: ./base_converter -B|O|D|X|h -b|o|d|x <number>
    -h: Print help and exit
    -B: Output binary
    -O: Output octal
    -D: Output decimal (default)
    -X: Output hexidecimal
    -b: Input binary
    -o: Input octal
    -d: Input decimal
    -x: Input hexidecimal (default)"};
}

fn set_type(arg: &str, is_in: bool) -> Result<u8, String> {
    /* Set the type based on the argument */
    if is_in {
        match arg {
            "-B" => Ok(0),
            "-O" => Ok(1),
            "-D" => Ok(2),
            "-X" => Ok(3),
            _ => Err(format!("{arg} is not an output option!"))
        }
    }
    else {
        match arg {
            "-b" => Ok(0),
            "-o" => Ok(1),
            "-d" => Ok(2),
            "-x" => Ok(3),
            _ => Err(format!("{arg} is not an input option!"))
        }
    }
}

fn read_number(num: &str, in_type: u8) -> Result<i128, &str> {
    match in_type {
        //0 => i128::from_str_radix(num, 2),
        0 => {
            match i128::from_str_radix(num, 2) {
                Ok(a) => Ok(a),
                Err(_e) => {
                    println!("Invalid number!  Enter a valid integer.  Exiting...");
                    exit(1);
                }
            }
        },
        1 => {
            match i128::from_str_radix(num, 8) {
                Ok(a) => Ok(a),
                Err(_e) => {
                    println!("Invalid number!  Enter a valid integer.  Exiting...");
                    exit(1);
                }
            }
        },
        2 => {
            match i128::from_str_radix(num, 10) {
                Ok(a) => Ok(a),
                Err(_e) => {
                    println!("Invalid number!  Enter a valid integer.  Exiting...");
                    exit(1);
                }
            }
        },
        3 => {
            match i128::from_str_radix(num, 16) {
                Ok(a) => Ok(a),
                Err(_e) => {
                    println!("Invalid number!  Enter a valid integer.  Exiting...");
                    exit(1);
                }
            }
        },
        _ => Err("Not a valid base option")
    }
}

fn print_convert(in_num: i128, out_type: u8) -> Result<(), &'static str> {
    match out_type {
        0 => Ok(println!("{}", format!("{:b}", in_num))),
        1 => Ok(println!("{}", format!("{:o}", in_num))),
        2 => Ok(println!("{in_num}")),
        3 => Ok(println!("{}", format!("{:x}", in_num))),
        _ => Err("Invalid type output!")
    }
}

fn main() {
    /* Convert among binary, octal, decimal, and hexidecimal */
    let args: Vec<String> = env::args().collect();
    if args[1]=="-h" || args[1]=="--help" {
        print_help();
        exit(0);
    }
    let in_type: u8 = set_type(&args[2], false).expect("Invalid ingest type.");  // num for ingest type: 0=b, 1=o, 2=d, 3=x
    let out_type: u8 = set_type(&args[1], true).expect("Invalid output type.");  // num for output type: 0=B, 1=O, 2=D, 3=X
    let in_num: i128 = read_number(&args[3], in_type).expect("Invalid number!  Enter a valid integer.  Exiting.");
    print_convert(in_num, out_type).expect("Invalid type output!");
    exit(0);
}
