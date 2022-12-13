use std::io::{self, Write};
use std::time::Duration;

use serialport::{DataBits,StopBits};

fn main() {
    let port_name = "/dev/ttyUSB0";
    let test_data = [255u8; 512];

    let builder = serialport::new(port_name, 250_000)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight);

    let mut port = builder.open().unwrap_or_else(|e| {
        panic!("Failed to open \"{}\". Error: {}", port_name, e);
    });

    loop {
        port.set_break().unwrap();
        port.clear_break().unwrap();
        match port.write(&test_data) {
            Ok(_) => {
                println!("Ok");
            }

            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                println!("Timed out");
            }

            Err(e) => eprintln!("{:?}", e),
        }
    }

    //port.set_break().unwrap();
    //port.clear_break().unwrap();
}
