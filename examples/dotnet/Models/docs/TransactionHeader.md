# Org.OpenAPITools.Model.TransactionHeader

## Properties

| Name                    | Type                                                | Description                                                                                         | Notes |
| ----------------------- | --------------------------------------------------- | --------------------------------------------------------------------------------------------------- | ----- |
| **\_Version**           | **int**                                             | An 8-bit unsigned number.                                                                           |
| **NetworkId**           | **int**                                             | An 8-bit unsigned number.                                                                           |
| **StartEpochInclusive** | **int**                                             | A 64-bit unsigned integer representing the epoch the earliest that this transaction can be executed |
| **EndEpochExclusive**   | **int**                                             | A 64-bit unsigned integer representing the latest epoch this transaction should be executed         |
| **Nonce**               | **int**                                             | A 64-bit unsigned integer of the nonce to use for the transaction                                   |
| **NotaryPublicKey**     | [**EcdsaPublicKeyString**](EcdsaPublicKeyString.md) |                                                                                                     |
| **NotaryAsSignatory**   | **bool**                                            | A boolean representing whether the notary is also one of the people signing the transaction         |
| **CostUnitLimit**       | **int**                                             | A 32-bit unsigned integer of the maximum cost units that this transaction can spend                 |
| **TipPercentage**       | **int**                                             | A 32-bit unsigned integer of the percentage of the tip to give to validators                        |

[[Back to Model list]](../README.md#documentation-for-models)
[[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to README]](../README.md)
