// ==================================================================================
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

//! Redis SDL Backend

use std::env;
use std::iter::{zip, FromIterator};
use std::str::FromStr;

use redis::Commands;

use crate::{DataMap, KeySet, SdlError, SdlStorageApi, ValueType};

const SERVICE_HOST_NAME_ENV_VAR: &str = "DBAAS_SERVICE_HOST";
const SERVICE_PORT_NAME_ENV_VAR: &str = "DBAAS_SERVICE_PORT";
// TODO: Support following Env Variables
// - "DBAAS_SERVICE_SENTINEL_PORT"
// - "DBAAS_SERVICE_MASTER_NAME"
// - "DBAAS_SERVICE_NODE_COUNT
// - "DBAAS_SERVICE_CLUSTER_ADDR_LIST"

// Internal structure holding a redis hostname and port used to connect to redis server.
#[derive(Debug)]
struct RedisHostPort {
    host: String,
    port: u16,
}

/// SdlConfig: Representing SDL Configuration received from the environment variables `DBAAS_*`
struct SdlRedisConfig {
    host_ports: Vec<RedisHostPort>,
}

impl SdlRedisConfig {
    fn from_env() -> Result<Self, SdlError> {
        if env::var(SERVICE_HOST_NAME_ENV_VAR).is_err() {
            log::warn!("Environment variable `DBAAS_SERVICE_HOST` not set. Using default value for the host name 'dbaas'");
        }
        if env::var(SERVICE_PORT_NAME_ENV_VAR).is_err() {
            log::warn!("Environment variable `DBAAS_SERVICE_PORT` not set. Using default value for the port number '6379'");
        }
        let hostnames = env::var(SERVICE_HOST_NAME_ENV_VAR).unwrap_or_else(|_| "dbaas".to_string());
        let hostnames = hostnames
            .split(',')
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let mut parse_errors = vec![];
        let mut ports = env::var(SERVICE_PORT_NAME_ENV_VAR)
            .unwrap_or_else(|_| "6379".to_string())
            .split(',')
            .map(u16::from_str)
            .inspect(|x| parse_errors.extend(x.clone().err()))
            .filter_map(Result::ok)
            .collect::<Vec<u16>>();
        if !parse_errors.is_empty() {
            return Err(SdlError::from(
                "Port Numbers should be integer.".to_string(),
            ));
        }

        if ports.len() > hostnames.len() {
            let err_str = "Redis Config: Specified ports should not be more than specified hosts.";
            log::error!("{}", err_str);
            // Number of ports cannot be more than number of hostnames.
            Err(SdlError::from(err_str.to_string()))
        } else {
            // Fill out the last port_number to make the ports count same as hostnames
            // This will typically happen with multiple hostnames and a single port number like the
            // default port no. 6379.
            let mut remaining = hostnames.len() - ports.len();
            let last_portnum = ports.last().unwrap();
            let last_portnum = *last_portnum;
            loop {
                if remaining == 0 {
                    break;
                }

                ports.push(last_portnum);
                remaining -= 1;
            }

            let mut host_ports = vec![];
            for (host, port) in zip(hostnames, ports) {
                host_ports.push(RedisHostPort { host, port });
            }
            log::debug!("Created Redis SDL Config with: {:?}", host_ports);
            Ok(SdlRedisConfig { host_ports })
        }
    }
}

pub struct RedisStorage {
    dbs: Vec<redis::Client>,
    is_ready: bool,
}

impl RedisStorage {
    /// Create a new instance of the `RedisStorage` from current environment (`DBAAS_SERVICE_HOST`,
    /// and `DBAAS_SERVICE_PORT`) values.
    ///
    /// Checks for environment variables `DBAAS_SERVICE_HOST` and `DBAAS_SERVICE_PORT` for a comma
    /// separated list of values and uses (host, port) tuple as Redis DB connection string. Default
    /// values are ('dbaas', 6379) if the environment variables are not set.
    pub fn new_from_env() -> Result<Self, SdlError> {
        let config = SdlRedisConfig::from_env()?;

        RedisStorage::from_config(config)
    }

    fn from_config(config: SdlRedisConfig) -> Result<Self, SdlError> {
        let mut dbs = vec![];
        for host_port in config.host_ports {
            let connect_string = format!("redis://{}:{}/", host_port.host, host_port.port);
            let client = redis::Client::open(connect_string);
            if client.is_err() {
                log::error!("Redis Client Error: {}", client.as_ref().err().unwrap());
                return Err(SdlError::from(format!(
                    "RedisClientError: {}",
                    client.err().unwrap()
                )));
            }
            dbs.push(client.unwrap());
        }

        Ok(RedisStorage {
            dbs,
            is_ready: false,
        })
    }

    // for the redis backend, we will have to use the same keys as are being used by the Go and
    // Python frameworks, otherwise we won't be able to read keys used by code written in SDK used
    // in other languages.
    fn key_from_ns_and_key(namespace: &str, key: &str) -> String {
        format!("{{{}}},{}", namespace, key)
    }

    fn db_handle_for_ns(&mut self, namespace: &str) -> Result<&mut redis::Client, SdlError> {
        if self.dbs.is_empty() {
            Err(SdlError::from("Not connected to any DB!".to_string()))
        } else {
            let id = crc32fast::hash(namespace.as_bytes());
            let bucket = id as usize % self.dbs.len();
            let db_client = self.dbs.get_mut(bucket);
            match db_client {
                Some(client) => Ok(client),
                None => Err(SdlError::from(format!(
                    "DB handle for namespace: {} not found!",
                    namespace
                ))),
            }
        }
    }
}

impl SdlStorageApi for RedisStorage {
    fn is_ready(&mut self, namespace: &str) -> bool {
        if self.is_ready {
            self.is_ready
        } else {
            let result = self.list_keys(namespace, "*");
            self.is_ready = result.is_ok();

            self.is_ready
        }
    }

    fn set(&mut self, namespace: &str, data: &DataMap) -> Result<(), SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let dataset = data
            .iter()
            .map(|(k, v)| (Self::key_from_ns_and_key(namespace, k), v))
            .collect::<Vec<(String, _)>>();

        db.mset::<_, _, ()>(&dataset)
            .map_err(|e| SdlError::from(e.to_string()))
    }

    fn set_if_not_exists(
        &mut self,
        namespace: &str,
        key: &str,
        value: &[u8],
    ) -> Result<(), SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        db.set_nx::<_, Vec<u8>, ()>(Self::key_from_ns_and_key(namespace, key), value.to_vec())
            .map_err(|e| SdlError::from(e.to_string()))?;

        Ok(())
    }

    fn get(&mut self, namespace: &str, keys: &KeySet) -> Result<DataMap, SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let db_keys = keys
            .iter()
            .map(|k| Self::key_from_ns_and_key(namespace, k))
            .collect::<Vec<String>>();
        let db_values = db
            .mget::<_, Vec<Vec<u8>>>(db_keys)
            .map_err(|e| SdlError::from(e.to_string()))?;

        let data_map = zip(keys, db_values).map(|(k, v)| (k.clone(), v)).collect();
        Ok(data_map)
    }

    fn delete(&mut self, namespace: &str, keys: &KeySet) -> Result<(), SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let db_keys = keys
            .iter()
            .map(|k| Self::key_from_ns_and_key(namespace, k))
            .collect::<Vec<String>>();
        db.del::<_, ()>(db_keys)
            .map_err(|e| SdlError::from(e.to_string()))?;

        Ok(())
    }

    fn delete_all(&mut self, namespace: &str) -> Result<(), SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let all_keys_pattern = Self::key_from_ns_and_key(namespace, "*");
        let db_keys = db
            .scan_match(all_keys_pattern)
            .map_err(|e| SdlError::from(e.to_string()))?
            .collect::<Vec<String>>();

        db.del::<_, ()>(db_keys)
            .map_err(|e| SdlError::from(e.to_string()))?;

        Ok(())
    }

    fn delete_if(&mut self, namespace: &str, key: &str, value: &[u8]) -> Result<bool, SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let stored_value = db
            .get::<_, Vec<u8>>(Self::key_from_ns_and_key(namespace, key))
            .map_err(|e| SdlError::from(e.to_string()))?;

        Ok(stored_value == value)
    }

    fn list_keys(&mut self, namespace: &str, pattern: &str) -> Result<KeySet, SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let keys_pattern = Self::key_from_ns_and_key(namespace, pattern);
        let db_keys = db
            .scan_match::<_, String>(keys_pattern)
            .map_err(|e| SdlError::from(e.to_string()))?;

        Ok(KeySet::from_iter(db_keys))
    }

    fn add_member(
        &mut self,
        namespace: &str,
        group: &str,
        value: &ValueType,
    ) -> Result<(), SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let set_name = Self::key_from_ns_and_key(namespace, group);
        db.sadd(set_name, value)
            .map_err(|e| SdlError::from(e.to_string()))?;
        Ok(())
    }

    fn delete_member(
        &mut self,
        namespace: &str,
        group: &str,
        value: &ValueType,
    ) -> Result<(), SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let set_name = Self::key_from_ns_and_key(namespace, group);
        db.srem(set_name, value)
            .map_err(|e| SdlError::from(e.to_string()))?;
        Ok(())
    }

    fn get_members(&mut self, namespace: &str, group: &str) -> Result<Vec<Vec<u8>>, SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let set_name = Self::key_from_ns_and_key(namespace, group);
        let members = db
            .smembers(set_name)
            .map_err(|e| SdlError::from(e.to_string()))?;
        Ok(members)
    }

    fn del_group(&mut self, namespace: &str, group: &str) -> Result<(), SdlError> {
        let db = self.db_handle_for_ns(namespace)?;
        let set_name = Self::key_from_ns_and_key(namespace, group);
        db.del::<_, ()>(set_name)
            .map_err(|e| SdlError::from(e.to_string()))?;

        Ok(())
    }
}
