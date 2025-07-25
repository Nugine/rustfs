#![deny(clippy::unwrap_used)]
// Copyright 2024 RustFS Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod encdec;
mod error;
mod jwt;

pub use encdec::decrypt::decrypt_data;
pub use encdec::encrypt::encrypt_data;
pub use error::Error;
pub use jwt::decode::decode as jwt_decode;
pub use jwt::encode::encode as jwt_encode;
