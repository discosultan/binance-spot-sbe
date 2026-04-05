#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum WorkingFloor {
    Exchange = 0x1_u8,
    Broker = 0x2_u8,
    Sor = 0x3_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for WorkingFloor {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Exchange,
            0x2_u8 => Self::Broker,
            0x3_u8 => Self::Sor,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<WorkingFloor> for u8 {
    #[inline]
    fn from(v: WorkingFloor) -> Self {
        match v {
            WorkingFloor::Exchange => 0x1_u8,
            WorkingFloor::Broker => 0x2_u8,
            WorkingFloor::Sor => 0x3_u8,
            WorkingFloor::NonRepresentable => 0xfe_u8,
            WorkingFloor::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for WorkingFloor {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Exchange" => Ok(Self::Exchange),
            "Broker" => Ok(Self::Broker),
            "Sor" => Ok(Self::Sor),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for WorkingFloor {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Exchange => write!(f, "Exchange"),
            Self::Broker => write!(f, "Broker"),
            Self::Sor => write!(f, "Sor"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
