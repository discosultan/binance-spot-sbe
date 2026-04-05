#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MatchType {
    OnePartyTradeReport = 49_u8,
    AutoMatch = 52_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for MatchType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::OnePartyTradeReport,
            52_u8 => Self::AutoMatch,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<MatchType> for u8 {
    #[inline]
    fn from(v: MatchType) -> Self {
        match v {
            MatchType::OnePartyTradeReport => 49_u8,
            MatchType::AutoMatch => 52_u8,
            MatchType::NonRepresentable => 126_u8,
            MatchType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for MatchType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "OnePartyTradeReport" => Ok(Self::OnePartyTradeReport),
            "AutoMatch" => Ok(Self::AutoMatch),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MatchType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OnePartyTradeReport => write!(f, "OnePartyTradeReport"),
            Self::AutoMatch => write!(f, "AutoMatch"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
