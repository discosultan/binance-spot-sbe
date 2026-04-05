#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExecType {
    New = 48_u8,
    Canceled = 52_u8,
    Replaced = 53_u8,
    Rejected = 56_u8,
    Trade = 70_u8,
    Expired = 67_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for ExecType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::New,
            52_u8 => Self::Canceled,
            53_u8 => Self::Replaced,
            56_u8 => Self::Rejected,
            70_u8 => Self::Trade,
            67_u8 => Self::Expired,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ExecType> for u8 {
    #[inline]
    fn from(v: ExecType) -> Self {
        match v {
            ExecType::New => 48_u8,
            ExecType::Canceled => 52_u8,
            ExecType::Replaced => 53_u8,
            ExecType::Rejected => 56_u8,
            ExecType::Trade => 70_u8,
            ExecType::Expired => 67_u8,
            ExecType::NonRepresentable => 126_u8,
            ExecType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for ExecType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "New" => Ok(Self::New),
            "Canceled" => Ok(Self::Canceled),
            "Replaced" => Ok(Self::Replaced),
            "Rejected" => Ok(Self::Rejected),
            "Trade" => Ok(Self::Trade),
            "Expired" => Ok(Self::Expired),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ExecType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::New => write!(f, "New"),
            Self::Canceled => write!(f, "Canceled"),
            Self::Replaced => write!(f, "Replaced"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Trade => write!(f, "Trade"),
            Self::Expired => write!(f, "Expired"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
