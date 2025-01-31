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

use crate::internal_prelude::*;

#[ext_sized]
pub impl NetworkDefinition {
    fn from_network_id(network_id: u8) -> Self {
        match network_id {
            // Public facing networks
            0x01 => NetworkDefinition::mainnet(),
            0x02 => NetworkDefinition {
                id: network_id,
                logical_name: "stokenet".into(),
                hrp_suffix: "tdx_2_".into(),
            },

            // Babylon Temporary Testnets
            0x0A => NetworkDefinition::adapanet(),
            0x0B => NetworkDefinition::nebunet(),
            0x0C => NetworkDefinition {
                id: network_id,
                logical_name: "kisharnet".into(),
                hrp_suffix: "tdx_c_".into(),
            },
            0x0D => NetworkDefinition {
                id: network_id,
                logical_name: "ansharnet".into(),
                hrp_suffix: "tdx_d_".into(),
            },
            0x0E => NetworkDefinition {
                id: network_id,
                logical_name: "ansharnet".into(),
                hrp_suffix: "tdx_e_".into(),
            },

            // RDX Works Development
            0x20 => NetworkDefinition {
                id: 0x20,
                logical_name: "gilganet".into(),
                hrp_suffix: "tdx_20_".into(),
            },
            0x21 => NetworkDefinition {
                id: 0x21,
                logical_name: "enkinet".into(),
                hrp_suffix: "tdx_21_".into(),
            },
            0x22 => NetworkDefinition {
                id: 0x22,
                logical_name: "hammunet".into(),
                hrp_suffix: "tdx_22_".into(),
            },
            0x23 => NetworkDefinition {
                id: 0x23,
                logical_name: "nergalnet".into(),
                hrp_suffix: "tdx_23_".into(),
            },
            0x24 => NetworkDefinition {
                id: 0x24,
                logical_name: "mardunet".into(),
                hrp_suffix: "tdx_24_".into(),
            },
            0x25 => NetworkDefinition {
                id: 0x25,
                logical_name: "dumunet".into(),
                hrp_suffix: "tdx_25_".into(),
            },

            // Ephemeral Networks
            0xF0 => NetworkDefinition {
                id: 240,
                logical_name: "localnet".into(),
                hrp_suffix: "loc".into(),
            },
            0xF1 => NetworkDefinition {
                id: 241,
                logical_name: "inttestnet".into(),
                hrp_suffix: "test".into(),
            },
            0xF2 => NetworkDefinition::simulator(),

            // Unnamed
            network_id => NetworkDefinition {
                id: network_id,
                logical_name: "unnamed".into(),
                hrp_suffix: format!("tdx_{:x}_", network_id).into(),
            },
        }
    }

    fn from_hrp(hrp: impl AsRef<str>) -> Option<Self> {
        let network_specifier = {
            let re = Regex::new("_(sim|loc|rdx|test|tdx_[A-Fa-f0-9]{1,2}_)$")
                .expect("Failed to create Regex. Must panic");
            re.captures(hrp.as_ref())
                .and_then(|captures| captures.get(1))
                .map(|capture| capture.as_str().trim_end_matches('_'))
        };

        let network_id = match network_specifier {
            Some("rdx") => Some(0x01),
            Some("loc") => Some(0xF0),
            Some("test") => Some(0xF1),
            Some("sim") => Some(0xF2),
            Some(numeric_network_specifier) => {
                if let Some(network_id_string) =
                    numeric_network_specifier.split('_').nth(1)
                {
                    if let Ok(num) = u8::from_str_radix(network_id_string, 16) {
                        Some(num)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            None => None,
        };
        network_id.map(Self::from_network_id)
    }

    fn from_address_string(address: impl AsRef<str>) -> Option<Self> {
        bech32::decode(address.as_ref())
            .ok()
            .and_then(|(hrp, ..)| Self::from_hrp(hrp))
    }
}
