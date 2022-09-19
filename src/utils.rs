use scrypto::prelude::NetworkDefinition;

use bech32;
use serde::Deserialize;

use crate::error::Error;

pub unsafe fn read_and_deserialize<'t, T>(
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
pub fn network_definition_from_network_id(network_id: u8) -> NetworkDefinition {
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
        0xF2 => NetworkDefinition::simulator(),

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

pub fn network_id_from_hrp(hrp: &str) -> Result<u8, scrypto::address::AddressError> {
    // Getting the network specifier from the given HRP. Bech32 HRPs used in Babylon are structured
    // as follows:
    let splitted_hrp: Vec<&str> = hrp.split("_").collect();
    let network_specifier: String = {
        match splitted_hrp.get(1) {
            Some(_) => Ok(splitted_hrp
                .into_iter()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("_")),
            None => Err(scrypto::address::AddressError::InvalidHrp),
        }
    }?;

    // Matching the network specifier to obtain the network id from it
    let network_id: u8 = match network_specifier.as_str() {
        "rdx" => NetworkDefinition::mainnet().id,
        "sim" => NetworkDefinition::simulator().id,
        numeric_network_specifier => {
            match numeric_network_specifier.split('_').skip(1).next() {
                Some(network_id_string) => Ok(u8::from_str_radix(network_id_string, 16)
                    .map_err(|_| scrypto::address::AddressError::InvalidHrp)?),
                None => Err(scrypto::address::AddressError::InvalidHrp),
            }
        }?,
    };
    Ok(network_id)
}

pub fn network_id_from_address_string(address: &str) -> Result<u8, scrypto::address::AddressError> {
    // Attempt to Bech32m decode this address to get the hrp and the data type (will not be used).
    // The decoding process also yields a variant. We will not be verifying that this is bech32m
    // since this method is not meant to be a validation method.
    let (hrp, _, _): (String, _, _) = bech32::decode(address)
        .map_err(|error| scrypto::address::AddressError::DecodingError(error))?;
    network_id_from_hrp(&hrp)
}

#[cfg(test)]
mod tests {
    use scrypto::prelude::NetworkDefinition;

    use crate::utils::network_id_from_address_string;

    use super::network_id_from_hrp;

    #[test]
    fn mainnet_hrp_to_network_id_succeeds() {
        // Arrange
        let hrp: &str = "resource_rdx";
        let expected_network_id: u8 = NetworkDefinition::mainnet().id;

        // Act
        let network_id: Result<u8, _> = network_id_from_hrp(hrp);

        // Assert
        assert_eq!(Ok(expected_network_id), network_id);
    }

    #[test]
    fn simulator_hrp_to_network_id_succeeds() {
        // Arrange
        let hrp: &str = "resource_sim";
        let expected_network_id: u8 = NetworkDefinition::simulator().id;

        // Act
        let network_id: Result<u8, _> = network_id_from_hrp(hrp);

        // Assert
        assert_eq!(Ok(expected_network_id), network_id);
    }

    #[test]
    fn numeric_test_network_hrp_to_network_id_succeeds() {
        // Arrange
        let hrp: &str = "resource_tdx_a0_";
        let expected_network_id: u8 = 0xA0;

        // Act
        let network_id: Result<u8, _> = network_id_from_hrp(hrp);

        // Assert
        assert_eq!(Ok(expected_network_id), network_id);
    }

    #[test]
    fn mainnet_address_to_network_id_succeeds() {
        // Arrange
        let address: &str = "resource_rdx1qd86hmk89j4q8nayxe28krxv7jfd3zu5p663nrzzqsgwqv9z";
        let expected_network_id: u8 = NetworkDefinition::mainnet().id;

        // Act
        let network_id: Result<u8, _> = network_id_from_address_string(address);

        // Assert
        assert_eq!(Ok(expected_network_id), network_id);
    }

    #[test]
    fn simulator_address_to_network_id_succeeds() {
        // Arrange
        let address: &str = "component_sim1qd86hmk89j4q8nayxe28krxv7jfd3zu5p663nrzzqsyml02z";
        let expected_network_id: u8 = NetworkDefinition::simulator().id;

        // Act
        let network_id: Result<u8, _> = network_id_from_address_string(address);

        // Assert
        assert_eq!(Ok(expected_network_id), network_id);
    }

    #[test]
    fn numeric_test_network_address_to_network_id_succeeds() {
        // Arrange
        let address: &str = "validator_tdx_a0_1qd86hmk89j4q8nayxe28krxv7jfd3zu5p663nrzzqsw5xdp6";
        let expected_network_id: u8 = 0xA0;

        // Act
        let network_id: Result<u8, _> = network_id_from_address_string(address);

        // Assert
        assert_eq!(Ok(expected_network_id), network_id);
    }
}
