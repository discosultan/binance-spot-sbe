#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InstrumentListRequestType {
    SingleInstrument = 0x0_u8,
    AllInstruments = 0x4_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for InstrumentListRequestType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::SingleInstrument,
            0x4_u8 => Self::AllInstruments,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<InstrumentListRequestType> for u8 {
    #[inline]
    fn from(v: InstrumentListRequestType) -> Self {
        match v {
            InstrumentListRequestType::SingleInstrument => 0x0_u8,
            InstrumentListRequestType::AllInstruments => 0x4_u8,
            InstrumentListRequestType::NonRepresentable => 0xfe_u8,
            InstrumentListRequestType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for InstrumentListRequestType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "SingleInstrument" => Ok(Self::SingleInstrument),
            "AllInstruments" => Ok(Self::AllInstruments),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for InstrumentListRequestType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SingleInstrument => write!(f, "SingleInstrument"),
            Self::AllInstruments => write!(f, "AllInstruments"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
