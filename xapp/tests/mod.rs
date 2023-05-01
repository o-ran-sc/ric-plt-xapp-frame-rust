#![cfg(test)]

use rmr::RMRClient;
use xapp::XApp;

#[test]
fn test_xapp_start_rmr_nt_ready_stop() {
    let xapp = XApp::new("4560", RMRClient::RMRFL_NONE);
    assert!(xapp.is_ok());

    let mut xapp = xapp.unwrap();

    xapp.start();

    assert_eq!(xapp.is_rmr_ready(), false);

    xapp.stop();

    xapp.join();

    assert!(true);
}
