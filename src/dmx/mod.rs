pub extern crate serialport as serial;

use std::result::Result;

#[derive(Debug)]
pub enum DmxError {
    IoError(serial::Error),
}

pub trait DmxTransmitter {
    fn dmx_frame(&self, data: &[u8]) -> Result<(), DmxError>;
    fn dmx_break(&self) -> Result<(), DmxError>;
}

impl<T: serial::SerialPort> DmxTransmitter for T
{
    fn dmx_frame(&self, data: &[u8]) -> Result<(), DmxError> {
        Ok(())
    }

    fn dmx_break(&self) -> Result<(), DmxError> {
        Ok(())
    }
}
