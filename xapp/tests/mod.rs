#![cfg(test)]

use xapp::XApp;

fn get_config_data() -> xapp::XAppConfig {
    let config_json = r#"{
        "messaging": {
            "ports" : [
                {
                    "name": "rmrdata",
                    "port": 4560
                }
            ]
        }
    }"#;

    xapp::XAppConfig {
        metadata: Box::new(xapp::ConfigMetadata {
            xapp_name: "int-tests".to_string(),
            config_type: "json".to_string(),
        }),
        config: serde_json::from_str(config_json).unwrap(),
    }
}

#[test]
fn test_xapp_start_rmr_nt_ready_stop() {
    let xapp = XApp::from_config(get_config_data());
    assert!(xapp.is_ok());

    let mut xapp = xapp.unwrap();

    xapp.start();

    assert_eq!(xapp.is_rmr_ready(), false);

    // Added to make sure that the function can be called.
    // Since we are not starting any DB, this should return err.
    // For the test it's okay.
    let result = xapp.rnib_get_nodeb_ids();
    assert!(result.is_err(), "{:#?}", result.ok().unwrap());

    xapp.stop();

    xapp.join();

    assert!(true);
}
