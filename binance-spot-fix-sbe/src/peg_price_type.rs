#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PegPriceType {
    MarketPeg = 52_u8,
    PrimaryPeg = 53_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for PegPriceType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            52_u8 => Self::MarketPeg,
            53_u8 => Self::PrimaryPeg,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<PegPriceType> for u8 {
    #[inline]
    fn from(v: PegPriceType) -> Self {
        match v {
            PegPriceType::MarketPeg => 52_u8,
            PegPriceType::PrimaryPeg => 53_u8,
            PegPriceType::NonRepresentable => 126_u8,
            PegPriceType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for PegPriceType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "MarketPeg" => Ok(Self::MarketPeg),
            "PrimaryPeg" => Ok(Self::PrimaryPeg),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for PegPriceType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MarketPeg => write!(f, "MarketPeg"),
            Self::PrimaryPeg => write!(f, "PrimaryPeg"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
