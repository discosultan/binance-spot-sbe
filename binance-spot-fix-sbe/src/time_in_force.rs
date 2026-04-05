#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TimeInForce {
    GoodTillCancel = 49_u8,
    ImmediateOrCancel = 51_u8,
    FillOrKill = 52_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for TimeInForce {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::GoodTillCancel,
            51_u8 => Self::ImmediateOrCancel,
            52_u8 => Self::FillOrKill,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<TimeInForce> for u8 {
    #[inline]
    fn from(v: TimeInForce) -> Self {
        match v {
            TimeInForce::GoodTillCancel => 49_u8,
            TimeInForce::ImmediateOrCancel => 51_u8,
            TimeInForce::FillOrKill => 52_u8,
            TimeInForce::NonRepresentable => 126_u8,
            TimeInForce::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for TimeInForce {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "GoodTillCancel" => Ok(Self::GoodTillCancel),
            "ImmediateOrCancel" => Ok(Self::ImmediateOrCancel),
            "FillOrKill" => Ok(Self::FillOrKill),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for TimeInForce {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GoodTillCancel => write!(f, "GoodTillCancel"),
            Self::ImmediateOrCancel => write!(f, "ImmediateOrCancel"),
            Self::FillOrKill => write!(f, "FillOrKill"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
