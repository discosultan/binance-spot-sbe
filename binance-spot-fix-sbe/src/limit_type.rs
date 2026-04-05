#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LimitType {
    OrderLimit = 49_u8,
    MessageLimit = 50_u8,
    SubscriptionLimit = 51_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for LimitType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::OrderLimit,
            50_u8 => Self::MessageLimit,
            51_u8 => Self::SubscriptionLimit,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<LimitType> for u8 {
    #[inline]
    fn from(v: LimitType) -> Self {
        match v {
            LimitType::OrderLimit => 49_u8,
            LimitType::MessageLimit => 50_u8,
            LimitType::SubscriptionLimit => 51_u8,
            LimitType::NonRepresentable => 126_u8,
            LimitType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for LimitType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "OrderLimit" => Ok(Self::OrderLimit),
            "MessageLimit" => Ok(Self::MessageLimit),
            "SubscriptionLimit" => Ok(Self::SubscriptionLimit),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for LimitType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderLimit => write!(f, "OrderLimit"),
            Self::MessageLimit => write!(f, "MessageLimit"),
            Self::SubscriptionLimit => write!(f, "SubscriptionLimit"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
