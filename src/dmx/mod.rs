pub extern crate serialport as serial;

use std::result::Result as StdResult;
use std::io;

// 8<----------------------------------

#[derive(Debug)]
pub enum DmxError {
    SerialError(serial::Error),
    IoError(io::Error),
}

impl From<serial::Error> for DmxError {
    fn from(err: serial::Error) -> Self {
        Self::SerialError(err)
    }
}

impl From<io::Error> for DmxError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

// 8<----------------------------------

type Result<T> = StdResult<T, DmxError>;

// 8<----------------------------------

pub trait DmxTransmitter{
    fn do_break(&self) -> Result<()>;
    fn send_data(&mut self, buf: &[u8]) -> Result<()>;
    fn flush(&mut self) -> Result<()>;

    fn send_frame(&mut self, buf: &[u8]) -> Result<()> {
        const start_code: [u8;1] = [0xFF];

        self.do_break()?;
        self.send_data(&start_code)?;
        self.send_data(buf)?;
        self.flush()?;

        Ok(())
    }
}

// 8<----------------------------------

pub struct SerialDmxTransmitter<T: serial::SerialPort> {
    port: T,
}

impl<T: serial::SerialPort> SerialDmxTransmitter<T> {
    fn attach_to_port(port: T) -> Self {
        Self {
            port
        }
    }
}

impl<T: serial::SerialPort> DmxTransmitter for SerialDmxTransmitter<T> {
    fn do_break(&self) -> Result<()> {
        self.port.set_break()?;
        self.port.clear_break()?;
        Ok(())
    }
    
    fn send_data(&mut self, buf: &[u8]) -> Result<()> {
        self.port.write(buf)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.port.flush()?;
        Ok(())
    }
}

// 8<----------------------------------

pub fn from_path(path: &str) -> Result<SerialDmxTransmitter<serial::TTYPort>> {
    let builder = serial::new(path, 250_000)
        .stop_bits(serial::StopBits::Two)
        .data_bits(serial::DataBits::Eight);

    Ok(SerialDmxTransmitter::attach_to_port(
        builder.open_native()?
    ))
}
