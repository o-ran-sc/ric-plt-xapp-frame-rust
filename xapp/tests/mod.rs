#![cfg(test)]

use rmr::RMRClient;
use xapp::XApp;

#[test]
fn test_simple_xapp_start_stop() {
    let xapp = XApp::new("4560", RMRClient::RMRFL_NONE);
    assert!(xapp.is_ok());

    let mut xapp = xapp.unwrap();

    xapp.start();

    xapp.stop();

    xapp.join();

    assert!(true);
}
