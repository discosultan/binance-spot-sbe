#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ResponseMode {
    Everything = 0x1_u8,
    OnlyAcks = 0x2_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ResponseMode {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Everything,
            0x2_u8 => Self::OnlyAcks,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ResponseMode> for u8 {
    #[inline]
    fn from(v: ResponseMode) -> Self {
        match v {
            ResponseMode::Everything => 0x1_u8,
            ResponseMode::OnlyAcks => 0x2_u8,
            ResponseMode::NonRepresentable => 0xfe_u8,
            ResponseMode::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for ResponseMode {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Everything" => Ok(Self::Everything),
            "OnlyAcks" => Ok(Self::OnlyAcks),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ResponseMode {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Everything => write!(f, "Everything"),
            Self::OnlyAcks => write!(f, "OnlyAcks"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
