use serialport::{DataBits, SerialPort};
use std::time::Duration;
use std::thread;
use std::io::{self, BufReader, BufRead};
use tauri::{Manager, AppHandle};

pub fn read_data(port: &mut Box<dyn SerialPort>, app: AppHandle) -> Result<(), io::Error> {
    let mut reader = BufReader::new(port);
    let mut line = String::new();
    let mut stored_reading = String::new();
    loop {
        let _bytes = reader.read_line(&mut line)?;
        if line != stored_reading {
            let _ = app.emit("new_reading", Some(line.clone())).unwrap();
            stored_reading = line.clone();
        }
        line.clear();
    }
}

pub fn setup_connection(app: AppHandle) {
    // Change these for your scale
    let port_name = "COM9";
    let baud_rate = 1200;
    println!("Connecting on {} with {} baud.", &port_name, &baud_rate);

    match serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(80))
        // .data_bits(DataBits::Eight)
        .open(){
            Ok(mut port) => {
                let _ = read_data(&mut port, app);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
}