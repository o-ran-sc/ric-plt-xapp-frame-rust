// ==================================================================================
//   Copyright (c) 2022 Caurus
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

//! Crate for handling the RMR functionality provided by `librmr_si.so`
//!
//! ORAN-SC xApps use RMR as a message bus for communicating between different components of an
//! ORAN NearRT RIC. This crate handles basic functionality related to receiving RMR messages,
//! processing them. An implementation of xApp would typically use the APIs provided by this crate
//! for dealing with the RMR Message bus.
mod rmr_int;

mod client;
mod error;
mod mbuf;
mod processor;
mod receiver;

pub use client::RMRClient;
pub use error::RMRError;
pub use mbuf::RMRMessageBuffer;
pub use processor::{RMRProcessor, RMRProcessorFn};
pub use receiver::RMRReceiver;
