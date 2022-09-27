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

use std::convert::TryInto;
use std::ffi::CString;

use crate::RMRError;
use crate::RMRMessageBuffer;

use super::rmr_int;

type RMRContext = *mut ::std::os::raw::c_void;

/// `RMRClient: A Wrapper over the internal 'context' of the RMR library.
///
/// APIs from the RMR library are available as methods of this struct.
pub struct RMRClient {
    context: RMRContext,
}

impl RMRClient {
    /// Max Size to be used for `rmr_init`
    pub const RMR_MAX_RCV_BYTES: u32 = rmr_int::RMR_MAX_RCV_BYTES;
    /// Flags To be used for `rmr_init`
    pub const RMRFL_NONE: u32 = rmr_int::RMRFL_NONE;
    pub const RMRFL_NOTHREAD: u32 = rmr_int::RMRFL_NOTHREAD;
    pub const RMRFL_MTCALL: u32 = rmr_int::RMRFL_MTCALL;
    pub const RMRFL_AUTO_ALLOC: u32 = rmr_int::RMRFL_AUTO_ALLOC;
    pub const RMRFL_NAME_ONLY: u32 = rmr_int::RMRFL_NAME_ONLY;
    pub const RMRFL_NOLOCK: u32 = rmr_int::RMRFL_NOLOCK;

    /// Create a new RMRClient instance by calling `rmr_init`.
    pub fn new(port: &str, max_size: u32, flags: u32) -> Result<Self, RMRError> {
        let port_chars = CString::new(port).unwrap();
        unsafe {
            let context = rmr_int::rmr_init(
                port_chars.into_raw(),
                max_size.try_into().unwrap(),
                flags.try_into().unwrap(),
            );
            if context.is_null() {
                Err(RMRError)
            } else {
                Ok(Self { context })
            }
        }
    }

    /// Client is Ready?
    pub fn is_ready(&self) -> bool {
        unsafe { rmr_int::rmr_ready(self.context) == 1 }
    }

    /// Get the Client's receiver FD
    pub fn get_recv_fd(&self) -> Result<i32, RMRError> {
        unsafe {
            let fd = rmr_int::rmr_get_rcvfd(self.context);
            if fd < 0 {
                Err(RMRError)
            } else {
                Ok(fd)
            }
        }
    }

    pub fn alloc_msg(&self) -> Result<*mut rmr_int::rmr_mbuf_t, RMRError> {
        unsafe {
            let buff = rmr_int::rmr_alloc_msg(self.context, 4096);
            if buff.is_null() {
                Err(RMRError)
            } else {
                Ok(buff)
            }
        }
    }

    pub fn rcv_msg(
        &self,
        buff: *mut rmr_int::rmr_mbuf_t,
    ) -> Result<*mut rmr_int::rmr_mbuf_t, RMRError> {
        unsafe {
            let buff = rmr_int::rmr_rcv_msg(self.context, buff);
            if buff.is_null() {
                Err(RMRError)
            } else {
                Ok(buff)
            }
        }
    }
    pub fn rts_msg(&self, msg: &RMRMessageBuffer) -> Result<(), RMRError> {
        unsafe {
            let buff = msg.buff;
            let send_buff = rmr_int::rmr_rts_msg(self.context, buff);
            if send_buff.is_null() {
                Err(RMRError)
            } else {
                Ok(())
            }
        }
    }
}

// This is required to wrap the `RMRClient` inside `Arc`.
unsafe impl Send for RMRClient {}
unsafe impl Sync for RMRClient {}

impl Drop for RMRClient {
    fn drop(&mut self) {
        unsafe { rmr_int::rmr_close(self.context) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
