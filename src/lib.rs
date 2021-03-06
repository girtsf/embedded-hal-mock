extern crate embedded_hal as hal;

use std::io::{self, Read};

#[derive(Debug)]
pub enum MockError {
    Io(io::Error),
}

impl From<io::Error> for MockError {
    fn from(e: io::Error) -> Self {
        MockError::Io(e)
    }
}

pub struct I2cMock<'a> {
    data: &'a [u8],
}

impl<'a> I2cMock<'a> {
    pub fn new() -> Self {
        I2cMock {
            data: &[],
        }
    }

    /// Set data that will be read by `read()`.
    pub fn set_read_data(&mut self, data: &'a [u8]) {
        self.data = data;
    }
}

impl<'a> hal::blocking::i2c::Read for I2cMock<'a> {
    type Error = MockError;

    fn read(&mut self, _address: u8, mut buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.data.read(&mut buffer)?;
        Ok(())
    }
}

impl<'a> hal::blocking::i2c::Write for I2cMock<'a> {
    type Error = MockError;

    fn write(&mut self, _address: u8, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a> hal::blocking::i2c::WriteRead for I2cMock<'a> {
    type Error = MockError;

    fn write_read(
        &mut self,
        _address: u8,
        _bytes: &[u8],
        mut buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.data.read(&mut buffer)?;
        Ok(())
    }
}

pub struct DelayMockNoop;

macro_rules! impl_delay_us {
    ($type:ty) => {
        impl hal::blocking::delay::DelayUs<$type> for DelayMockNoop {
            /// A no-op delay implementation.
            fn delay_us(&mut self, _n: $type) { }
        }
    }
}

impl_delay_us!(u8);
impl_delay_us!(u16);
impl_delay_us!(u32);
impl_delay_us!(u64);

macro_rules! impl_delay_ms {
    ($type:ty) => {
        impl hal::blocking::delay::DelayMs<$type> for DelayMockNoop {
            /// A no-op delay implementation.
            fn delay_ms(&mut self, _n: $type) { }
        }
    }
}

impl_delay_ms!(u8);
impl_delay_ms!(u16);
impl_delay_ms!(u32);
impl_delay_ms!(u64);

#[cfg(test)]
mod tests {
    use super::*;

    use hal::blocking::i2c::Read;

    #[test]
    fn i2c_read_no_data_set() {
        let mut i2c = I2cMock::new();
        let mut buf = [0; 3];
        i2c.read(0, &mut buf).unwrap();
        assert_eq!(buf, [0; 3]);
    }

    #[test]
    fn i2c_read_some_data_set() {
        let mut i2c = I2cMock::new();
        let mut buf = [0; 3];
        i2c.set_read_data(&[1, 2]);
        i2c.read(0, &mut buf).unwrap();
        assert_eq!(buf, [1, 2, 0]);
    }

    #[test]
    fn i2c_read_too_much_data_set() {
        let mut i2c = I2cMock::new();
        let mut buf = [0; 3];
        i2c.set_read_data(&[1, 2, 3, 4]);
        i2c.read(0, &mut buf).unwrap();
        assert_eq!(buf, [1, 2, 3]);
    }
}
