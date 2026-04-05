#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrdStatus {
    New = 48_u8,
    PartiallyFilled = 49_u8,
    Filled = 50_u8,
    Canceled = 52_u8,
    PendingCancel = 54_u8,
    Rejected = 56_u8,
    PendingNew = 65_u8,
    Expired = 67_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for OrdStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::New,
            49_u8 => Self::PartiallyFilled,
            50_u8 => Self::Filled,
            52_u8 => Self::Canceled,
            54_u8 => Self::PendingCancel,
            56_u8 => Self::Rejected,
            65_u8 => Self::PendingNew,
            67_u8 => Self::Expired,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<OrdStatus> for u8 {
    #[inline]
    fn from(v: OrdStatus) -> Self {
        match v {
            OrdStatus::New => 48_u8,
            OrdStatus::PartiallyFilled => 49_u8,
            OrdStatus::Filled => 50_u8,
            OrdStatus::Canceled => 52_u8,
            OrdStatus::PendingCancel => 54_u8,
            OrdStatus::Rejected => 56_u8,
            OrdStatus::PendingNew => 65_u8,
            OrdStatus::Expired => 67_u8,
            OrdStatus::NonRepresentable => 126_u8,
            OrdStatus::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for OrdStatus {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "New" => Ok(Self::New),
            "PartiallyFilled" => Ok(Self::PartiallyFilled),
            "Filled" => Ok(Self::Filled),
            "Canceled" => Ok(Self::Canceled),
            "PendingCancel" => Ok(Self::PendingCancel),
            "Rejected" => Ok(Self::Rejected),
            "PendingNew" => Ok(Self::PendingNew),
            "Expired" => Ok(Self::Expired),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrdStatus {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::New => write!(f, "New"),
            Self::PartiallyFilled => write!(f, "PartiallyFilled"),
            Self::Filled => write!(f, "Filled"),
            Self::Canceled => write!(f, "Canceled"),
            Self::PendingCancel => write!(f, "PendingCancel"),
            Self::Rejected => write!(f, "Rejected"),
            Self::PendingNew => write!(f, "PendingNew"),
            Self::Expired => write!(f, "Expired"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
