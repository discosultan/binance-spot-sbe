#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrdType {
    Market = 49_u8,
    Limit = 50_u8,
    Stop = 51_u8,
    StopLimit = 52_u8,
    Pegged = 80_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for OrdType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::Market,
            50_u8 => Self::Limit,
            51_u8 => Self::Stop,
            52_u8 => Self::StopLimit,
            80_u8 => Self::Pegged,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<OrdType> for u8 {
    #[inline]
    fn from(v: OrdType) -> Self {
        match v {
            OrdType::Market => 49_u8,
            OrdType::Limit => 50_u8,
            OrdType::Stop => 51_u8,
            OrdType::StopLimit => 52_u8,
            OrdType::Pegged => 80_u8,
            OrdType::NonRepresentable => 126_u8,
            OrdType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for OrdType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Market" => Ok(Self::Market),
            "Limit" => Ok(Self::Limit),
            "Stop" => Ok(Self::Stop),
            "StopLimit" => Ok(Self::StopLimit),
            "Pegged" => Ok(Self::Pegged),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrdType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Market => write!(f, "Market"),
            Self::Limit => write!(f, "Limit"),
            Self::Stop => write!(f, "Stop"),
            Self::StopLimit => write!(f, "StopLimit"),
            Self::Pegged => write!(f, "Pegged"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
