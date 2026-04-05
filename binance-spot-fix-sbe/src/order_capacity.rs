#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderCapacity {
    Agency = 65_u8,
    Principal = 80_u8,
    NonRepresentable = 126_u8,
    #[default]
    NullVal = 0_u8,
}
impl From<u8> for OrderCapacity {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            65_u8 => Self::Agency,
            80_u8 => Self::Principal,
            126_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<OrderCapacity> for u8 {
    #[inline]
    fn from(v: OrderCapacity) -> Self {
        match v {
            OrderCapacity::Agency => 65_u8,
            OrderCapacity::Principal => 80_u8,
            OrderCapacity::NonRepresentable => 126_u8,
            OrderCapacity::NullVal => 0_u8,
        }
    }
}
impl core::str::FromStr for OrderCapacity {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Agency" => Ok(Self::Agency),
            "Principal" => Ok(Self::Principal),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for OrderCapacity {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Agency => write!(f, "Agency"),
            Self::Principal => write!(f, "Principal"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
