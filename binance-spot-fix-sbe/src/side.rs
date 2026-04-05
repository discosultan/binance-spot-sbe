#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Side {
    Buy = 49_u8,
    Sell = 50_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for Side {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::Buy,
            50_u8 => Self::Sell,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<Side> for u8 {
    #[inline]
    fn from(v: Side) -> Self {
        match v {
            Side::Buy => 49_u8,
            Side::Sell => 50_u8,
            Side::NonRepresentable => 126_u8,
            Side::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for Side {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Buy" => Ok(Self::Buy),
            "Sell" => Ok(Self::Sell),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for Side {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Buy => write!(f, "Buy"),
            Self::Sell => write!(f, "Sell"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
