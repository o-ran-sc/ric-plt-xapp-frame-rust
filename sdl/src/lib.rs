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

//! An Interface to Shared Data Layer (SDL) for the ORAN-SC
//!
//! Shared Data Layer (SDL) provides an API for data persistence using backend storages. The
//! applications can use these API to interface with persistent data.
//!
//! This crate defines a common trait `SdlStorage` - this defines the APIs for accessing the stored
//! data. Additionally included in this crate is an implementation of an actual storage backend
//! that supports Redis database for persistence. The backend implemented implements the trait
//! `SdlStorage`.
//!
//! Initialization of SDL and backend happens through a set of environment variables.

mod api;

#[doc(inline)]
pub use api::SdlStorageApi;

#[doc(inline)]
pub use api::SdlError;

#[doc(inline)]
pub use api::DataMap;

#[doc(inline)]
pub use api::KeySet;

#[doc(inline)]
pub use api::ValueType;

mod backends;
pub use backends::RedisStorage;
