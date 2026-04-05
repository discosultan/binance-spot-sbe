#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PegMoveType {
    Fixed = 0x1_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for PegMoveType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Fixed,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<PegMoveType> for u8 {
    #[inline]
    fn from(v: PegMoveType) -> Self {
        match v {
            PegMoveType::Fixed => 0x1_u8,
            PegMoveType::NonRepresentable => 0xfe_u8,
            PegMoveType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for PegMoveType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Fixed" => Ok(Self::Fixed),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for PegMoveType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fixed => write!(f, "Fixed"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
