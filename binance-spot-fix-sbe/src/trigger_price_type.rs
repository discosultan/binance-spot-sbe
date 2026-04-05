#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TriggerPriceType {
    LastTrade = 50_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for TriggerPriceType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            50_u8 => Self::LastTrade,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<TriggerPriceType> for u8 {
    #[inline]
    fn from(v: TriggerPriceType) -> Self {
        match v {
            TriggerPriceType::LastTrade => 50_u8,
            TriggerPriceType::NonRepresentable => 126_u8,
            TriggerPriceType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for TriggerPriceType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "LastTrade" => Ok(Self::LastTrade),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for TriggerPriceType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LastTrade => write!(f, "LastTrade"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
