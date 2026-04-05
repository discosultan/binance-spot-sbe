#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderRateLimitExceededMode {
    DoNothing = 0x1_u8,
    CancelOnly = 0x2_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderRateLimitExceededMode {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::DoNothing,
            0x2_u8 => Self::CancelOnly,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<OrderRateLimitExceededMode> for u8 {
    #[inline]
    fn from(v: OrderRateLimitExceededMode) -> Self {
        match v {
            OrderRateLimitExceededMode::DoNothing => 0x1_u8,
            OrderRateLimitExceededMode::CancelOnly => 0x2_u8,
            OrderRateLimitExceededMode::NonRepresentable => 0xfe_u8,
            OrderRateLimitExceededMode::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for OrderRateLimitExceededMode {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "DoNothing" => Ok(Self::DoNothing),
            "CancelOnly" => Ok(Self::CancelOnly),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrderRateLimitExceededMode {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DoNothing => write!(f, "DoNothing"),
            Self::CancelOnly => write!(f, "CancelOnly"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
