use std::io::{self, Write};
use std::time::Duration;

pub fn docs() {
    // THIS COM PORT IS FOR MY COMPUTER - CHANGE
    let port_name = "COM9";
    let baud_rate = 9600;

    // Open the port with the name & baud rate
    let port = serialport::new(port_name, baud_rate)
        // Give it 10 ms to open the port before failure
        .timeout(Duration::from_millis(10))
        // Open the port
        .open();

    // Find the port
    match port {
        // if the port exists make it mutable
        Ok(mut port) => {
            // define the buffer as a vector of u8 values (are u8 == to 0x0A?)
            // Make that vector first defined as an array of 14 zeroes
            let mut serial_buf: Vec<u8> = vec![0; 14];  // NOTE: I need to be able to using \n only, not using buffers
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                // Read in the data from the port as mutable slices
                match port.read(serial_buf.as_mut_slice()) {    // What determines the end of a slice??
                    Ok(t) => {
                        // Without formatting text, gather the data from the reference to the buffer and send it to the console
                        io::stdout().write_all(&serial_buf[..t]).unwrap();
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