#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ListTriggerType {
    Activated = 49_u8,
    PartiallyFilled = 50_u8,
    Filled = 51_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for ListTriggerType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::Activated,
            50_u8 => Self::PartiallyFilled,
            51_u8 => Self::Filled,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ListTriggerType> for u8 {
    #[inline]
    fn from(v: ListTriggerType) -> Self {
        match v {
            ListTriggerType::Activated => 49_u8,
            ListTriggerType::PartiallyFilled => 50_u8,
            ListTriggerType::Filled => 51_u8,
            ListTriggerType::NonRepresentable => 126_u8,
            ListTriggerType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for ListTriggerType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Activated" => Ok(Self::Activated),
            "PartiallyFilled" => Ok(Self::PartiallyFilled),
            "Filled" => Ok(Self::Filled),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ListTriggerType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Activated => write!(f, "Activated"),
            Self::PartiallyFilled => write!(f, "PartiallyFilled"),
            Self::Filled => write!(f, "Filled"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
