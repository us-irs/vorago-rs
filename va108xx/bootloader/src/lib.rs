#![no_std]

use core::convert::Infallible;

/// Simple trait which makes swapping the NVM easier. NVMs only need to implement this interface.
pub trait NvmInterface {
    fn write(&mut self, address: usize, data: &[u8]) -> Result<(), Infallible>;
    fn read(&mut self, address: usize, buf: &mut [u8]) -> Result<(), Infallible>;
    fn verify(&mut self, address: usize, data: &[u8]) -> Result<bool, Infallible>;
}
