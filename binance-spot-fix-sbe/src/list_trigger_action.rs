#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ListTriggerAction {
    Release = 49_u8,
    Cancel = 50_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for ListTriggerAction {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::Release,
            50_u8 => Self::Cancel,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ListTriggerAction> for u8 {
    #[inline]
    fn from(v: ListTriggerAction) -> Self {
        match v {
            ListTriggerAction::Release => 49_u8,
            ListTriggerAction::Cancel => 50_u8,
            ListTriggerAction::NonRepresentable => 126_u8,
            ListTriggerAction::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for ListTriggerAction {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Release" => Ok(Self::Release),
            "Cancel" => Ok(Self::Cancel),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ListTriggerAction {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Release => write!(f, "Release"),
            Self::Cancel => write!(f, "Cancel"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
