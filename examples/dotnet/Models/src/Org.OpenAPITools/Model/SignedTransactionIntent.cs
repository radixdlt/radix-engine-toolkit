/*
 * Transaction Library
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */

using System;
using System.Linq;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.ComponentModel.DataAnnotations;
using OpenAPIDateConverter = Org.OpenAPITools.Client.OpenAPIDateConverter;

namespace Org.OpenAPITools.Model
{
    /// <summary>
    /// A transaction intent which has been signed
    /// </summary>
    [DataContract]
    public partial class SignedTransactionIntent :  IEquatable<SignedTransactionIntent>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="SignedTransactionIntent" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected SignedTransactionIntent() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="SignedTransactionIntent" /> class.
        /// </summary>
        /// <param name="transactionIntent">transactionIntent (required).</param>
        /// <param name="signatures">An array containing the signature of the signers (required).</param>
        public SignedTransactionIntent(TransactionIntent transactionIntent = default(TransactionIntent), List<Signature> signatures = default(List<Signature>))
        {
            // to ensure "transactionIntent" is required (not null)
            if (transactionIntent == null)
            {
                throw new InvalidDataException("transactionIntent is a required property for SignedTransactionIntent and cannot be null");
            }
            else
            {
                this.TransactionIntent = transactionIntent;
            }

            // to ensure "signatures" is required (not null)
            if (signatures == null)
            {
                throw new InvalidDataException("signatures is a required property for SignedTransactionIntent and cannot be null");
            }
            else
            {
                this.Signatures = signatures;
            }

        }

        /// <summary>
        /// Gets or Sets TransactionIntent
        /// </summary>
        [DataMember(Name="transaction_intent", EmitDefaultValue=true)]
        public TransactionIntent TransactionIntent { get; set; }

        /// <summary>
        /// An array containing the signature of the signers
        /// </summary>
        /// <value>An array containing the signature of the signers</value>
        [DataMember(Name="signatures", EmitDefaultValue=true)]
        public List<Signature> Signatures { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class SignedTransactionIntent {\n");
            sb.Append("  TransactionIntent: ").Append(TransactionIntent).Append("\n");
            sb.Append("  Signatures: ").Append(Signatures).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// Returns true if objects are equal
        /// </summary>
        /// <param name="input">Object to be compared</param>
        /// <returns>Boolean</returns>
        public override bool Equals(object input)
        {
            return this.Equals(input as SignedTransactionIntent);
        }

        /// <summary>
        /// Returns true if SignedTransactionIntent instances are equal
        /// </summary>
        /// <param name="input">Instance of SignedTransactionIntent to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(SignedTransactionIntent input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.TransactionIntent == input.TransactionIntent ||
                    (this.TransactionIntent != null &&
                    this.TransactionIntent.Equals(input.TransactionIntent))
                ) && 
                (
                    this.Signatures == input.Signatures ||
                    this.Signatures != null &&
                    input.Signatures != null &&
                    this.Signatures.SequenceEqual(input.Signatures)
                );
        }

        /// <summary>
        /// Gets the hash code
        /// </summary>
        /// <returns>Hash code</returns>
        public override int GetHashCode()
        {
            unchecked // Overflow is fine, just wrap
            {
                int hashCode = 41;
                if (this.TransactionIntent != null)
                    hashCode = hashCode * 59 + this.TransactionIntent.GetHashCode();
                if (this.Signatures != null)
                    hashCode = hashCode * 59 + this.Signatures.GetHashCode();
                return hashCode;
            }
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<System.ComponentModel.DataAnnotations.ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            yield break;
        }
    }

}