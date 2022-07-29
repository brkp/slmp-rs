use enum_primitive_derive::Primitive;
use num_traits::{FromPrimitive, ToPrimitive};

/// TODO: docs
#[derive(Debug, Copy, Clone, PartialEq, Eq, Primitive)]
pub enum SLMPEndCode {
    Success = 0x00,
    WrongCommand = 0xC059,
    WrongFormat = 0xC05C,
    WrongLength = 0xC061,
    Busy = 0xCEE0,
    ExceedReqLength = 0xCEE1,
    ExceedRespLength = 0xCEE2,
    ServerNotFound = 0xCF10,
    WrongConfigItem = 0xCF20,
    PrmIDNotFound = 0xCF30,
    NotStartExclusiveWrite = 0xCF31,
    RelayFailure = 0xCF70,
    TimeoutError = 0xCF71,
    CANAppNotPermittedRead = 0xCCC7,
    CANAppWriteOnly = 0xCCC8,
    CANAppReadOnly = 0xCCC9,
    CANAppUndefinedObjectAccess = 0xCCCA,
    CANAppNotPermittedPDOMapping = 0xCCCB,
    CANAppExceedPDOMapping = 0xCCCC,
    CANAppNotExistSubIndex = 0xCCD3,
    CANAppWrongParameter = 0xCCD4,
    CANAppMoreOverParameterRange = 0xCCD5,
    CANAppLessOverParameterRange = 0xCCD6,
    CANAppTransOrStoreError = 0xCCDA,
    CANAppOtherError = 0xCCFF,
    OtherNetworkError = 0xCF00,
    DataFragmentShortage = 0xCF40,
    DataFragmentDup = 0xCF41,
    DataFragmentLost = 0xCF43,
    DataFragmentNotSupport = 0xCF44,
}

impl std::fmt::Display for SLMPEndCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
