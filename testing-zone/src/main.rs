use std::io::{self, Write};
use std::time::Duration;
use std::str;
use std::io::{BufReader, BufRead};

use serialport::SerialPort;

fn hex_to_string(hex: &str) -> Result<String, hex::FromHexError> {
    hex::decode(hex).map(|bytes| String::from_utf8(bytes).unwrap())
}

fn main() {
    let port_name = "COM9"; // port for my computer only. change
    let baud_rate = 1200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(80))    //Is this the time given to open the port, or the time given to read from the port?
        .open();

    // Define a vector to hold the bytes we read to compare to for changes
    let mut stored_buffer = "";
    let mut stored_reading: Vec<u8> = vec![];

    match port {
        Ok(mut port) => {
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);

            // ATTEMPTED BUFREADER

            // let reader = BufReader::new(port);
            // for line in reader.lines() {
            //     match line {
            //         Ok(serial_buf) => {
            //             if serial_buf != stored_buffer {
            //                 // match hex_to_string(&serial_buf[..]) {
            //                 //     Ok(utf_string) => {
            //                 //         println!("{}", utf_string);
            //                 //         stored_buffer = &serial_buf.clone();
            //                 //     }
            //                 //     Err(e) => {
            //                 //         eprintln!("{}", serial_buf);
            //                 //         panic!("invalid UTF-8 sequence: {}", e);
            //                 //     }
            //                 // }                            
            //                 println!("{}", serial_buf);
            //                 stored_buffer = &serial_buf;

            //             }
            //         } Err(e) => {
            //             eprintln!("Error reading line: {}", e);
            //             break;
            //         }
            //     }
            // }

            // MANUALLY WRITTEN

            let mut serial_buf: Vec<u8> = vec![];   // Declare a new vector to hold the bytes we read
            loop {
                // read the byte
                match port.read(&mut [0; 1]) {
                    Ok(t) => {
                        let mut byte = [0; 1];
                        match port.read(&mut byte) {
                            // if the byte exist:
                            Ok(_) => {
                                // if the byte is a newline:
                                if byte[0] == b'\n' {
                                    if serial_buf != stored_reading {
                                        // turn the whole buffer into a string
                                        let utf_string = match std::str::from_utf8(&serial_buf) {
                                            Ok(s) => {
                                                // println!("Converted to string");
                                                s
                                            },
                                            Err(e) => {
                                                let _ = io::stdout().write_all(&serial_buf);
                                                panic!("Invalid UTF-8 sequence: {}", e);
                                            }
                                        };
                                        println!("{}", utf_string);
                                        // replace the old reading with the new reading
                                        stored_reading = serial_buf.clone();
                                        // should I clear the buffer?
                                    }
                                    serial_buf.clear();
                                } else {
                                    serial_buf.push(byte[0]);
                                    // for byte in &serial_buf {
                                    //     println!("{:X}", byte);
                                    // }
                                }
                                if serial_buf.len() > 14 {
                                    serial_buf.clear();
                                }
                            },
                            Err(ref e) if e.kind() == io::ErrorKind:: TimedOut => (),
                            Err(e) => {
                                eprintln!("{:?}", e);
                                break;
                            }
                        }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            // END MANUAL
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

