#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SessionRejectReason {
    InvalidTagNumber = 0x0_u8,
    RequiredTagMissing = 0x1_u8,
    TagNotDefinedForThisMessageType = 0x2_u8,
    UndefinedTag = 0x3_u8,
    ValueIsIncorrect = 0x5_u8,
    IncorrectDataFormatForValue = 0x6_u8,
    SignatureProblem = 0x8_u8,
    SendingTimeAccuracyProblem = 0xa_u8,
    TagAppearsMoreThanOnce = 0xd_u8,
    TagSpecifiedOutOfRequiredOrder = 0xe_u8,
    RepeatingGroupFieldsOutOfOrder = 0xf_u8,
    IncorrectNumInGroupCountForRepeatingGroup = 0x10_u8,
    Other = 0x63_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for SessionRejectReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::InvalidTagNumber,
            0x1_u8 => Self::RequiredTagMissing,
            0x2_u8 => Self::TagNotDefinedForThisMessageType,
            0x3_u8 => Self::UndefinedTag,
            0x5_u8 => Self::ValueIsIncorrect,
            0x6_u8 => Self::IncorrectDataFormatForValue,
            0x8_u8 => Self::SignatureProblem,
            0xa_u8 => Self::SendingTimeAccuracyProblem,
            0xd_u8 => Self::TagAppearsMoreThanOnce,
            0xe_u8 => Self::TagSpecifiedOutOfRequiredOrder,
            0xf_u8 => Self::RepeatingGroupFieldsOutOfOrder,
            0x10_u8 => Self::IncorrectNumInGroupCountForRepeatingGroup,
            0x63_u8 => Self::Other,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<SessionRejectReason> for u8 {
    #[inline]
    fn from(v: SessionRejectReason) -> Self {
        match v {
            SessionRejectReason::InvalidTagNumber => 0x0_u8,
            SessionRejectReason::RequiredTagMissing => 0x1_u8,
            SessionRejectReason::TagNotDefinedForThisMessageType => 0x2_u8,
            SessionRejectReason::UndefinedTag => 0x3_u8,
            SessionRejectReason::ValueIsIncorrect => 0x5_u8,
            SessionRejectReason::IncorrectDataFormatForValue => 0x6_u8,
            SessionRejectReason::SignatureProblem => 0x8_u8,
            SessionRejectReason::SendingTimeAccuracyProblem => 0xa_u8,
            SessionRejectReason::TagAppearsMoreThanOnce => 0xd_u8,
            SessionRejectReason::TagSpecifiedOutOfRequiredOrder => 0xe_u8,
            SessionRejectReason::RepeatingGroupFieldsOutOfOrder => 0xf_u8,
            SessionRejectReason::IncorrectNumInGroupCountForRepeatingGroup => 0x10_u8,
            SessionRejectReason::Other => 0x63_u8,
            SessionRejectReason::NonRepresentable => 0xfe_u8,
            SessionRejectReason::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for SessionRejectReason {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "InvalidTagNumber" => Ok(Self::InvalidTagNumber),
            "RequiredTagMissing" => Ok(Self::RequiredTagMissing),
            "TagNotDefinedForThisMessageType" => Ok(Self::TagNotDefinedForThisMessageType),
            "UndefinedTag" => Ok(Self::UndefinedTag),
            "ValueIsIncorrect" => Ok(Self::ValueIsIncorrect),
            "IncorrectDataFormatForValue" => Ok(Self::IncorrectDataFormatForValue),
            "SignatureProblem" => Ok(Self::SignatureProblem),
            "SendingTimeAccuracyProblem" => Ok(Self::SendingTimeAccuracyProblem),
            "TagAppearsMoreThanOnce" => Ok(Self::TagAppearsMoreThanOnce),
            "TagSpecifiedOutOfRequiredOrder" => Ok(Self::TagSpecifiedOutOfRequiredOrder),
            "RepeatingGroupFieldsOutOfOrder" => Ok(Self::RepeatingGroupFieldsOutOfOrder),
            "IncorrectNumInGroupCountForRepeatingGroup" => {
                Ok(Self::IncorrectNumInGroupCountForRepeatingGroup)
            }
            "Other" => Ok(Self::Other),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for SessionRejectReason {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidTagNumber => write!(f, "InvalidTagNumber"),
            Self::RequiredTagMissing => write!(f, "RequiredTagMissing"),
            Self::TagNotDefinedForThisMessageType => write!(f, "TagNotDefinedForThisMessageType"),
            Self::UndefinedTag => write!(f, "UndefinedTag"),
            Self::ValueIsIncorrect => write!(f, "ValueIsIncorrect"),
            Self::IncorrectDataFormatForValue => write!(f, "IncorrectDataFormatForValue"),
            Self::SignatureProblem => write!(f, "SignatureProblem"),
            Self::SendingTimeAccuracyProblem => write!(f, "SendingTimeAccuracyProblem"),
            Self::TagAppearsMoreThanOnce => write!(f, "TagAppearsMoreThanOnce"),
            Self::TagSpecifiedOutOfRequiredOrder => write!(f, "TagSpecifiedOutOfRequiredOrder"),
            Self::RepeatingGroupFieldsOutOfOrder => write!(f, "RepeatingGroupFieldsOutOfOrder"),
            Self::IncorrectNumInGroupCountForRepeatingGroup => {
                write!(f, "IncorrectNumInGroupCountForRepeatingGroup")
            }
            Self::Other => write!(f, "Other"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}
