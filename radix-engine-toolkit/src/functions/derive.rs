use bech32::{FromBase32, ToBase32};
use scrypto::prelude::*;

pub fn virtual_account_address_from_public_key<P>(public_key: &P) -> ComponentAddress
where
    P: Into<PublicKey> + Clone,
{
    ComponentAddress::virtual_account_from_public_key(public_key)
}

pub fn virtual_identity_address_from_public_key<P>(public_key: &P) -> ComponentAddress
where
    P: Into<PublicKey> + Clone,
{
    ComponentAddress::virtual_identity_from_public_key(public_key)
}

pub fn virtual_signature_non_fungible_global_id_from_public_key<P>(
    public_key: &P,
) -> NonFungibleGlobalId
where
    P: HasPublicKeyHash,
{
    NonFungibleGlobalId::from_public_key(public_key)
}

pub fn virtual_account_address_from_olympia_account_address<S>(
    olympia_account_address: &S,
) -> Result<ComponentAddress, DerivationError>
where
    S: AsRef<str>,
{
    public_key_from_olympia_account_address(olympia_account_address)
        .map(|public_key| virtual_account_address_from_public_key(&public_key))
}

pub fn public_key_from_olympia_account_address<S>(
    olympia_account_address: &S,
) -> Result<EcdsaSecp256k1PublicKey, DerivationError>
where
    S: AsRef<str>,
{
    let olympia_account_address = olympia_account_address.as_ref();

    // Ensure that the second and third characters in the string are d and x which are present in
    // all account HRPs in Olympia regardless of the network.
    match (
        olympia_account_address.chars().nth(1),
        olympia_account_address.chars().nth(2),
    ) {
        (Some('d'), Some('x')) => Ok(()),
        (Some(char1), Some(char2)) => Err(
            DerivationError::InvalidCharsInOlympiaAddressEntitySpecifier {
                expected: ('d', 'x'),
                actual: (char1, char2),
            },
        ),
        _ => Err(DerivationError::InvalidOlympiaAddressLength {
            expected: 65,
            actual: olympia_account_address.len(),
        }),
    }?;

    let (_, data, variant) =
        bech32::decode(olympia_account_address).map_err(DerivationError::Bech32DecodeError)?;
    if let bech32::Variant::Bech32 = variant {
        Ok(())
    } else {
        Err(DerivationError::InvalidOlympiaBech32Variant {
            expected: bech32::Variant::Bech32,
            actual: bech32::Variant::Bech32m,
        })
    }?;

    let mut data =
        Vec::<u8>::from_base32(&data).map_err(DerivationError::Bech32BaseConversionError)?;

    // Check the length of the data to ensure that it's a public key. Length should be 1 + 33
    // where the added 1 byte is because of the 0x04 prefix that public keys have.
    if data.len() != 34 {
        Err(DerivationError::InvalidOlympiaAddressLength {
            expected: 34,
            actual: data.len(),
        })
    } else if *data.first().unwrap() != 0x04 {
        Err(DerivationError::InvalidOlympiaAddressPrefix {
            expected: 0x04,
            actual: *data.first().unwrap(),
        })
    } else {
        data.remove(0);
        Ok(())
    }?;

    let public_key =
        EcdsaSecp256k1PublicKey(data.try_into().expect("Impossible case. Length is known."));

    Ok(public_key)
}

pub fn olympia_account_address_from_public_key(
    public_key: EcdsaSecp256k1PublicKey,
    olympia_network: OlympiaNetwork,
) -> String {
    let public_key = {
        let mut vector = vec![0x04];
        vector.extend(public_key.0);
        vector
    };
    bech32::encode(
        olympia_network.hrp(),
        public_key.to_base32(),
        bech32::Variant::Bech32,
    )
    .unwrap()
}

pub enum OlympiaNetwork {
    Mainnet,
    Stokenet,
    Releasenet,
    RCNet,
    Milestonenet,
    Devopsnet,
    Sandpitnet,
    Localnet,
}

impl OlympiaNetwork {
    pub const fn hrp(&self) -> &str {
        match self {
            Self::Mainnet => "rdx",
            Self::Stokenet => "tdx",
            Self::Releasenet => "tdx3",
            Self::RCNet => "tdx4",
            Self::Milestonenet => "tdx5",
            Self::Devopsnet => "tdx6",
            Self::Sandpitnet => "tdx7",
            Self::Localnet => "ddx",
        }
    }
}

#[derive(Debug)]
pub enum DerivationError {
    InvalidCharsInOlympiaAddressEntitySpecifier {
        expected: (char, char),
        actual: (char, char),
    },
    InvalidOlympiaAddressLength {
        expected: usize,
        actual: usize,
    },
    InvalidOlympiaBech32Variant {
        expected: bech32::Variant,
        actual: bech32::Variant,
    },
    InvalidOlympiaAddressPrefix {
        expected: u8,
        actual: u8,
    },
    Bech32DecodeError(bech32::Error),
    Bech32BaseConversionError(bech32::Error),
}
