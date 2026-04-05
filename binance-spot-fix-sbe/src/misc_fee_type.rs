#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MiscFeeType {
    ExchangeFees = 0x4_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for MiscFeeType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x4_u8 => Self::ExchangeFees,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<MiscFeeType> for u8 {
    #[inline]
    fn from(v: MiscFeeType) -> Self {
        match v {
            MiscFeeType::ExchangeFees => 0x4_u8,
            MiscFeeType::NonRepresentable => 0xfe_u8,
            MiscFeeType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for MiscFeeType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "ExchangeFees" => Ok(Self::ExchangeFees),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MiscFeeType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ExchangeFees => write!(f, "ExchangeFees"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
