// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

/// A constant representing the length of Radix Engine Node Identifiers (`RENodeId`s)
pub const RADIX_ENGINE_NODE_IDENTIFIER_LENGTH: usize = 36;

/// A constant representing the length of hashes uses by the Radix Engine and Scrypto. This is
/// defined as a constant to allow for easy changes if the hashing algorithm uses is changed.
pub const RADIX_ENGINE_HASH_LENGTH: usize = 32;
