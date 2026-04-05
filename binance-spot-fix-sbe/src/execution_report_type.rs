#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExecutionReportType {
    Full = 0x1_u8,
    Mini = 0x2_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ExecutionReportType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Full,
            0x2_u8 => Self::Mini,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<ExecutionReportType> for u8 {
    #[inline]
    fn from(v: ExecutionReportType) -> Self {
        match v {
            ExecutionReportType::Full => 0x1_u8,
            ExecutionReportType::Mini => 0x2_u8,
            ExecutionReportType::NonRepresentable => 0xfe_u8,
            ExecutionReportType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for ExecutionReportType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Full" => Ok(Self::Full),
            "Mini" => Ok(Self::Mini),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for ExecutionReportType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Full => write!(f, "Full"),
            Self::Mini => write!(f, "Mini"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
