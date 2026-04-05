#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MassCancelResponse {
    CancelRequestRejected = 48_u8,
    CancelSymbolOrders = 49_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for MassCancelResponse {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::CancelRequestRejected,
            49_u8 => Self::CancelSymbolOrders,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<MassCancelResponse> for u8 {
    #[inline]
    fn from(v: MassCancelResponse) -> Self {
        match v {
            MassCancelResponse::CancelRequestRejected => 48_u8,
            MassCancelResponse::CancelSymbolOrders => 49_u8,
            MassCancelResponse::NonRepresentable => 126_u8,
            MassCancelResponse::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for MassCancelResponse {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "CancelRequestRejected" => Ok(Self::CancelRequestRejected),
            "CancelSymbolOrders" => Ok(Self::CancelSymbolOrders),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MassCancelResponse {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CancelRequestRejected => write!(f, "CancelRequestRejected"),
            Self::CancelSymbolOrders => write!(f, "CancelSymbolOrders"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
