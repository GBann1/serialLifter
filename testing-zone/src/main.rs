use std::time::Duration;
use std::io::{self, Write};
use std::io::{BufReader, BufRead};
use serialport::SerialPort;

fn read_data(port: &mut Box<dyn SerialPort>) -> Result<(), io::Error> {
    let mut reader = BufReader::new(port);
    let mut line = String::new();
    let mut stored_reading = String::new();
    loop {
        if let Ok(_) = reader.read_line(&mut line){
            if line != stored_reading {
                println!("Data: {}", line);
                io::stdout().flush().unwrap();
                stored_reading = line.clone();
            }
            line.clear();
        }
    }
}

fn main() {
    let port_name = "COM9"; // port for my computer only. change
    let baud_rate = 1200;
    println!("Trying to connect on {} with {} baud.", &port_name, &baud_rate);
    if let Ok(mut port) = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open(){
            let _ = read_data(&mut port);
        }
}

