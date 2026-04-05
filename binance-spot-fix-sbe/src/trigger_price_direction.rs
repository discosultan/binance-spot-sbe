#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TriggerPriceDirection {
    Up = 85_u8,
    Down = 68_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for TriggerPriceDirection {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            85_u8 => Self::Up,
            68_u8 => Self::Down,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<TriggerPriceDirection> for u8 {
    #[inline]
    fn from(v: TriggerPriceDirection) -> Self {
        match v {
            TriggerPriceDirection::Up => 85_u8,
            TriggerPriceDirection::Down => 68_u8,
            TriggerPriceDirection::NonRepresentable => 126_u8,
            TriggerPriceDirection::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for TriggerPriceDirection {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Up" => Ok(Self::Up),
            "Down" => Ok(Self::Down),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for TriggerPriceDirection {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
