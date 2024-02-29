// ==================================================================================
//   Copyright (c) 2024 Abhijit Gadgil
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

//! Metrics support for the XApp
//!
//! This module is to be used for enabling exporting `metrics` from the XApp.
//!
//! All the metrics for a given XApp are prefixed with `XAppName` and `XAppNameSpace` prefix along
//! with the actual metrics name.
//!
//! By Default, we
//! provide, `rmr_messages_rx` and `rmr_messages_tx` (with Label `message_type`) metrics available
//! to app XApps.
//!
//! Example:
//!
//! An XApp called `hw-rust` running in `ricplt` namespace will have the following metrics always
//! available served from the `/ric/v1/metrics` url.
//!
//! `ricplt_hw_rust_rmr_messages_rx` and `ricplt_hw_rust_rmr_messages_tx`. Optionally other metrics
//! can be added.

use prometheus_client::registry::Registry;

use crate::XAppError;

pub(crate) fn registry_for_ns_app(ns: &str, app_name: &str) -> Result<Registry, XAppError> {
    let app_name = app_name.replace('-', "_");
    let prefix = format!("{ns}_{app_name}");
    let mut registry = Registry::with_prefix(prefix);

    Err(XAppError("Something".to_string()))
}
