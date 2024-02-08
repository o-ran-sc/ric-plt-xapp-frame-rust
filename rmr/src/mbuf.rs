// ==================================================================================
//   Copyright (c) 2022 Caurus
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

//! Safe wrapper for the RMR `rmr_mbuf_t` handling.
//!
//! Methods related to RMR library API that provide access to the internals of the RMR Message
//! Buffer `rmr_mbuf_t`.
//!
use std::convert::TryInto;

use crate::RMRClient;

use super::rmr_int;

type RMRMBuf = *mut rmr_int::rmr_mbuf_t;

/// Structure wrapping `rmr_mbuf_t` and associated Message Type.
#[derive(Debug)]
pub struct RMRMessageBuffer {
    pub(crate) msgtype: i32,
    pub(crate) buff: RMRMBuf,
}

impl RMRMessageBuffer {
    /// Allocate a new Buffer using the `RMRClient`
    pub fn new(client: &RMRClient) -> Self {
        let buff = client.alloc_msg().expect("Alloc Message Failed!");
        Self { msgtype: -1, buff }
    }

    pub(crate) fn from_buf(buff: RMRMBuf, mtype: i32) -> Self {
        Self {
            msgtype: mtype,
            buff,
        }
    }

    pub fn free(&self) {
        unsafe {
            rmr_int::rmr_free_msg(self.buff);
        }
    }

    pub fn set_payload(&mut self, payload: &[u8]) {
        // Safety: self.buff is a valid pointer. This is because, the structure can only be created
        // through internal function calls where we can guarantee as implementors that the pointers
        // passed to the `new` is a valid one.
        unsafe {
            // TODO: Use this to potentially 'realloc' the buffer.
            let _max_size = rmr_int::rmr_payload_size(self.buff);
            let payload_size = payload.len();
            (*self.buff).len = payload_size.try_into().unwrap();
            std::ptr::copy(payload.as_ptr(), (*self.buff).payload, payload_size);
        }
    }

    pub fn set_mtype(&mut self, mtype: i32) {
        // Safety: self.buff is a valid pointer. This is because, the structure can only be created
        // through internal function calls where we can guarantee as implementors that the pointers
        // passed to the `new` is a valid one.
        unsafe {
            (*self.buff).mtype = mtype;
        }
    }

    pub fn get_state(&self) -> i32 {
        // Safety: self.buff is a valid pointer. This is because, the structure can only be created
        // through internal function calls where we can guarantee as implementors that the pointers
        // passed to the `new` is a valid one.
        unsafe { (*self.buff).state }
    }

    pub fn get_length(&self) -> i32 {
        // Safety: self.buff is a valid pointer. This is because, the structure can only be created
        // through internal function calls where we can guarantee as implementors that the pointers
        // passed to the `new` is a valid one.
        unsafe { (*self.buff).len }
    }

    pub fn get_payload_size(&self) -> i32 {
        // Safety: self.buff is a valid pointer. This is because, the structure can only be created
        // through internal function calls where we can guarantee as implementors that the pointers
        // passed to the `new` is a valid one.
        unsafe { rmr_int::rmr_payload_size(self.buff) }
    }

    pub fn get_payload(&self) -> &[u8] {
        // Safety: self.buff is a valid pointer. This is because, the structure can only be created
        // through internal function calls where we can guarantee as implementors that the pointers
        // passed to the `new` is a valid one.
        unsafe {
            let size = (*self.buff).len as usize;
            std::slice::from_raw_parts((*self.buff).payload, size)
        }
    }

    pub fn get_msgtype(&self) -> i32 {
        self.msgtype
    }
}

unsafe impl Send for RMRMessageBuffer {}
unsafe impl Sync for RMRMessageBuffer {}
