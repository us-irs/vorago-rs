use core::{
    future::Future,
    sync::atomic::{AtomicU8, Ordering},
};

use crate::can::regs::BufferState;

use super::{
    regs::{DiagnosticRegister, InterruptClear, MmioCan, StatusPending},
    CanChannelLowLevel, CanFrame, CanId, InvalidBufferIndexError,
};

#[derive(Debug)]
pub enum TxChannelState {
    Unconfigured = 0,
    Idle = 1,
    TxDataFrame = 2,
    TxRtrTransmission = 3,
    TxRtrReception = 4,
    Finished = 5,
}

static TX_STATES: [AtomicU8; 15] = [const { AtomicU8::new(0) }; 15];
static TX_WAKERS: [embassy_sync::waitqueue::AtomicWaker; 15] =
    [const { embassy_sync::waitqueue::AtomicWaker::new() }; 15];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxEventId {
    /// Buffer state went from [BufferState::TxOnce] to [BufferState::TxNotActive].
    TxDataFrame,
    /// Buffer state went from [BufferState::TxOnce] to [BufferState::TxNotActive] for a remote
    /// frame (RTR bit set). Channel might be in reception mode [BufferState::RxReady] now.
    TxRemoteFrame,
    /// A response to a remote frame was performed successfully, and the buffer state went from
    /// [BufferState::TxOnceRtr] to [BufferState::TxRtr].
    RtrResponse,
    /// A remote frame was received and the transmission of a response frame was scheduled. The
    /// buffer state went from [BufferState::TxRtr] to [BufferState::TxOnceRtr].
    TransmitScheduling,
}

#[derive(Debug)]
pub enum InterruptResult {
    NoInterrupt,
    ReceivedFrame {
        channel_index: usize,
        frame: CanFrame,
    },
    TransmissionEvent {
        channel_index: usize,
        id: TxEventId,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterruptError {
    UnexpectedError,
    InvalidInterruptId(StatusPending),
    InvalidStatus(u8),
    UnexpectedState(BufferState),
    CanError(DiagnosticRegister),
}

/// This interrupt handler allow asynchronous transmission and reception of CAN frames.
///
/// This handler will re-configure a channel to [BufferState::RxReady] after successfull reception
/// of a frame without disabling the interrupts, assuming that the user wants to immediately
/// receive the next frame on the channel.
/// The user should re-configure the buffer state to [BufferState::RxNotActive] if the reception
/// should be disabled.
///
/// The handler will re-configure a channel to [BufferState::TxNotActive] instead of
/// [BufferState::RxReady] if the completed frame transmission was a remote frame and after
/// successfully having received a response to that remote frame. The assumption is that this
/// channel is used to request more frames. If the argument `reconfigure_tx_rtr_to_tx` is set to
/// true, the channel will automatically be configured back to [BufferState::TxNotActive] with
/// interrupts for the respective channel disabled after transmission of a remote frame.
///
/// The handler will not disable the interrupts realted to the TX RTR and TX RTR ONCE auto-response
/// functionality of the CAN peripheral. It will report the event type to the caller via the
/// [TxEventId] enumeration.
pub fn on_interrupt_can(
    id: CanId,
    reconfigure_tx_rtr_to_tx: bool,
) -> Result<InterruptResult, InterruptError> {
    let mut regs = unsafe { id.steal_regs() };
    // Check if any interrupts are enabled.
    let ie = regs.read_ien();
    if ie.raw_value() == 0 {
        return Ok(InterruptResult::NoInterrupt);
    }
    let pending_id = regs.read_status_pending();
    if pending_id.interrupt_id().is_none() {
        regs.write_iclr(InterruptClear::new_with_raw_value(0xFFFF_FFFF));
        return Err(InterruptError::InvalidInterruptId(pending_id));
    }
    match pending_id.interrupt_id().unwrap() {
        super::regs::CanInterruptId::None => Ok(InterruptResult::NoInterrupt),
        super::regs::CanInterruptId::Error => Err(InterruptError::CanError(regs.read_diag())),
        super::regs::CanInterruptId::Buffer(idx) => {
            let mut channel = unsafe { CanChannelLowLevel::steal_unchecked(id, idx) };
            let status = channel.read_state();
            if let Err(e) = status {
                let mut clr = InterruptClear::new_with_raw_value(0);
                clr.set_buffer(idx, true);
                regs.write_iclr(clr);
                regs.modify_ien(|mut val| {
                    val.set_buffer(idx, false);
                    val
                });
                return Err(InterruptError::InvalidStatus(e));
            }
            let buf_state = status.unwrap();
            if buf_state == BufferState::TxNotActive {
                let tx_state = TX_STATES[idx].load(Ordering::Relaxed);
                clear_and_disable_interrupt(&mut regs, idx);
                // Handle reading frames, updating states etc.
                if tx_state == TxChannelState::TxDataFrame as u8 {
                    // Transmission complete.
                    TX_STATES[idx].store(TxChannelState::Finished as u8, Ordering::Relaxed);
                    TX_WAKERS[idx].wake();
                    return Ok(InterruptResult::TransmissionEvent {
                        channel_index: idx,
                        id: TxEventId::TxDataFrame,
                    });
                }
            }
            if buf_state == BufferState::RxReady {
                let tx_state = TX_STATES[idx].load(Ordering::Relaxed);
                if tx_state == TxChannelState::TxRtrTransmission as u8 {
                    if reconfigure_tx_rtr_to_tx {
                        channel.write_state(BufferState::TxNotActive);
                        clear_and_disable_interrupt(&mut regs, idx);
                        // Transmission complete.
                        TX_STATES[idx].store(TxChannelState::Idle as u8, Ordering::Relaxed);
                    } else {
                        // Do not disable interrupt, channel is now used to receive the frame.
                        clear_interrupt(&mut regs, idx);
                        // Transmission complete.
                        TX_STATES[idx]
                            .store(TxChannelState::TxRtrReception as u8, Ordering::Relaxed);
                    }
                    TX_WAKERS[idx].wake();
                    return Ok(InterruptResult::TransmissionEvent {
                        channel_index: idx,
                        id: TxEventId::TxRemoteFrame,
                    });
                }
            }
            if buf_state == BufferState::RxOverrun || buf_state == BufferState::RxFull {
                let tx_state = TX_STATES[idx].load(Ordering::Relaxed);
                // Do not disable interrupt and assume continuous reception.
                clear_interrupt(&mut regs, idx);
                let frame = channel.read_frame_unchecked();
                if tx_state == TxChannelState::TxRtrReception as u8 {
                    // Reception of response complete. We can release the channel for TX (or RX)
                    // usage again.
                    TX_STATES[idx].store(TxChannelState::Idle as u8, Ordering::Relaxed);
                    channel.write_state(BufferState::TxNotActive);
                } else {
                    // Assume continous reception of frames.
                    channel.write_state(BufferState::RxReady);
                }
                return Ok(InterruptResult::ReceivedFrame {
                    channel_index: idx,
                    frame,
                });
            }
            if buf_state == BufferState::TxRtr {
                // Do not disable interrupt and assume continuous transmission.
                clear_interrupt(&mut regs, idx);
                return Ok(InterruptResult::TransmissionEvent {
                    channel_index: idx,
                    id: TxEventId::RtrResponse,
                });
            }
            if buf_state == BufferState::TxOnceRtr {
                // Do not disable interrupt and assume continuous transmission.
                clear_interrupt(&mut regs, idx);
                return Ok(InterruptResult::TransmissionEvent {
                    channel_index: idx,
                    id: TxEventId::TransmitScheduling,
                });
            }

            Err(InterruptError::UnexpectedState(buf_state))
        }
    }
}

#[inline(always)]
fn clear_interrupt(regs: &mut MmioCan<'static>, idx: usize) {
    let mut clr = InterruptClear::new_with_raw_value(0);
    clr.set_buffer(idx, true);
    regs.write_iclr(clr);
}

#[inline(always)]
fn clear_and_disable_interrupt(regs: &mut MmioCan<'static>, idx: usize) {
    clear_interrupt(regs, idx);
    regs.modify_ien(|mut val| {
        val.set_buffer(idx, false);
        val
    });
}

#[derive(Debug, thiserror::Error)]
#[error("all channels are unconfigured, none available for TX")]
pub struct AllTxChannelsUnconfiguredError;

pub struct CanTxFuture(usize);

impl Future for CanTxFuture {
    type Output = ();

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        TX_WAKERS[self.0].register(cx.waker());
        if TX_STATES[self.0].load(Ordering::Relaxed) == TxChannelState::Finished as u8 {
            TX_STATES[self.0].store(TxChannelState::Idle as u8, Ordering::Relaxed);
            return core::task::Poll::Ready(());
        }
        core::task::Poll::Pending
    }
}

impl CanTxFuture {
    pub fn new(frame: CanFrame) -> nb::Result<Self, AllTxChannelsUnconfiguredError> {
        let mut channel_is_free = [false; 15];
        let mut all_channels_unused = true;
        for (idx, state) in TX_STATES.iter().enumerate() {
            let state = state.load(Ordering::Relaxed);
            if state == TxChannelState::Idle as u8 {
                channel_is_free[idx] = true;
            }
            if state != TxChannelState::Unconfigured as u8 {
                all_channels_unused = false;
            }
        }
        if channel_is_free.iter().all(|&x| !x) {
            return Err(nb::Error::WouldBlock);
        }
        if all_channels_unused {
            return Err(nb::Error::Other(AllTxChannelsUnconfiguredError));
        }
        let free_channel_id = channel_is_free.iter().position(|&x| x).unwrap();
        let mut channel =
            unsafe { CanChannelLowLevel::steal_unchecked(CanId::Can0, free_channel_id) };
        TX_STATES[free_channel_id].store(TxChannelState::TxDataFrame as u8, Ordering::Relaxed);
        channel.write_state(BufferState::TxNotActive);
        channel.transmit_frame_unchecked(frame);
        channel.clear_interrupt();
        channel.enable_interrupt(true);
        channel.enable_error_interrupt(true);
        Ok(CanTxFuture(free_channel_id))
    }
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChannelConfigError {
    #[error("channel is busy")]
    Busy,
    #[error("invalid offset: {0}")]
    Offset(#[from] InvalidBufferIndexError),
}

pub struct CanTxAsync;

impl CanTxAsync {
    pub fn new(can: &mut super::Can) -> Self {
        can.clear_interrupts();
        can.enable_nvic_interrupt();
        CanTxAsync
    }

    pub fn configure_channel(&mut self, channel_idx: usize) -> Result<(), ChannelConfigError> {
        if channel_idx >= TX_STATES.len() {
            return Err(ChannelConfigError::Offset(InvalidBufferIndexError(
                channel_idx,
            )));
        }
        let state = TX_STATES[channel_idx].load(Ordering::Relaxed);
        if state != TxChannelState::Idle as u8 && state != TxChannelState::Unconfigured as u8 {
            return Err(ChannelConfigError::Busy);
        }
        TX_STATES[channel_idx].store(TxChannelState::Idle as u8, Ordering::Relaxed);
        Ok(())
    }

    /// Start a transmission and returns the future which can be polled to completion.
    pub fn start_transmit(
        &mut self,
        frame: CanFrame,
    ) -> nb::Result<CanTxFuture, AllTxChannelsUnconfiguredError> {
        CanTxFuture::new(frame)
    }

    /// Calls [Self::start_transmit] and awaits the returned future to completion immediately.
    pub async fn transmit(
        &mut self,
        frame: CanFrame,
    ) -> nb::Result<(), AllTxChannelsUnconfiguredError> {
        self.start_transmit(frame)?.await;
        Ok(())
    }
}
