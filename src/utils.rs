use crate::error::Error;
use scrypto::prelude::NetworkDefinition;
use serde::Deserialize;

pub unsafe fn prepare_request<'t, T>(
    request_string_pointer: *const std::os::raw::c_char,
) -> Result<T, Error>
where
    T: Deserialize<'t>,
{
    let string: &str = std::ffi::CStr::from_ptr(request_string_pointer).to_str()?;
    Ok(serde_json::from_str(string)?)
}

/// A deterministic function that generates a network definition given a network ID. Implemented with reference to
/// https://github.com/radixdlt/babylon-node/blob/51e4fb9dbb999b8e02aa6cce07162aef2affd6a7/common/src/main/java/com/radixdlt/networks/Network.java#L72-L99
pub fn network_id_to_network_definition(network_id: u8) -> NetworkDefinition {
    match network_id {
        0x01 => NetworkDefinition::mainnet(),
        i @ 0x02 => NetworkDefinition {
            id: i,
            logical_name: "Stokenet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },

        i @ 0x0A => NetworkDefinition {
            id: i,
            logical_name: "Adapanet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },
        i @ 0x0B => NetworkDefinition {
            id: i,
            logical_name: "Nebunet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },

        i @ 0x20 => NetworkDefinition {
            id: i,
            logical_name: "Gilganet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },
        i @ 0x21 => NetworkDefinition {
            id: i,
            logical_name: "Enkinet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },
        i @ 0x22 => NetworkDefinition {
            id: i,
            logical_name: "Hammunet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },

        i @ 0xF0 => NetworkDefinition {
            id: i,
            logical_name: "Localnet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },
        i @ 0xF1 => NetworkDefinition {
            id: i,
            logical_name: "IntTestNet".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },
        0xF2 => NetworkDefinition::local_simulator(),

        // TODO: Evaluate if this is needed or not. The implementation in the
        // Babylon node repo does not have something of this sort. So, perhaps
        // we do not need arbitrary conversions like this?
        i => NetworkDefinition {
            id: i,
            logical_name: "Unnamed Numeric Test Network".into(),
            hrp_suffix: format!("tdx_{:x}_", i),
        },
    }
    .into()
}
