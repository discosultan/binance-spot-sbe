#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TriggerType {
    PriceMovement = 52_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for TriggerType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            52_u8 => Self::PriceMovement,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<TriggerType> for u8 {
    #[inline]
    fn from(v: TriggerType) -> Self {
        match v {
            TriggerType::PriceMovement => 52_u8,
            TriggerType::NonRepresentable => 126_u8,
            TriggerType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for TriggerType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "PriceMovement" => Ok(Self::PriceMovement),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for TriggerType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PriceMovement => write!(f, "PriceMovement"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
