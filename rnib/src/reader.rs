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

//! RNIB : Implements functionality for the RNIB Database. Uses internally SDL APIs

#[allow(unused)]
use crate::entities::*;

#[derive(Debug)]
pub struct RnibError(String);

impl std::fmt::Display for RnibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SdlError: {}", self.0)
    }
}

impl std::convert::From<String> for RnibError {
    fn from(s: String) -> Self {
        Self(format!("RnibError: {}", s))
    }
}

/// Publc RNIB API. An implementor of this API will implement this trait.
pub trait RnibApi {
    /// The Namespace for the RNIB in the SDL
    const NS: &'static str;

    /// Get the
    fn get_nodeb(&mut self, inventory: &str) -> Result<NodebInfo, RnibError>;

    fn get_nodeb_by_global_nbid(
        &mut self,
        typ: node::Type,
        nbid: &GlobalNbId,
    ) -> Result<NodebInfo, RnibError>;

    fn get_cell_list(&mut self, inventory: &str) -> Result<Cells, RnibError>;

    fn get_gnb_ids(&mut self) -> Result<Vec<NbIdentity>, RnibError>;

    fn get_enb_ids(&mut self) -> Result<Vec<NbIdentity>, RnibError>;

    fn get_cell_by_id(&mut self, typ: cell::Type, cell_id: &str) -> Result<Vec<Cell>, RnibError>;

    fn get_nodeb_ids(&mut self) -> Result<Vec<NbIdentity>, RnibError>;

    fn get_ran_load_info(&mut self, inventory: &str) -> Result<RanLoadInformation, RnibError>;

    // TODO:
    // fn get_e2t_instance(&mut self, addr: &str) -> Result<E2TInstance, RnibError>;

    // TODO:
    // fn get_e2t_instances(&mut self, addrs: &[&str]) -> Result<Vec<E2TInstance>, RnibError>;
}
