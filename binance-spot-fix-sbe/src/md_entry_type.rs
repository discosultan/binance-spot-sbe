#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MdEntryType {
    Bid = 48_u8,
    Offer = 49_u8,
    Trade = 50_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for MdEntryType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::Bid,
            49_u8 => Self::Offer,
            50_u8 => Self::Trade,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<MdEntryType> for u8 {
    #[inline]
    fn from(v: MdEntryType) -> Self {
        match v {
            MdEntryType::Bid => 48_u8,
            MdEntryType::Offer => 49_u8,
            MdEntryType::Trade => 50_u8,
            MdEntryType::NonRepresentable => 126_u8,
            MdEntryType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for MdEntryType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Bid" => Ok(Self::Bid),
            "Offer" => Ok(Self::Offer),
            "Trade" => Ok(Self::Trade),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MdEntryType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bid => write!(f, "Bid"),
            Self::Offer => write!(f, "Offer"),
            Self::Trade => write!(f, "Trade"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
