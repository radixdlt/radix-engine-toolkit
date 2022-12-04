use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Identifier {
    String(String),
    U32(u32),
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct BucketId(pub Identifier);

impl From<Identifier> for BucketId {
    fn from(identifier: Identifier) -> Self {
        Self(identifier)
    }
}

impl From<BucketId> for Identifier {
    fn from(bucket_id: BucketId) -> Self {
        bucket_id.0
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct ProofId(pub Identifier);

impl From<Identifier> for ProofId {
    fn from(identifier: Identifier) -> Self {
        Self(identifier)
    }
}

impl From<ProofId> for Identifier {
    fn from(proof_id: ProofId) -> Self {
        proof_id.0
    }
}