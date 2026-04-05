#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MassCancelRequestType {
    CancelSymbolOrders = 49_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for MassCancelRequestType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::CancelSymbolOrders,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<MassCancelRequestType> for u8 {
    #[inline]
    fn from(v: MassCancelRequestType) -> Self {
        match v {
            MassCancelRequestType::CancelSymbolOrders => 49_u8,
            MassCancelRequestType::NonRepresentable => 126_u8,
            MassCancelRequestType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for MassCancelRequestType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "CancelSymbolOrders" => Ok(Self::CancelSymbolOrders),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MassCancelRequestType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CancelSymbolOrders => write!(f, "CancelSymbolOrders"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
