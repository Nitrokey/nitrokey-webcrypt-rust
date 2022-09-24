use heapless_bytes::{Bytes, Bytes32, Bytes64};
use serde::{Deserialize, Serialize};
use trussed::types::{KeyId, Message};

use crate::types::ERROR_ID;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandStatusResponse {
    /// True, when unlocked through the Login command; U2F compatibility
    pub(crate) unlocked: bool,
    /// Webcrypt's version
    pub(crate) version: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) version_string: Option<Bytes<100>>,
    /// Count of a free Webcrypt's resident keys' slots
    pub(crate) slots: u16,
    /// The current FIDO2/U2F PIN attempt counter value
    pub(crate) pin_attempts: u8,
}

pub type Bytes8 = Bytes<8>;
pub type Bytes40 = Bytes<40>;
pub type Bytes65 = Bytes<65>;
pub type Bytes200 = Bytes<200>;
pub type Bytes250 = Bytes<250>;
pub type DataBytes = Bytes<1024>;
pub type SessionToken = Bytes32;
pub type ExpectedSessionToken = Option<SessionToken>;
pub(crate) type SerializedCredential = trussed::types::Message;
pub type ResultW<T> = core::result::Result<T, ERROR_ID>;

// TODO move to struct, instead of the type alias
pub type KeyHandleSerialized = Bytes250;

#[derive(Clone, Debug, serde_indexed::DeserializeIndexed, serde_indexed::SerializeIndexed)]
pub struct CredentialData {
    pub creation_time: u32,
    /// P256 or Ed25519
    pub algorithm: i32,
    pub allowed_use: i32,
    pub key_id: KeyId,
}

impl CredentialData {
    pub fn new(key_id: KeyId) -> Self {
        Self {
            key_id,
            creation_time: 0,
            algorithm: 0,
            allowed_use: 0,
        }
    }
    pub fn serialize(&self) -> ResultW<SerializedCredential> {
        trussed::cbor_serialize_bytes(self).map_err(|_| ERROR_ID::ERR_INTERNAL_ERROR)
    }
    pub fn deserialize(&self, buffer: SerializedCredential) -> ResultW<Self> {
        trussed::cbor_deserialize(buffer.as_ref()).map_err(|_| ERROR_ID::ERR_INTERNAL_ERROR)
    }
}

#[derive(Clone, Debug, serde_indexed::DeserializeIndexed, serde_indexed::SerializeIndexed)]
pub struct Credential {
    pub data: CredentialData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandGenerateResponse {
    /// resulting public key
    pub(crate) pubkey: Bytes65,

    /// key handle, should be less than 200 bytes
    pub(crate) keyhandle: KeyHandleSerialized,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandSignResponse {
    /// signed hash, the same given on input, 32 bytes
    pub(crate) inhash: Bytes32,

    /// signature, should be less than 100 bytes
    pub(crate) signature: Bytes200,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandSignRequest {
    /// hash to sign, 32 bytes
    pub(crate) hash: Bytes32,

    /// key handle, should be less than 200 bytes
    pub(crate) keyhandle: KeyHandleSerialized,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandOpenPGPDecryptRequest {
    /// ephemeral ecc encryption key
    pub(crate) eccekey: DataBytes,

    /// key handle, should be less than 200 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) keyhandle: Option<KeyHandleSerialized>,

    /// public key fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fingerprint: Option<DataBytes>,

    /// name of the algorithm for decryption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) oid: Option<DataBytes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandOpenPGPDecryptResponse {
    /// result of decryption
    pub(crate) data: DataBytes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandOpenPGPSignRequest {
    /// data to sign
    pub(crate) data: DataBytes,

    /// name of the algorithm for decryption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) oid: Option<DataBytes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandOpenPGPSignResponse {
    /// result of decryption
    pub(crate) signature: DataBytes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandOpenPGPInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandOpenPGPInfoResponse {
    pub(crate) encr_pubkey: DataBytes,
    pub(crate) auth_pubkey: DataBytes,
    pub(crate) sign_pubkey: DataBytes,
    pub(crate) date: DataBytes,
    // pub(crate) sign_keyhandle: KeyHandleSerialized,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandOpenPGPImportRequest {
    pub(crate) encr_privkey: DataBytes,
    pub(crate) auth_privkey: DataBytes,
    pub(crate) sign_privkey: DataBytes,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) date: Option<DataBytes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}
// no response
// pub type CommandOpenPGPImportResponse = CommandOpenPGPInfoResponse;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandDecryptRequest {
    /// data to decrypt
    pub(crate) data: DataBytes,

    /// ciphertext's hmac
    pub(crate) hmac: DataBytes,

    /// ephemeral ecc encryption key
    pub(crate) eccekey: DataBytes,

    /// key handle, should be less than 200 bytes
    pub(crate) keyhandle: KeyHandleSerialized,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandDecryptResponse {
    /// data to decrypt
    pub(crate) data: DataBytes,
}

// TODO use macros to better represent data scheme

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandEmptyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

pub type CommandGenerateRequest = CommandEmptyRequest;
pub type CommandLogoutRequest = CommandEmptyRequest;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandGenerateFromDataRequest {
    /// data to be used for key derivation
    pub(crate) hash: DataBytes,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

// note: identical to GenerateResponse
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandGenerateFromDataResponse {
    /// resulting public key
    pub(crate) pubkey: Bytes65,

    /// key handle, should be less than 200 bytes
    pub(crate) keyhandle: KeyHandleSerialized,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandInitializeRequest {
    /// data to be used as an additional source of the random data for the master key generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) entropy: Option<Bytes40>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandInitializeResponse {
    /// generated master key, returned for backup, 32 bytes
    pub(crate) master: Bytes32,

    /// salt, 8 bytes
    pub(crate) salt: Bytes8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandRestoreRequest {
    /// generated master key, returned for backup, 32 bytes
    pub(crate) master: Bytes32,

    /// salt, 8 bytes
    pub(crate) salt: Bytes8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandRestoreResponse {
    /// data to be used as an additional source of the random data for the master key generation
    pub(crate) hash: Bytes32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandWriteResidentKeyRequest {
    /// Sent in P256 serialized private key
    pub(crate) raw_key_data: Bytes32,

    // a placeholder for metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandGenerateResidentKeyRequest {
    // a placeholder for metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

pub type CommandWriteResidentKeyResponse = CommandGenerateResidentKeyResponse;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandGenerateResidentKeyResponse {
    /// resulting public key
    pub(crate) pubkey: Bytes65,

    /// key handle, should be less than 200 bytes
    /// should contain short KH, with the type==RK
    // pub(crate) keyhandle: KeyHandleSerialized,
    pub(crate) keyhandle: Bytes32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandDiscoverResidentKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

// TODO allow to find RK by public key?

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandDiscoverResidentKeyResponse {
    /// resulting public key
    pub(crate) pubkey: Bytes65,

    /// key handle, should be less than 200 bytes
    /// should contain short KH, with the type==RK
    // pub(crate) keyhandle: KeyHandleSerialized,
    pub(crate) keyhandle: KeyId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandReadResidentKeyRequest {
    /// key handle, should be less than 200 bytes
    /// should contain short KH, with the type==RK
    pub(crate) keyhandle: Bytes32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandLoginRequest {
    /// User PIN. Equal requirements to the FIDO2 PIN.
    pub(crate) pin: Bytes64,

    // TODO this command does not need TP, but added this field
    // to make the calls unified. Remove later?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandSetPINRequest {
    /// User PIN. Equal requirements to the FIDO2 PIN.
    pub(crate) pin: Bytes64,

    // TODO this command does not need TP, but added this field
    // to make the calls unified. Remove later?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandChangePINRequest {
    /// User PIN. Equal requirements to the FIDO2 PIN.
    pub(crate) pin: Bytes64,
    pub(crate) newpin: Bytes64,

    // TODO this command does not need TP, but added this field
    // to make the calls unified. Remove later?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandConfigureRequest {
    /// User PIN. Equal requirements to the FIDO2 PIN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirmation: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tp: ExpectedSessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct CommandLoginResponse {
    /// Temporary password / access token
    pub(crate) tp: SessionToken,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) struct KeyHandle {
    pub(crate) appid: Bytes32,
    /// encrypted private key, containing appid info
    pub(crate) wrapped_private_key: Bytes<256>,
    /// nonce for encryption
    pub(crate) nonce: Bytes<12>,
    /// usage flags for key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) usage_flags: Option<u8>,
}

impl KeyHandle {
    pub(crate) fn ser(self) -> Bytes200 {
        trussed::cbor_serialize_bytes(&self).unwrap()
    }

    pub(crate) fn deser(b: Message) -> Self {
        trussed::cbor_deserialize(b.as_slice()).unwrap()
    }
}
