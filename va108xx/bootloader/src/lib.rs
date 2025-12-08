#![no_std]

/// Simple trait which makes swapping the NVM easier. NVMs only need to implement this interface.
pub trait NvmInterface {
    fn write(&mut self, address: usize, data: &[u8]);
    fn read(&mut self, address: usize, buf: &mut [u8]);
    fn verify(&mut self, address: usize, data: &[u8]) -> bool;
}
