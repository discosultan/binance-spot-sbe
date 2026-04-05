#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderCancelRequestAndNewOrderSingleMode {
    StopOnFailure = 0x1_u8,
    AllowFailure = 0x2_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderCancelRequestAndNewOrderSingleMode {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::StopOnFailure,
            0x2_u8 => Self::AllowFailure,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<OrderCancelRequestAndNewOrderSingleMode> for u8 {
    #[inline]
    fn from(v: OrderCancelRequestAndNewOrderSingleMode) -> Self {
        match v {
            OrderCancelRequestAndNewOrderSingleMode::StopOnFailure => 0x1_u8,
            OrderCancelRequestAndNewOrderSingleMode::AllowFailure => 0x2_u8,
            OrderCancelRequestAndNewOrderSingleMode::NonRepresentable => 0xfe_u8,
            OrderCancelRequestAndNewOrderSingleMode::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for OrderCancelRequestAndNewOrderSingleMode {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "StopOnFailure" => Ok(Self::StopOnFailure),
            "AllowFailure" => Ok(Self::AllowFailure),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrderCancelRequestAndNewOrderSingleMode {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StopOnFailure => write!(f, "StopOnFailure"),
            Self::AllowFailure => write!(f, "AllowFailure"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
