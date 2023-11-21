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

/// XAppError: Errors during execution of the framework code.
#[derive(Debug)]
pub struct XAppError(pub(crate) String);

impl std::fmt::Display for XAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<XAppError> for std::io::Error {
    fn from(x: XAppError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, format!("{}", x))
    }
}

impl From<rmr::RMRError> for XAppError {
    fn from(_r: rmr::RMRError) -> Self {
        XAppError("RMRError".to_string())
    }
}

impl From<rnib::RnibError> for XAppError {
    fn from(r: rnib::RnibError) -> Self {
        XAppError(format!("RNIB Error: {}", r))
    }
}
