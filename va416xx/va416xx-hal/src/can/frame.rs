pub use embedded_can::{ExtendedId, Id, StandardId};

/// The given data slice is larger than the 8 bytes a CAN frame can hold.
#[derive(Debug, thiserror::Error)]
#[error("invalid data size error {0}")]
pub struct InvalidDataSizeError(usize);

/// A regular CAN data frame.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CanFrameNormal {
    id: embedded_can::Id,
    size: usize,
    data: [u8; 8],
}

impl CanFrameNormal {
    /// Create a new data frame with the given ID and data.
    pub fn new(id: embedded_can::Id, data: &[u8]) -> Result<Self, InvalidDataSizeError> {
        if data.len() > 8 {
            return Err(InvalidDataSizeError(data.len()));
        }
        let size = data.len();
        let mut data_array = [0; 8];
        data_array[0..size].copy_from_slice(data);
        Ok(Self {
            id,
            size,
            data: data_array,
        })
    }

    /// Identifier of this frame.
    #[inline]
    pub fn id(&self) -> embedded_can::Id {
        self.id
    }

    /// Data payload of this frame.
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data[0..self.dlc()]
    }

    /// Data length code of this frame.
    #[inline]
    pub fn dlc(&self) -> usize {
        self.size
    }
}

/// A remote transmission request (RTR) CAN frame.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CanFrameRtr {
    id: embedded_can::Id,
    dlc: usize,
}

impl CanFrameRtr {
    /// Create a new remote frame with the given ID and requested data length code.
    pub fn new(id: embedded_can::Id, dlc: usize) -> Self {
        Self { id, dlc }
    }

    /// Identifier of this frame.
    pub fn id(&self) -> embedded_can::Id {
        self.id
    }

    /// Requested data length code of this frame.
    pub fn dlc(&self) -> usize {
        self.dlc
    }
}

/// A CAN frame, either a regular data frame or a remote transmission request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanFrame {
    /// A regular data frame.
    Normal(CanFrameNormal),
    /// A remote transmission request frame.
    Rtr(CanFrameRtr),
}

impl From<CanFrameNormal> for CanFrame {
    fn from(value: CanFrameNormal) -> Self {
        Self::Normal(value)
    }
}

impl From<CanFrameRtr> for CanFrame {
    fn from(value: CanFrameRtr) -> Self {
        Self::Rtr(value)
    }
}

impl embedded_can::Frame for CanFrame {
    fn new(id: impl Into<embedded_can::Id>, data: &[u8]) -> Option<Self> {
        if data.len() > 8 {
            return None;
        }
        let id: embedded_can::Id = id.into();
        Some(Self::Normal(CanFrameNormal::new(id, data).unwrap()))
    }

    fn new_remote(id: impl Into<embedded_can::Id>, dlc: usize) -> Option<Self> {
        let id: embedded_can::Id = id.into();
        Some(Self::Rtr(CanFrameRtr::new(id, dlc)))
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            embedded_can::Id::Extended(_) => true,
            embedded_can::Id::Standard(_) => false,
        }
    }

    fn is_remote_frame(&self) -> bool {
        match self {
            CanFrame::Normal(_) => false,
            CanFrame::Rtr(_) => true,
        }
    }

    fn id(&self) -> embedded_can::Id {
        match self {
            CanFrame::Normal(can_frame_normal) => can_frame_normal.id(),
            CanFrame::Rtr(can_frame_rtr) => can_frame_rtr.id(),
        }
    }

    fn dlc(&self) -> usize {
        match self {
            CanFrame::Normal(can_frame_normal) => can_frame_normal.dlc(),
            CanFrame::Rtr(can_frame_rtr) => can_frame_rtr.dlc(),
        }
    }

    fn data(&self) -> &[u8] {
        match self {
            CanFrame::Normal(can_frame_normal) => can_frame_normal.data(),
            CanFrame::Rtr(_) => &[],
        }
    }
}
