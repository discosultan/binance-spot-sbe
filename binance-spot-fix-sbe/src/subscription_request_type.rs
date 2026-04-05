#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SubscriptionRequestType {
    Subscribe = 49_u8,
    Unsubscribe = 50_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for SubscriptionRequestType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::Subscribe,
            50_u8 => Self::Unsubscribe,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<SubscriptionRequestType> for u8 {
    #[inline]
    fn from(v: SubscriptionRequestType) -> Self {
        match v {
            SubscriptionRequestType::Subscribe => 49_u8,
            SubscriptionRequestType::Unsubscribe => 50_u8,
            SubscriptionRequestType::NonRepresentable => 126_u8,
            SubscriptionRequestType::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for SubscriptionRequestType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Subscribe" => Ok(Self::Subscribe),
            "Unsubscribe" => Ok(Self::Unsubscribe),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for SubscriptionRequestType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Subscribe => write!(f, "Subscribe"),
            Self::Unsubscribe => write!(f, "Unsubscribe"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
