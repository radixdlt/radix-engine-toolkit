use crate::prelude::*;

#[derive(Clone, Debug, Object)]
pub struct SubintentV2 {
    pub intent_core: Arc<IntentCoreV2>,
}

#[uniffi::export]
impl SubintentV2 {
    #[uniffi::constructor]
    pub fn new(intent_core: Arc<IntentCoreV2>) -> Arc<Self> {
        Arc::new(Self { intent_core })
    }

    #[uniffi::constructor]
    pub fn from_payload_bytes(compiled_intent: Vec<u8>) -> Result<Arc<Self>> {
        core_transaction_v2_subintent_from_payload_bytes(compiled_intent)
            .map_err(RadixEngineToolkitError::from)
            .and_then(|intent| intent.try_into().map(Arc::new))
    }

    pub fn subintent_hash(&self) -> Result<Arc<TransactionHash>> {
        let hash = NativeSubintentV2::try_from(self.clone())?
            .prepare(&NativePreparationSettings::latest())?
            .subintent_hash();
        Ok(Arc::new(TransactionHash::new(
            &hash,
            self.intent_core.header.network_id,
        )))
    }
}

impl TryFrom<NativeSubintentV2> for SubintentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(
        NativeSubintentV2 { intent_core }: NativeSubintentV2,
    ) -> Result<Self> {
        Ok(Self {
            intent_core: Arc::new(intent_core.try_into()?),
        })
    }
}

impl TryFrom<SubintentV2> for NativeSubintentV2 {
    type Error = RadixEngineToolkitError;

    fn try_from(SubintentV2 { intent_core }: SubintentV2) -> Result<Self> {
        Ok(Self {
            intent_core: (*intent_core).clone().try_into()?,
        })
    }
}
