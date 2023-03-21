// ==================================================================================
//   Copyright (c) 2023 Abhijit Gadgil
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
// ==================================================================================

//! Public API: `SdlStorageApi` is a trait that defines functions and associated types. An
//! SDL Storage backend will implement this API and provide the abstracted SDL functionality to the
//! xApps.

// TODO: When supported, make these as trait Associated Types
type KeySet = std::collections::BTreeSet<String>;

type DataMap = std::collections::HashMap<String, Vec<u8>>;

/// `SdlError`: Error structure used by the `SdlStorageApi` functions.
#[derive(Debug)]
pub struct SdlError(String);

impl std::error::Error for SdlError {}

impl core::fmt::Display for SdlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SdlError: {}", self.0)
    }
}

/// `SdlStorageApi`: An API for the DB operations for Shared Data Layer.
///
/// An implementation of a backend for SDL (for example Redis), will implement this trait and thus
/// provide the common API for storing data for the xApps, through the backend structure maintained
/// by the xApp framework.
///
/// All the API function take as a first parameter a 'namespace' (that is the namespace under which
/// given set of Keys / Values is stored. The backend can internally use own mechanisms to provide
/// this logical 'namespace' separation. The API per se do not dictate the implementation.
///
/// The Associated Types, `KeySet` and `DataMap` determine the types used by the storage
/// implementation (The default associated types should be good enoough for most use cases.)
pub trait SdlStorageApi {
    /// Set the `data` in the given namespace.
    ///
    /// This will add all the keys present in the `DataMap` to the storage. The operation is
    /// atomic, that is either all keys are set or an error occurs.
    fn set(namespace: &str, data: &DataMap) -> Result<(), SdlError>;

    /// Set the keys in the `data` if the `key` does not exists in the database.
    ///
    /// If the key exists in the database, this operation is a no-op. The actual implementation of
    /// this function may use logging to provide the feedback to the caller.
    fn set_if_not_exists(namespace: &str, key: &str, value: &[u8]) -> Result<(), SdlError>;

    /// Read values for the given 'keys'.
    ///
    /// If no value from the given `keys` exists in the database, this is not an error condition,
    /// but an empty `DataMap` is returned.  The actual implementation may use suitable logging
    /// ( for example `log::warn`) to indicate the missing keys to the caller.
    fn get(namespace: &str, keys: &KeySet) -> Result<DataMap, SdlError>;

    /// Delete the values for the 'keys' from the storage.
    ///
    /// If any key in the `keys` is missing, this is not an error and simply the `key` is ignored.
    /// The actual implementation may use suitable logging ( for example `log::warn`) to indicate
    /// the missing keys to the caller.
    fn delete(namespace: &str, keys: &KeySet) -> Result<(), SdlError>;

    /// Delete the key from the SDL Storage if the current value matches given value.
    ///
    /// If the key does not exist,it is not considered an error.
    fn delete_if(namespace: &str, key: &str, value: &[u8]) -> Result<bool, SdlError>;

    /// List keys from the SDL Storage that match the given pattern.
    ///
    /// This function can return an empty `KeySet`.
    fn list_keys(namespace: &str, pattern: &str) -> Result<KeySet, SdlError>;

    /// Delete all Keys from the SDL Storage
    fn delete_all(namespace: &str) -> Result<(), SdlError>;
}
