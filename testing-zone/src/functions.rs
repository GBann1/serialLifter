use serialport::{DataBits, SerialPort};
use std::time::Duration;

use std::io::{self, BufReader, BufRead};

pub fn read_data(port: &mut Box<dyn SerialPort>) -> Result<(), io::Error> {
    let mut reader = BufReader::new(port);
    let mut line = String::new();
    let mut stored_reading = String::new();

    loop {
        let bytes_ = reader.read_line(&mut line)?;
        if line != stored_reading {
            print!("Data: {}", line);
            stored_reading = line.clone();
        }
        line.clear();
    }
}

pub fn setup_connection() {
    let port_name = "COM9"; // port for my computer only. change
    let baud_rate = 1200;
    println!("Connecting on {} with {} baud.", &port_name, &baud_rate);
    match serialport::new(port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .open(){
            Ok(mut port) => {
                let _ = read_data(&mut port);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
}