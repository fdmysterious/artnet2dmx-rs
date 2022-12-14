use std::time::Duration;
use std::array;


mod dmx;
use crate::dmx::serial::{self, DataBits, StopBits};
use crate::dmx::DmxTransmitter;

fn main() {
    let port_name           = "/dev/ttyUSB0";
    let test_data: [u8;511] = array::from_fn(|i| (i%256) as u8);

    let builder = serial::new(port_name, 250_000)
        .stop_bits(StopBits::Two)
        .data_bits(DataBits::Eight);

    let mut port = builder.open_native().unwrap_or_else(|e| {
        panic!("Failed to open \"{}\". Error: {}", port_name, e);
    });

    println!("Test data: {:?}", test_data);

    port.dmx_break().unwrap();

    //loop {
        //port.set_break().unwrap();
        //port.clear_break().unwrap();
        //match port.write(&test_data) {
        //    Ok(_) => {}
        //    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
        //        println!("Timed out");
        //    }

        //    Err(e) => eprintln!("{:?}", e),
        //}
    //}

    //port.set_break().unwrap();
    //port.clear_break().unwrap();
}
