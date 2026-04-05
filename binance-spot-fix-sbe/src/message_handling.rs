#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MessageHandling {
    Unordered = 0x1_u8,
    Sequential = 0x2_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for MessageHandling {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Unordered,
            0x2_u8 => Self::Sequential,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<MessageHandling> for u8 {
    #[inline]
    fn from(v: MessageHandling) -> Self {
        match v {
            MessageHandling::Unordered => 0x1_u8,
            MessageHandling::Sequential => 0x2_u8,
            MessageHandling::NonRepresentable => 0xfe_u8,
            MessageHandling::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for MessageHandling {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Unordered" => Ok(Self::Unordered),
            "Sequential" => Ok(Self::Sequential),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MessageHandling {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unordered => write!(f, "Unordered"),
            Self::Sequential => write!(f, "Sequential"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
