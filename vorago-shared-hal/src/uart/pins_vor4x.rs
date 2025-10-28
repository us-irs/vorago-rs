#[cfg(not(feature = "va41628"))]
use crate::pins::{Pc15, Pf8};
use crate::{
    FunctionSelect,
    gpio::Pin,
    pins::{Pa2, Pa3, Pb14, Pb15, Pc4, Pc5, Pc14, Pd11, Pd12, Pe2, Pe3, Pf9, Pf12, Pf13, Pg0, Pg1},
};

use super::{Bank, RxPin, TxPin};

// UART 0 pins

impl TxPin for Pin<Pa2> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin for Pin<Pa3> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin for Pin<Pc4> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl RxPin for Pin<Pc5> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}

impl TxPin for Pin<Pe2> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin for Pin<Pe3> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin for Pin<Pg0> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl RxPin for Pin<Pg1> {
    const BANK: Bank = Bank::Uart0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}

// UART 1 pins

impl TxPin for Pin<Pb14> {
    const BANK: Bank = Bank::Uart1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin for Pin<Pb15> {
    const BANK: Bank = Bank::Uart1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin for Pin<Pd11> {
    const BANK: Bank = Bank::Uart1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin for Pin<Pd12> {
    const BANK: Bank = Bank::Uart1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin for Pin<Pf12> {
    const BANK: Bank = Bank::Uart1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl RxPin for Pin<Pf13> {
    const BANK: Bank = Bank::Uart1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}

// UART 2 pins

impl TxPin for Pin<Pc14> {
    const BANK: Bank = Bank::Uart2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
#[cfg(not(feature = "va41628"))]
impl RxPin for Pin<Pc15> {
    const BANK: Bank = Bank::Uart2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}

#[cfg(not(feature = "va41628"))]
impl TxPin for Pin<Pf8> {
    const BANK: Bank = Bank::Uart2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl RxPin for Pin<Pf9> {
    const BANK: Bank = Bank::Uart2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
