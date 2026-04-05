#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CxlRejResponseTo {
    OrderCancelRequest = 49_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for CxlRejResponseTo {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::OrderCancelRequest,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<CxlRejResponseTo> for u8 {
    #[inline]
    fn from(v: CxlRejResponseTo) -> Self {
        match v {
            CxlRejResponseTo::OrderCancelRequest => 49_u8,
            CxlRejResponseTo::NonRepresentable => 126_u8,
            CxlRejResponseTo::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for CxlRejResponseTo {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "OrderCancelRequest" => Ok(Self::OrderCancelRequest),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for CxlRejResponseTo {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderCancelRequest => write!(f, "OrderCancelRequest"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
