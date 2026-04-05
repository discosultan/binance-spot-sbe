#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LimitResetIntervalResolution {
    Second = 115_u8,
    Minute = 109_u8,
    Hour = 104_u8,
    Day = 100_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for LimitResetIntervalResolution {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            115_u8 => Self::Second,
            109_u8 => Self::Minute,
            104_u8 => Self::Hour,
            100_u8 => Self::Day,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<LimitResetIntervalResolution> for u8 {
    #[inline]
    fn from(v: LimitResetIntervalResolution) -> Self {
        match v {
            LimitResetIntervalResolution::Second => 115_u8,
            LimitResetIntervalResolution::Minute => 109_u8,
            LimitResetIntervalResolution::Hour => 104_u8,
            LimitResetIntervalResolution::Day => 100_u8,
            LimitResetIntervalResolution::NonRepresentable => 126_u8,
            LimitResetIntervalResolution::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for LimitResetIntervalResolution {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Second" => Ok(Self::Second),
            "Minute" => Ok(Self::Minute),
            "Hour" => Ok(Self::Hour),
            "Day" => Ok(Self::Day),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for LimitResetIntervalResolution {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Second => write!(f, "Second"),
            Self::Minute => write!(f, "Minute"),
            Self::Hour => write!(f, "Hour"),
            Self::Day => write!(f, "Day"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
