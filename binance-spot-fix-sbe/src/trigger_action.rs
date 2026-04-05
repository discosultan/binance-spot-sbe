#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TriggerAction {
    Activate = 49_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for TriggerAction {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::Activate,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<TriggerAction> for u8 {
    #[inline]
    fn from(v: TriggerAction) -> Self {
        match v {
            TriggerAction::Activate => 49_u8,
            TriggerAction::NonRepresentable => 126_u8,
            TriggerAction::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for TriggerAction {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Activate" => Ok(Self::Activate),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for TriggerAction {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Activate => write!(f, "Activate"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
