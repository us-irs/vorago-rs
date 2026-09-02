use arbitrary_int::u5;

pub(crate) mod asynch;

/// FIFO trigger level, shared by the SPI and UART drivers.
#[derive(Debug)]
pub struct TriggerLevel(arbitrary_int::UInt<u32, 5>);

impl TriggerLevel {
    /// Create a new trigger level.
    pub const fn new(value: u5) -> Self {
        TriggerLevel(arbitrary_int::UInt::<u32, 5>::new(value.value() as u32))
    }

    /// The raw trigger level value.
    pub const fn value(&self) -> u5 {
        u5::new(self.0.value() as u8)
    }
}

/// FIFO clear command, shared by the SPI and UART drivers.
#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct FifoClear {
    /// Clear the TX FIFO.
    #[bit(1, w)]
    tx_fifo: bool,
    /// Clear the RX FIFO.
    #[bit(0, w)]
    rx_fifo: bool,
}

impl FifoClear {
    /// Clears both the TX and RX FIFO.
    pub const ALL: Self = Self::builder()
        .with_tx_fifo(true)
        .with_rx_fifo(true)
        .build();
}
