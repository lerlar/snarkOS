// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChallengeResponse<N: Network> {
    pub signature: Data<Signature<N>>,
}

impl<N: Network> EventTrait for ChallengeResponse<N> {
    /// Returns the event name.
    #[inline]
    fn name(&self) -> String {
        "ChallengeResponse".to_string()
    }

    /// Serializes the event into the buffer.
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.signature.serialize_blocking_into(writer)
    }

    /// Deserializes the given buffer into a event.
    #[inline]
    fn deserialize(bytes: BytesMut) -> Result<Self> {
        let reader = bytes.reader();
        Ok(Self { signature: Data::Buffer(reader.into_inner().freeze()) })
    }
}
