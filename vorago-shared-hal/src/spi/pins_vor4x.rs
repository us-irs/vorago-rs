use crate::{
    FunctionSelect,
    gpio::{DynPinId, Pin, PinId},
    pins::{
        Pa0, Pa1, Pa2, Pa3, Pa4, Pa5, Pa6, Pa7, Pa8, Pa9, Pb0, Pb1, Pb2, Pb3, Pb4, Pb12, Pb13,
        Pb14, Pb15, Pc0, Pc1, Pc7, Pc8, Pc9, Pc10, Pc11, Pe5, Pe6, Pe7, Pe8, Pe9, Pe12, Pe13, Pe14,
        Pe15, Pf0, Pf1, Pg2, Pg3, Pg4,
    },
};

#[cfg(not(feature = "va41628"))]
use crate::pins::{Pb5, Pb6, Pb7, Pb8, Pb9, Pb10, Pb11, Pe10, Pe11, Pf2, Pf3, Pf4, Pf5, Pf6, Pf7};

use super::{Bank, HwChipSelectId, HwCsProvider, PinMiso, PinMosi, PinSck};

// SPI0

impl PinSck for Pin<Pb15> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl PinMosi for Pin<Pc1> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
impl PinMiso for Pin<Pc0> {
    const SPI_ID: Bank = Bank::Spi0;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}

hw_cs_pins!(
    Bank::Spi0,
    (Pb14, FunctionSelect::Sel1, HwChipSelectId::Id0),
    (Pb13, FunctionSelect::Sel1, HwChipSelectId::Id1),
    (Pb12, FunctionSelect::Sel1, HwChipSelectId::Id2),
);

#[cfg(not(feature = "va41628"))]
hw_cs_pins!(
    Bank::Spi0,
    (Pb11, FunctionSelect::Sel1, HwChipSelectId::Id3)
);

// SPI1

#[cfg(not(feature = "va41628"))]
impl PinSck for Pin<Pb8> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}
#[cfg(not(feature = "va41628"))]
impl PinMosi for Pin<Pb10> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}
#[cfg(not(feature = "va41628"))]
impl PinMiso for Pin<Pb9> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel3;
}

impl PinSck for Pin<Pc9> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl PinMosi for Pin<Pc11> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl PinMiso for Pin<Pc10> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}

impl PinSck for Pin<Pe13> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl PinMosi for Pin<Pe15> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl PinMiso for Pin<Pe14> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}

#[cfg(not(feature = "va41628"))]
impl PinSck for Pin<Pf3> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
#[cfg(not(feature = "va41628"))]
impl PinMosi for Pin<Pf5> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}
#[cfg(not(feature = "va41628"))]
impl PinMiso for Pin<Pf4> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel1;
}

impl PinSck for Pin<Pg3> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl PinMiso for Pin<Pg4> {
    const SPI_ID: Bank = Bank::Spi1;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}

hw_cs_pins!(
    Bank::Spi1,
    (Pb4, FunctionSelect::Sel3, HwChipSelectId::Id3),
    (Pb3, FunctionSelect::Sel3, HwChipSelectId::Id4),
    (Pb2, FunctionSelect::Sel3, HwChipSelectId::Id5),
    (Pb1, FunctionSelect::Sel3, HwChipSelectId::Id6),
    (Pb0, FunctionSelect::Sel3, HwChipSelectId::Id7),
    (Pc8, FunctionSelect::Sel2, HwChipSelectId::Id0),
    (Pc7, FunctionSelect::Sel2, HwChipSelectId::Id1),
    (Pe12, FunctionSelect::Sel2, HwChipSelectId::Id0),
    (Pe9, FunctionSelect::Sel2, HwChipSelectId::Id3),
    (Pe8, FunctionSelect::Sel2, HwChipSelectId::Id4),
    (Pe7, FunctionSelect::Sel3, HwChipSelectId::Id5),
    (Pe6, FunctionSelect::Sel3, HwChipSelectId::Id6),
    (Pe5, FunctionSelect::Sel3, HwChipSelectId::Id7),
    (Pg2, FunctionSelect::Sel2, HwChipSelectId::Id0),
);

#[cfg(not(feature = "va41628"))]
hw_cs_pins!(
    Bank::Spi1,
    (Pb7, FunctionSelect::Sel3, HwChipSelectId::Id0),
    (Pb6, FunctionSelect::Sel3, HwChipSelectId::Id1),
    (Pb5, FunctionSelect::Sel3, HwChipSelectId::Id2),
    (Pe11, FunctionSelect::Sel2, HwChipSelectId::Id1),
    (Pe10, FunctionSelect::Sel2, HwChipSelectId::Id2),
);

#[cfg(not(feature = "va41628"))]
hw_cs_multi_pin!(
    PinPf2Spi1HwCsId0,
    Pf2,
    Bank::Spi2,
    FunctionSelect::Sel1,
    HwChipSelectId::Id0
);

// SPI2

impl PinSck for Pin<Pa5> {
    const SPI_ID: Bank = Bank::Spi2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl PinMosi for Pin<Pa7> {
    const SPI_ID: Bank = Bank::Spi2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
impl PinMiso for Pin<Pa6> {
    const SPI_ID: Bank = Bank::Spi2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}

#[cfg(not(feature = "va41628"))]
impl PinSck for Pin<Pf5> {
    const SPI_ID: Bank = Bank::Spi2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
#[cfg(not(feature = "va41628"))]
impl PinMosi for Pin<Pf7> {
    const SPI_ID: Bank = Bank::Spi2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}
#[cfg(not(feature = "va41628"))]
impl PinMiso for Pin<Pf6> {
    const SPI_ID: Bank = Bank::Spi2;
    const FUN_SEL: FunctionSelect = FunctionSelect::Sel2;
}

hw_cs_pins!(
    Bank::Spi1,
    (Pa4, FunctionSelect::Sel2, HwChipSelectId::Id0),
    (Pa3, FunctionSelect::Sel2, HwChipSelectId::Id1),
    (Pa2, FunctionSelect::Sel2, HwChipSelectId::Id2),
    (Pa1, FunctionSelect::Sel2, HwChipSelectId::Id3),
    (Pa0, FunctionSelect::Sel2, HwChipSelectId::Id4),
    (Pa8, FunctionSelect::Sel2, HwChipSelectId::Id5),
    (Pa9, FunctionSelect::Sel2, HwChipSelectId::Id6),
    (Pf0, FunctionSelect::Sel2, HwChipSelectId::Id4),
    (Pf1, FunctionSelect::Sel2, HwChipSelectId::Id3),
);

#[cfg(not(feature = "va41628"))]
hw_cs_pins!(
    Bank::Spi1,
    (Pf3, FunctionSelect::Sel2, HwChipSelectId::Id1),
    (Pf4, FunctionSelect::Sel2, HwChipSelectId::Id0),
);

#[cfg(not(feature = "va41628"))]
hw_cs_multi_pin!(
    PinPf2Spi2HwCsId2,
    Pf2,
    Bank::Spi2,
    FunctionSelect::Sel2,
    HwChipSelectId::Id2
);
