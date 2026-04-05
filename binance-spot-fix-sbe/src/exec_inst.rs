#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExecInst {
    ParticipateDontInitiate = 54_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for ExecInst {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            54_u8 => Self::ParticipateDontInitiate,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ExecInst> for u8 {
    #[inline]
    fn from(v: ExecInst) -> Self {
        match v {
            ExecInst::ParticipateDontInitiate => 54_u8,
            ExecInst::NonRepresentable => 126_u8,
            ExecInst::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for ExecInst {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "ParticipateDontInitiate" => Ok(Self::ParticipateDontInitiate),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ExecInst {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParticipateDontInitiate => write!(f, "ParticipateDontInitiate"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
