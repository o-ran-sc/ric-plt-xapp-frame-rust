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
//!
//! There are two types of metrics supported currently - Counters and Gauges. Counters are used for
//! monotonically increasing integer values. Gauges are to be used for values within a given range
//! (eg. CPU utlization would be a Gauge, interrupts will be a counter.).
//!
//! XApp can register custom Counters and Gauges using `register_counter` and `register_gauge`
//! methods respectively. XApp can then use the public APIs on the XApp class to increment the
//! counters or update (increment or decrement the guages). In addition it's possible to `set` the
//! values of the Counters and Gauges.
//!
//! All the registered counters and guages along with default `rmr_messages_rx` and
//! `rmr_messages_tx` counters can be scraped through registry. This is available via
//! `/ric/v1/metrics` end point of the XApp.

use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;

use tokio::sync::mpsc::Sender as TokioSyncSender;

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};

use crate::{XApp, XAppError};

// RMR Messages
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct RMRMessage {
    message_type: i32,
}

impl XApp {
    /// Increment the internal RMR Received Messages Counter for a given message type.
    pub fn increment_rmr_rx_messages(&self, message_type: i32) {
        if let Some(ref metrics) = self.metrics {
            let metrics = metrics.lock().unwrap();
            metrics.increment_rmr_rx_messages(message_type);
        }
    }

    /// Increment the internal RMR Sent Messages Counter for a given message type.
    pub fn increment_rmr_tx_messages(&self, message_type: i32) {
        if let Some(ref metrics) = self.metrics {
            let metrics = metrics.lock().unwrap();
            metrics.increment_rmr_tx_messages(message_type);
        }
    }

    /// Increment the counter that was dynamically registered with `register_counter`
    pub fn increment_counter<S: Into<String>>(&self, counter_name: &str, labels: (S, S)) {
        if let Some(ref metrics) = self.metrics {
            let metrics = metrics.lock().unwrap();
            if let Some(counter_family) = metrics.counters.get(counter_name) {
                counter_family
                    .get_or_create(&vec![(labels.0.into(), labels.1.into())])
                    .inc();
            }
        }
    }

    /// Increment a Gauge value by given value.
    pub fn increment_gauge<S: Into<String>>(&self, gauge_name: &str, labels: (S, S), value: f64) {
        if let Some(ref metrics) = self.metrics {
            let metrics = metrics.lock().unwrap();
            if let Some(gauge_family) = metrics.gauges.get(gauge_name) {
                gauge_family
                    .get_or_create(&vec![(labels.0.into(), labels.1.into())])
                    .inc_by(value);
            }
        }
    }

    /// Decrement a Gauge value by given value.
    pub fn decrement_gauge<S: Into<String>>(&self, gauge_name: &str, labels: (S, S), value: f64) {
        if let Some(ref metrics) = self.metrics {
            let metrics = metrics.lock().unwrap();
            if let Some(gauge_family) = metrics.gauges.get(gauge_name) {
                gauge_family
                    .get_or_create(&vec![(labels.0.into(), labels.1.into())])
                    .dec_by(value);
            }
        }
    }

    /// Sets the value of the Gauge to the given value.
    pub fn set_gauge<S: Into<String>>(&self, gauge_name: &str, labels: (S, S), value: f64) {
        if let Some(ref metrics) = self.metrics {
            let metrics = metrics.lock().unwrap();
            if let Some(gauge_family) = metrics.gauges.get(gauge_name) {
                gauge_family
                    .get_or_create(&vec![(labels.0.into(), labels.1.into())])
                    .set(value);
            }
        }
    }

    /// Register a Counter with a given name and help message
    pub fn register_counter(&mut self, counter_name: &str, counter_help: &str) {
        if let Some(ref mut metrics) = self.metrics {
            let mut metrics = metrics.lock().unwrap();
            metrics.register_counter(counter_name, counter_help)
        }
    }

    /// Register a Gauge with a given name and help message
    ///
    /// Currently float gauges are supported.
    pub fn register_gauge(&mut self, counter_name: &str, counter_help: &str) {
        if let Some(ref mut metrics) = self.metrics {
            let mut metrics = metrics.lock().unwrap();
            metrics.register_gauge(counter_name, counter_help)
        }
    }
}

pub(crate) struct MetricsRegistry {
    registry: Registry,
    rmr_messages_rx: Family<RMRMessage, Counter>,
    rmr_messages_tx: Family<RMRMessage, Counter>,
    counters: HashMap<String, Family<Vec<(String, String)>, Counter>>,
    gauges: HashMap<String, Family<Vec<(String, String)>, Gauge<f64, AtomicU64>>>,
}

pub(crate) fn run_metrics_server(
    metrics: Arc<Mutex<MetricsRegistry>>,
    data_tx: TokioSyncSender<String>,
    app_is_running: Arc<AtomicBool>,
) -> Result<(), XAppError> {
    loop {
        let metrics = metrics.lock().unwrap();
        let metrics_data = metrics.encode();
        let _ = data_tx.blocking_send(metrics_data);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if !app_is_running.load(Ordering::SeqCst) {
            break;
        }
    }
    Ok(())
}

pub(crate) fn registry_for_ns_app(ns: &str, app_name: &str) -> Result<MetricsRegistry, XAppError> {
    let app_name = app_name.replace('-', "_");
    let prefix = format!("{ns}_{app_name}");
    let mut registry = Registry::with_prefix(prefix);

    let rmr_messages_rx = Family::<RMRMessage, Counter>::default();
    registry.register(
        // With the metric name.
        "rmr_messages_rx",
        // And the metric help text.
        "Number of RMR messages received",
        rmr_messages_rx.clone(),
    );

    let rmr_messages_tx = Family::<RMRMessage, Counter>::default();
    registry.register(
        // With the metric name.
        "rmr_messages_tx",
        // And the metric help text.
        "Number of RMR messages transmitted",
        rmr_messages_tx.clone(),
    );

    Ok(MetricsRegistry {
        registry,
        rmr_messages_rx,
        rmr_messages_tx,
        counters: HashMap::new(),
        gauges: HashMap::new(),
    })
}

impl MetricsRegistry {
    pub(crate) fn register_counter(&mut self, counter_name: &str, counter_help: &str) {
        let counter_family = Family::<Vec<(String, String)>, Counter>::default();

        self.registry
            .register(counter_name, counter_help, counter_family.clone());

        self.counters
            .insert(counter_name.to_owned(), counter_family);
    }

    pub(crate) fn register_gauge(&mut self, gauge_name: &str, gauge_help: &str) {
        let gauge_family = Family::<Vec<(String, String)>, Gauge<f64, AtomicU64>>::default();

        self.registry
            .register(gauge_name, gauge_help, gauge_family.clone());

        self.gauges.insert(gauge_name.to_owned(), gauge_family);
    }

    pub(crate) fn increment_rmr_rx_messages(&self, message_type: i32) {
        self.rmr_messages_rx
            .get_or_create(&RMRMessage { message_type })
            .inc();
    }

    pub(crate) fn increment_rmr_tx_messages(&self, message_type: i32) {
        self.rmr_messages_tx
            .get_or_create(&RMRMessage { message_type })
            .inc();
    }

    pub(crate) fn encode(&self) -> String {
        let mut buffer = String::new();
        encode(&mut buffer, &self.registry).unwrap();
        buffer
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_metrics_in_xapp() {
        let config = crate::xapp::tests::get_config_data(5678_u16);

        let xapp = crate::XApp::from_config(config);
        assert!(xapp.is_ok(), "{:?}", xapp.err().unwrap());
        let mut xapp = xapp.unwrap();

        assert!(xapp.metrics.is_some());

        xapp.register_counter("xapp-test-counter", "Test counter for XApp API");

        // TODO : When full integration tests are supported we should be able to get the metrics.
    }

    #[test]
    fn test_metrics_register_counter() {
        let metrics = super::registry_for_ns_app("ricxapp", "test-counters-app");
        assert!(metrics.is_ok(), "{:?}", metrics.err().unwrap());
        let mut metrics = metrics.unwrap();

        metrics.register_counter("test_counter", "This is a Test Counter");

        assert!(metrics.counters.len() == 1);
        assert!(metrics.counters.get("test_counter").is_some());

        let labels = vec![("method".to_owned(), "GET".to_owned())];
        let tc_family = metrics.counters.get("test_counter").unwrap();
        let value = tc_family.get_or_create(&labels);
        assert!(value.get() == 0, "{:}", value.get());
    }

    #[test]
    fn test_metrics_register_gauge() {
        let metrics = super::registry_for_ns_app("ricxapp", "test-gauges-app");
        assert!(metrics.is_ok(), "{:?}", metrics.err().unwrap());
        let mut metrics = metrics.unwrap();

        metrics.register_gauge("test_gauge", "This is a Test Gauge");

        assert!(metrics.gauges.len() == 1);
        assert!(metrics.gauges.get("test_gauge").is_some());
        let labels = vec![("snr".to_owned(), "upstream".to_owned())];
        let tc_family = metrics.gauges.get("test_gauge").unwrap();
        let value = tc_family.get_or_create(&labels);
        assert!(value.get() == 0.0, "{:}", value.get());
    }

    #[test]
    fn test_rmr_rx_tx_messages() {
        let metrics = super::registry_for_ns_app("ricxapp", "test-rmr-tx-rx-app");
        assert!(metrics.is_ok(), "{:?}", metrics.err().unwrap());
        let metrics = metrics.unwrap();

        metrics.increment_rmr_tx_messages(1);
        metrics.increment_rmr_rx_messages(1);

        let rx_ctr = metrics
            .rmr_messages_rx
            .get_or_create(&super::RMRMessage { message_type: 1 });
        assert!(rx_ctr.get() == 1, "{:?}", rx_ctr.get());

        let tx_ctr = metrics
            .rmr_messages_tx
            .get_or_create(&super::RMRMessage { message_type: 1 });
        assert!(tx_ctr.get() == 1, "{:?}", tx_ctr.get());
    }

    #[test]
    fn test_encode() {
        let metrics = super::registry_for_ns_app("ricxapp", "test-rmr-tx-rx-app");
        assert!(metrics.is_ok(), "{:?}", metrics.err().unwrap());
        let mut metrics = metrics.unwrap();

        metrics.increment_rmr_tx_messages(1);
        metrics.increment_rmr_rx_messages(1);

        metrics.register_counter("test_encode", "Counter for testing encode function");

        let s = metrics.encode();

        let expected = r#"# HELP ricxapp_test_rmr_tx_rx_app_rmr_messages_rx Number of RMR messages received.
# TYPE ricxapp_test_rmr_tx_rx_app_rmr_messages_rx counter
ricxapp_test_rmr_tx_rx_app_rmr_messages_rx_total{message_type="1"} 1
# HELP ricxapp_test_rmr_tx_rx_app_rmr_messages_tx Number of RMR messages transmitted.
# TYPE ricxapp_test_rmr_tx_rx_app_rmr_messages_tx counter
ricxapp_test_rmr_tx_rx_app_rmr_messages_tx_total{message_type="1"} 1
# HELP ricxapp_test_rmr_tx_rx_app_test_encode Counter for testing encode function.
# TYPE ricxapp_test_rmr_tx_rx_app_test_encode counter
# EOF
"#;

        assert_eq!(s, expected.to_owned(), "{:#?}", s);
    }
}
