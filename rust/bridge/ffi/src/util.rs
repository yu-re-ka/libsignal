//
// Copyright 2020-2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use device_transfer::Error as DeviceTransferError;
use hsm_enclave::Error as HsmEnclaveError;
use libc::c_char;
use libsignal_bridge::ffi::*;
use libsignal_protocol::*;
use signal_crypto::Error as SignalCryptoError;
use std::ffi::CString;
use zkgroup::ZkGroupError;

#[derive(Debug)]
#[repr(C)]
pub enum SignalErrorCode {
    #[allow(dead_code)]
    UnknownError = 1,
    InvalidState = 2,
    InternalError = 3,
    NullParameter = 4,
    InvalidArgument = 5,
    InvalidType = 6,
    InvalidUtf8String = 7,

    ProtobufError = 10,

    LegacyCiphertextVersion = 21,
    UnknownCiphertextVersion = 22,
    UnrecognizedMessageVersion = 23,

    InvalidMessage = 30,
    SealedSenderSelfSend = 31,

    InvalidKey = 40,
    InvalidSignature = 41,

    FingerprintVersionMismatch = 51,
    FingerprintParsingError = 52,

    UntrustedIdentity = 60,

    InvalidKeyIdentifier = 70,

    SessionNotFound = 80,
    InvalidRegistrationId = 81,
    InvalidSession = 82,
    InvalidSenderKeySession = 83,

    DuplicatedMessage = 90,

    CallbackError = 100,

    VerificationFailure = 110,
}

impl From<&SignalFfiError> for SignalErrorCode {
    fn from(err: &SignalFfiError) -> Self {
        match err {
            SignalFfiError::NullPointer => SignalErrorCode::NullParameter,
            SignalFfiError::InvalidType => SignalErrorCode::InvalidType,

            SignalFfiError::UnexpectedPanic(_)
            | SignalFfiError::DeviceTransfer(DeviceTransferError::InternalError(_))
            | SignalFfiError::Signal(SignalProtocolError::FfiBindingError(_)) => {
                SignalErrorCode::InternalError
            }

            SignalFfiError::InvalidUtf8String => SignalErrorCode::InvalidUtf8String,

            SignalFfiError::Signal(SignalProtocolError::InvalidProtobufEncoding) => {
                SignalErrorCode::ProtobufError
            }

            SignalFfiError::Signal(SignalProtocolError::DuplicatedMessage(_, _)) => {
                SignalErrorCode::DuplicatedMessage
            }

            SignalFfiError::Signal(SignalProtocolError::InvalidPreKeyId)
            | SignalFfiError::Signal(SignalProtocolError::InvalidSignedPreKeyId) => {
                SignalErrorCode::InvalidKeyIdentifier
            }

            SignalFfiError::Signal(SignalProtocolError::SealedSenderSelfSend) => {
                SignalErrorCode::SealedSenderSelfSend
            }

            SignalFfiError::Signal(SignalProtocolError::SignatureValidationFailed) => {
                SignalErrorCode::InvalidSignature
            }

            SignalFfiError::Signal(SignalProtocolError::NoKeyTypeIdentifier)
            | SignalFfiError::Signal(SignalProtocolError::BadKeyType(_))
            | SignalFfiError::Signal(SignalProtocolError::BadKeyLength(_, _))
            | SignalFfiError::Signal(SignalProtocolError::InvalidMacKeyLength(_))
            | SignalFfiError::DeviceTransfer(DeviceTransferError::KeyDecodingFailed)
            | SignalFfiError::HsmEnclave(HsmEnclaveError::InvalidPublicKeyError)
            | SignalFfiError::SignalCrypto(SignalCryptoError::InvalidKeySize) => {
                SignalErrorCode::InvalidKey
            }

            SignalFfiError::Signal(SignalProtocolError::SessionNotFound(_))
            | SignalFfiError::Signal(SignalProtocolError::NoSenderKeyState { .. }) => {
                SignalErrorCode::SessionNotFound
            }

            SignalFfiError::Signal(SignalProtocolError::InvalidRegistrationId(..)) => {
                SignalErrorCode::InvalidRegistrationId
            }

            SignalFfiError::Signal(SignalProtocolError::FingerprintParsingError) => {
                SignalErrorCode::FingerprintParsingError
            }

            SignalFfiError::Signal(SignalProtocolError::FingerprintVersionMismatch(_, _)) => {
                SignalErrorCode::FingerprintVersionMismatch
            }

            SignalFfiError::Signal(SignalProtocolError::UnrecognizedMessageVersion(_))
            | SignalFfiError::Signal(SignalProtocolError::UnknownSealedSenderVersion(_)) => {
                SignalErrorCode::UnrecognizedMessageVersion
            }

            SignalFfiError::Signal(SignalProtocolError::UnrecognizedCiphertextVersion(_)) => {
                SignalErrorCode::UnknownCiphertextVersion
            }

            SignalFfiError::Signal(SignalProtocolError::InvalidMessage(..))
            | SignalFfiError::Signal(SignalProtocolError::CiphertextMessageTooShort(_))
            | SignalFfiError::Signal(SignalProtocolError::InvalidSealedSenderMessage(_))
            | SignalFfiError::SignalCrypto(SignalCryptoError::InvalidTag)
            | SignalFfiError::HsmEnclave(HsmEnclaveError::HSMCommunicationError(_)) => {
                SignalErrorCode::InvalidMessage
            }

            SignalFfiError::Signal(SignalProtocolError::LegacyCiphertextVersion(_)) => {
                SignalErrorCode::LegacyCiphertextVersion
            }

            SignalFfiError::Signal(SignalProtocolError::UntrustedIdentity(_))
            | SignalFfiError::HsmEnclave(HsmEnclaveError::TrustedCodeError) => {
                SignalErrorCode::UntrustedIdentity
            }

            SignalFfiError::Signal(SignalProtocolError::InvalidState(_, _))
            | SignalFfiError::HsmEnclave(HsmEnclaveError::InvalidBridgeStateError) => {
                SignalErrorCode::InvalidState
            }

            SignalFfiError::Signal(SignalProtocolError::InvalidSessionStructure(_)) => {
                SignalErrorCode::InvalidSession
            }

            SignalFfiError::Signal(SignalProtocolError::InvalidSenderKeySession { .. }) => {
                SignalErrorCode::InvalidSenderKeySession
            }

            SignalFfiError::Signal(SignalProtocolError::InvalidArgument(_))
            | SignalFfiError::HsmEnclave(HsmEnclaveError::InvalidCodeHashError)
            | SignalFfiError::SignalCrypto(_)
            | SignalFfiError::ZkGroup(ZkGroupError::BadArgs) => SignalErrorCode::InvalidArgument,

            SignalFfiError::Signal(SignalProtocolError::ApplicationCallbackError(_, _)) => {
                SignalErrorCode::CallbackError
            }

            SignalFfiError::ZkGroup(
                ZkGroupError::DecryptionFailure
                | ZkGroupError::MacVerificationFailure
                | ZkGroupError::ProofVerificationFailure
                | ZkGroupError::SignatureVerificationFailure,
            ) => SignalErrorCode::VerificationFailure,
        }
    }
}

pub(crate) unsafe fn write_cstr_to(
    out: *mut *const c_char,
    value: Result<impl Into<Vec<u8>>, SignalProtocolError>,
) -> Result<(), SignalFfiError> {
    write_optional_cstr_to(out, value.map(Some))
}

pub(crate) unsafe fn write_optional_cstr_to(
    out: *mut *const c_char,
    value: Result<Option<impl Into<Vec<u8>>>, SignalProtocolError>,
) -> Result<(), SignalFfiError> {
    if out.is_null() {
        return Err(SignalFfiError::NullPointer);
    }

    match value {
        Ok(Some(value)) => {
            let cstr =
                CString::new(value).expect("No NULL characters in string being returned to C");
            *out = cstr.into_raw();
            Ok(())
        }
        Ok(None) => {
            *out = std::ptr::null();
            Ok(())
        }
        Err(e) => Err(SignalFfiError::Signal(e)),
    }
}
