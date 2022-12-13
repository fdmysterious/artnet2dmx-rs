use std::io::{self, Write};
use std::time::Duration;

use serialport::{DataBits,StopBits};

fn main() {
    let test_str  = String::from("Hello world!\r\n");
    let port_name = "/dev/ttyUSB0";

    let builder = serialport::new(port_name, 115_200)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight);

    let mut port = builder.open_native().unwrap_or_else(|e| {
        panic!("Failed to open \"{}\". Error: {}", port_name, e);
    });

    loop {
        match port.write(test_str.as_bytes()) {
            Ok(_) => {
                println!("Ok");
            }

            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                println!("Timed out");
            }

            Err(e) => eprintln!("{:?}", e),
        }

        std::thread::sleep(Duration::from_millis(1000));
    }
}
