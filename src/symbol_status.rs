#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SymbolStatus {
    PreTrading = 0x0_u8,
    Trading = 0x1_u8,
    PostTrading = 0x2_u8,
    EndOfDay = 0x3_u8,
    Halt = 0x4_u8,
    AuctionMatch = 0x5_u8,
    Break = 0x7_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for SymbolStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::PreTrading,
            0x1_u8 => Self::Trading,
            0x2_u8 => Self::PostTrading,
            0x3_u8 => Self::EndOfDay,
            0x4_u8 => Self::Halt,
            0x5_u8 => Self::AuctionMatch,
            0x7_u8 => Self::Break,
            _ => Self::NullVal,
        }
    }
}
impl From<SymbolStatus> for u8 {
    #[inline]
    fn from(v: SymbolStatus) -> Self {
        match v {
            SymbolStatus::PreTrading => 0x0_u8,
            SymbolStatus::Trading => 0x1_u8,
            SymbolStatus::PostTrading => 0x2_u8,
            SymbolStatus::EndOfDay => 0x3_u8,
            SymbolStatus::Halt => 0x4_u8,
            SymbolStatus::AuctionMatch => 0x5_u8,
            SymbolStatus::Break => 0x7_u8,
            SymbolStatus::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for SymbolStatus {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "PreTrading" => Ok(Self::PreTrading),
            "Trading" => Ok(Self::Trading),
            "PostTrading" => Ok(Self::PostTrading),
            "EndOfDay" => Ok(Self::EndOfDay),
            "Halt" => Ok(Self::Halt),
            "AuctionMatch" => Ok(Self::AuctionMatch),
            "Break" => Ok(Self::Break),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for SymbolStatus {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PreTrading => write!(f, "PreTrading"),
            Self::Trading => write!(f, "Trading"),
            Self::PostTrading => write!(f, "PostTrading"),
            Self::EndOfDay => write!(f, "EndOfDay"),
            Self::Halt => write!(f, "Halt"),
            Self::AuctionMatch => write!(f, "AuctionMatch"),
            Self::Break => write!(f, "Break"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
