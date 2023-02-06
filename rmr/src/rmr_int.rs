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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use std::convert::TryInto;
use std::ffi::CString;
use std::sync::Mutex;

use crate::{RMRClient, RMRError, RMRMessageBuffer};

type RMRContext = *mut ::std::os::raw::c_void;

static mut CONTEXT: RMRContext = std::ptr::null_mut();
// A Mutex controlling the creation of client.
static CLIENT_MUTEX: Mutex<()> = Mutex::new(());

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub(crate) fn rmr_client_new_internal(
    port: &str,
    max_size: u32,
    flags: u32,
) -> Result<RMRClient, RMRError> {
    // This is to make sure if two 'threads' are trying to create an `RMRClient` at the same
    // time, cannot do it.
    let _guard = CLIENT_MUTEX.lock().unwrap();
    // Safety: We are only checking for NULL and not de-referencing.
    unsafe {
        if !CONTEXT.is_null() {
            // CONTEXT is Not Null. It means someone else has already initialized the client. We cannot
            // re-initialize it.
            return Err(RMRError);
        }
    }

    let port_chars = CString::new(port).unwrap();
    // Safety: CONTEXT is guaranteed to be NULL - which means -
    // a) No one has created a client before.
    // b) A Client created previously has gone out of scope and hence it's okay to re-initialize
    // RMR.
    unsafe {
        eprintln!("2");
        CONTEXT = rmr_init(
            port_chars.into_raw(),
            max_size.try_into().unwrap(),
            flags.try_into().unwrap(),
        );
        if CONTEXT.is_null() {
            Err(RMRError)
        } else {
            Ok(RMRClient { _empty: () })
        }
    }
}

pub(crate) fn is_ready_internal() -> bool {
    // Safety: We are making sure that only one client can be constructed and the following
    // `CONTEXT` can be only accessed through that client.
    unsafe { rmr_ready(CONTEXT) == 1 }
}

pub(crate) fn get_recv_fd_internal() -> Result<i32, RMRError> {
    // Safety: We are making sure that only one client can be constructed and the following
    // `CONTEXT` can be only accessed through that client.
    unsafe {
        let fd = rmr_get_rcvfd(CONTEXT);
        if fd < 0 {
            Err(RMRError)
        } else {
            Ok(fd)
        }
    }
}

pub(crate) fn alloc_message_internal() -> Result<*mut rmr_mbuf_t, RMRError> {
    // Safety: We are making sure that only one client can be constructed and the following
    // `CONTEXT` can be only accessed through that client.
    unsafe {
        let buff = rmr_alloc_msg(CONTEXT, 4096);
        if buff.is_null() {
            Err(RMRError)
        } else {
            Ok(buff)
        }
    }
}

pub(crate) fn rcv_msg_internal(buff: *mut rmr_mbuf_t) -> Result<*mut rmr_mbuf_t, RMRError> {
    // Safety: We are making sure that only one client can be constructed and the following
    // `CONTEXT` can be only accessed through that client.
    unsafe {
        let buff = rmr_rcv_msg(CONTEXT, buff);
        if buff.is_null() {
            Err(RMRError)
        } else {
            Ok(buff)
        }
    }
}

pub(crate) fn rts_msg_internal(msg: &RMRMessageBuffer) -> Result<(), RMRError> {
    // Safety: We are making sure that only one client can be constructed and the following
    // `CONTEXT` can be only accessed through that client.
    unsafe {
        let buff = msg.buff;
        let send_buff = rmr_rts_msg(CONTEXT, buff);
        if send_buff.is_null() {
            Err(RMRError)
        } else {
            Ok(())
        }
    }
}

pub(crate) fn rmr_close_internal() {
    // Safety: We are making sure that only one client can be constructed and the following
    // `CONTEXT` can be only accessed through that client.
    unsafe {
        rmr_close(CONTEXT);
        CONTEXT = std::ptr::null_mut();
    }
}
