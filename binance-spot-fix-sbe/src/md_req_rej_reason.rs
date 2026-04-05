#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MdReqRejReason {
    DuplicateMdReqID = 49_u8,
    TooManySubscriptions = 50_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for MdReqRejReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::DuplicateMdReqID,
            50_u8 => Self::TooManySubscriptions,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<MdReqRejReason> for u8 {
    #[inline]
    fn from(v: MdReqRejReason) -> Self {
        match v {
            MdReqRejReason::DuplicateMdReqID => 49_u8,
            MdReqRejReason::TooManySubscriptions => 50_u8,
            MdReqRejReason::NonRepresentable => 126_u8,
            MdReqRejReason::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for MdReqRejReason {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "DuplicateMdReqID" => Ok(Self::DuplicateMdReqID),
            "TooManySubscriptions" => Ok(Self::TooManySubscriptions),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for MdReqRejReason {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DuplicateMdReqID => write!(f, "DuplicateMdReqID"),
            Self::TooManySubscriptions => write!(f, "TooManySubscriptions"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
