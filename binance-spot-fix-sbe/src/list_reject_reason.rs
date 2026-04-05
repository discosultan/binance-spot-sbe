#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ListRejectReason {
    Other = 0x63_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ListRejectReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x63_u8 => Self::Other,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ListRejectReason> for u8 {
    #[inline]
    fn from(v: ListRejectReason) -> Self {
        match v {
            ListRejectReason::Other => 0x63_u8,
            ListRejectReason::NonRepresentable => 0xfe_u8,
            ListRejectReason::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for ListRejectReason {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Other" => Ok(Self::Other),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ListRejectReason {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
