use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let port_name = "COM9"; // port for my computer only. change
    let baud_rate = 1200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))    //Is this the time given to open the port, or the time given to read from the port?
        .open();

    // Define a vector to hold the bytes we read to compare to for changes
    let mut stored_reading = Vec::new();

    match port {
        Ok(mut port) => {
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            let mut serial_buf: Vec<u8> = Vec::new();   // Declare a new vector to hold the bytes we read
            loop {
                // read the byte
                match port.read(&mut [0; 1]) {
                    Ok(t) => {
                        let mut byte = [0; 1];
                        match port.read(&mut byte) {
                            // if the byte exist:
                            Ok(_) => {
                                // if the byte is a newline:
                                if byte[0] == 0x0A {
                                    // go ahead and push it so the terminal looks nice (don't do it when not using the terminal)
                                    serial_buf.push(byte[0]);
                                    // check if the current buffer is NOT equal to the last reading
                                    if serial_buf != stored_reading {
                                        io::stdout().write_all(&serial_buf).unwrap();
                                        // turn the whole buffer into a string
                                        let utf_string = match std::str::from_utf8(&serial_buf) {
                                            Ok(s) => {
                                                println!("Converted to string");
                                                s
                                            },
                                            Err(e) => {
                                                panic!("Invalid UTF-8 sequence: {}", e);
                                            }
                                        };
                                        println!("Serial string {}", utf_string);
                                        // replace the old reading with the new reading
                                        stored_reading = serial_buf.clone();
                                        // should I clear the buffer?
                                        // serial_buf.clear()
                                    }
                                } else {
                                    serial_buf.push(byte[0]);
                                }
                                // io::stdout().write_all(&serial_buf[..t]).unwrap();
                            },
                            Err(ref e) if e.kind() == io::ErrorKind:: TimedOut => (),
                            Err(e) => {
                                eprintln!("{:?}", e);
                                break;
                            }
                        }
                        // io::stdout().write_all(&serial_buf[..t]).unwrap();
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

