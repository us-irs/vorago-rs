// UART A pins

use crate::{
    FunctionSelect,
    pins::{
        Pa2, Pa3, Pa8, Pa9, Pa16, Pa17, Pa18, Pa19, Pa26, Pa27, Pa30, Pa31, Pb6, Pb7, Pb8, Pb9,
        Pb18, Pb19, Pb20, Pb21, Pb22, Pb23, Pin,
    },
};

use super::{Bank, RxPin0, RxPin1, TxPin0, TxPin1};

impl TxPin0 for Pin<Pa9> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl RxPin0 for Pin<Pa8> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel2;
}

impl TxPin0 for Pin<Pa17> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin0 for Pin<Pa16> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin0 for Pin<Pa31> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin0 for Pin<Pa30> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin0 for Pin<Pb9> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl RxPin0 for Pin<Pb8> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}

impl TxPin0 for Pin<Pb23> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl RxPin0 for Pin<Pb22> {
    const BANK: Bank = Bank::Uart0;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}

// UART B pins

impl TxPin1 for Pin<Pa3> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl RxPin1 for Pin<Pa2> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel2;
}

impl TxPin1 for Pin<Pa19> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin1 for Pin<Pa18> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin1 for Pin<Pa27> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}
impl RxPin1 for Pin<Pa26> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl TxPin1 for Pin<Pb7> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl RxPin1 for Pin<Pb6> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}

impl TxPin1 for Pin<Pb19> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl RxPin1 for Pin<Pb18> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel2;
}

impl TxPin1 for Pin<Pb21> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl RxPin1 for Pin<Pb20> {
    const BANK: Bank = Bank::Uart1;
    const FUNC_SEL: FunctionSelect = FunctionSelect::Sel1;
}
