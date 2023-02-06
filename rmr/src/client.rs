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

//! Safe wrapper for the RMR library wrapping the RMR Context.
//!
//! All the RMR APIs that use the `context` as one of the parameters as well as APIs required to
//! create context etc.

use crate::RMRError;
use crate::RMRMessageBuffer;

#[allow(unused)]
use super::rmr_int::*;

/// `RMRClient: A Wrapper over the internal 'context' of the RMR library.
///
/// APIs from the RMR library are available as methods of this struct.
pub struct RMRClient {
    // The empty `()` prevents this structure from being trivially constructed and can only be
    // constructed through the public API we provide (`new`) with appropriate errors.
    pub(crate) _empty: (),
}

impl RMRClient {
    /// Max Size to be used for `rmr_init`
    pub const RMR_MAX_RCV_BYTES: u32 = RMR_MAX_RCV_BYTES;
    /// Flags To be used for `rmr_init`
    pub const RMRFL_NONE: u32 = RMRFL_NONE;
    pub const RMRFL_NOTHREAD: u32 = RMRFL_NOTHREAD;
    pub const RMRFL_MTCALL: u32 = RMRFL_MTCALL;
    pub const RMRFL_AUTO_ALLOC: u32 = RMRFL_AUTO_ALLOC;
    pub const RMRFL_NAME_ONLY: u32 = RMRFL_NAME_ONLY;
    pub const RMRFL_NOLOCK: u32 = RMRFL_NOLOCK;

    /// Create a new RMRClient instance by calling `rmr_init`.
    pub fn new(port: &str, max_size: u32, flags: u32) -> Result<Self, RMRError> {
        rmr_client_new_internal(port, max_size, flags)
    }

    /// Client is Ready?
    pub fn is_ready(&self) -> bool {
        is_ready_internal()
    }

    /// Get the Client's receiver FD
    pub fn get_recv_fd(&self) -> Result<i32, RMRError> {
        get_recv_fd_internal()
    }

    /// Allocate a Message for transmission
    pub fn alloc_msg(&self) -> Result<*mut rmr_mbuf_t, RMRError> {
        alloc_message_internal()
    }

    /// Receive Message from RMR
    pub fn rcv_msg(&self, buff: *mut rmr_mbuf_t) -> Result<*mut rmr_mbuf_t, RMRError> {
        rcv_msg_internal(buff)
    }

    /// Return the Message to Sender
    pub fn rts_msg(&self, msg: &RMRMessageBuffer) -> Result<(), RMRError> {
        rts_msg_internal(msg)
    }
}

impl Drop for RMRClient {
    fn drop(&mut self) {
        rmr_close_internal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_two_clients_fails() {
        let result = RMRClient::new("1111", 0, 0);
        assert!(result.is_ok());

        let result_2 = RMRClient::new("2345", 0, 0);
        assert!(result_2.is_err());

        // This assertion should be here 'after' we try to create another client, if not, the
        // `result` gets dropped even before `result_2` is created and the test fails.
        assert!(!result.unwrap().is_ready());
    }

    #[test]
    fn test_integer_port_client_init_success_not_ready() {
        let result = RMRClient::new("1234", 0, 0);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_ready());
    }

    #[test]
    fn test_string_port_rmr_client_init_err() {
        let result = RMRClient::new("foo", 0, 0);
        assert!(result.is_err());
    }

    #[ignore]
    #[test]
    fn test_integer_port_below_1024_rmr_client_init_fail() {
        let result = RMRClient::new("80", 0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_flags_nothread_init_success_ready() {
        let result = RMRClient::new("1234", 0, RMRClient::RMRFL_NOTHREAD);
        assert!(result.is_ok());
        assert!(result.unwrap().is_ready());
    }
}
