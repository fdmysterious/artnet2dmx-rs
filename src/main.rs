use std::time::Duration;
use std::array;

mod dmx;
use dmx::DmxTransmitter;

fn main() {
    let port_name           = "/dev/ttyUSB0";
    let test_data: [u8;511] = array::from_fn(|i| (i%256) as u8);

    let test_port = dmx::from_path(port_name).unwrap_or_else(|e| {
        panic!("Failed to open serial port: {:?}", e);
    });

    loop {
        test_port.send_frame(&test_data).unwrap();
    }

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
