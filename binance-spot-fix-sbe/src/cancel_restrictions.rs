#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CancelRestrictions {
    OnlyNew = 0x1_u8,
    OnlyPartiallyFilled = 0x2_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for CancelRestrictions {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::OnlyNew,
            0x2_u8 => Self::OnlyPartiallyFilled,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<CancelRestrictions> for u8 {
    #[inline]
    fn from(v: CancelRestrictions) -> Self {
        match v {
            CancelRestrictions::OnlyNew => 0x1_u8,
            CancelRestrictions::OnlyPartiallyFilled => 0x2_u8,
            CancelRestrictions::NonRepresentable => 0xfe_u8,
            CancelRestrictions::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for CancelRestrictions {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "OnlyNew" => Ok(Self::OnlyNew),
            "OnlyPartiallyFilled" => Ok(Self::OnlyPartiallyFilled),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for CancelRestrictions {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OnlyNew => write!(f, "OnlyNew"),
            Self::OnlyPartiallyFilled => write!(f, "OnlyPartiallyFilled"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
