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

//! Implementation of RNIB Reader API for the Redis SDL Backend.

use prost::{DecodeError, Message};

use rnib::entities::*;
use rnib::{RnibApi, RnibError};

use crate::{RedisStorage, SdlStorageApi};

impl RnibApi for RedisStorage {
    const NS: &'static str = "e2Manager";

    fn get_nodeb(&mut self, _inventory: &str) -> Result<NodebInfo, RnibError> {
        todo!();
    }

    fn get_nodeb_by_global_nbid(
        &mut self,
        _typ: node::Type,
        _nbid: &GlobalNbId,
    ) -> Result<NodebInfo, RnibError> {
        todo!();
    }

    fn get_cell_list(&mut self, _inventory: &str) -> Result<Cells, RnibError> {
        todo!();
    }

    fn get_gnb_ids(&mut self) -> Result<Vec<NbIdentity>, RnibError> {
        self.get_nb_ids_by_type(node::Type::Gnb.as_str_name())
    }

    fn get_enb_ids(&mut self) -> Result<Vec<NbIdentity>, RnibError> {
        self.get_nb_ids_by_type(node::Type::Enb.as_str_name())
    }

    fn get_cell_by_id(&mut self, _typ: cell::Type, _cell_id: &str) -> Result<Vec<Cell>, RnibError> {
        todo!();
    }

    fn get_nodeb_ids(&mut self) -> Result<Vec<NbIdentity>, RnibError> {
        let mut nbids = self.get_enb_ids()?;
        nbids.extend(self.get_gnb_ids()?);

        Ok(nbids)
    }

    fn get_ran_load_info(&mut self, _inventory: &str) -> Result<RanLoadInformation, RnibError> {
        todo!();
    }
}

impl RedisStorage {
    fn get_nb_ids_by_type(&mut self, typ: &str) -> Result<Vec<NbIdentity>, RnibError> {
        let nbids: _ = self
            .get_members(Self::NS, typ)
            .map_err(|e| RnibError::from(e.to_string()))?
            .iter()
            .map(|m| NbIdentity::decode(m.as_slice()))
            .collect::<Vec<Result<NbIdentity, DecodeError>>>();

        if nbids
            .iter()
            .filter(|v| v.is_err())
            .map(|e| e.as_ref().err().unwrap())
            .next()
            .is_some()
        {
            Err(RnibError::from("NodebIdentityDecodeError:".to_string()))
        } else {
            let nbids = nbids
                .iter()
                .filter(|v| v.is_ok())
                .map(|v| v.as_ref().ok().unwrap().clone())
                .collect();

            Ok(nbids)
        }
    }
}
