#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ContingencyType {
    OneCancelsTheOther = 0x1_u8,
    OneTriggersTheOther = 0x2_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ContingencyType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::OneCancelsTheOther,
            0x2_u8 => Self::OneTriggersTheOther,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ContingencyType> for u8 {
    #[inline]
    fn from(v: ContingencyType) -> Self {
        match v {
            ContingencyType::OneCancelsTheOther => 0x1_u8,
            ContingencyType::OneTriggersTheOther => 0x2_u8,
            ContingencyType::NonRepresentable => 0xfe_u8,
            ContingencyType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for ContingencyType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "OneCancelsTheOther" => Ok(Self::OneCancelsTheOther),
            "OneTriggersTheOther" => Ok(Self::OneTriggersTheOther),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ContingencyType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OneCancelsTheOther => write!(f, "OneCancelsTheOther"),
            Self::OneTriggersTheOther => write!(f, "OneTriggersTheOther"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
