#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrdRejReason {
    Other = 0x63_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrdRejReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x63_u8 => Self::Other,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<OrdRejReason> for u8 {
    #[inline]
    fn from(v: OrdRejReason) -> Self {
        match v {
            OrdRejReason::Other => 0x63_u8,
            OrdRejReason::NonRepresentable => 0xfe_u8,
            OrdRejReason::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for OrdRejReason {
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
impl core::fmt::Display for OrdRejReason {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Other => write!(f, "Other"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
