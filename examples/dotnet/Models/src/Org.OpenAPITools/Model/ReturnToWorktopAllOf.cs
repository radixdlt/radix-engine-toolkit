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
    /// ReturnToWorktopAllOf
    /// </summary>
    [DataContract]
    public partial class ReturnToWorktopAllOf :  IEquatable<ReturnToWorktopAllOf>, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ReturnToWorktopAllOf" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected ReturnToWorktopAllOf() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="ReturnToWorktopAllOf" /> class.
        /// </summary>
        /// <param name="bucket">bucket (required).</param>
        public ReturnToWorktopAllOf(Bucket bucket = default(Bucket))
        {
            // to ensure "bucket" is required (not null)
            if (bucket == null)
            {
                throw new InvalidDataException("bucket is a required property for ReturnToWorktopAllOf and cannot be null");
            }
            else
            {
                this.Bucket = bucket;
            }

        }

        /// <summary>
        /// Gets or Sets Bucket
        /// </summary>
        [DataMember(Name="bucket", EmitDefaultValue=true)]
        public Bucket Bucket { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class ReturnToWorktopAllOf {\n");
            sb.Append("  Bucket: ").Append(Bucket).Append("\n");
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
            return this.Equals(input as ReturnToWorktopAllOf);
        }

        /// <summary>
        /// Returns true if ReturnToWorktopAllOf instances are equal
        /// </summary>
        /// <param name="input">Instance of ReturnToWorktopAllOf to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(ReturnToWorktopAllOf input)
        {
            if (input == null)
                return false;

            return 
                (
                    this.Bucket == input.Bucket ||
                    (this.Bucket != null &&
                    this.Bucket.Equals(input.Bucket))
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
                if (this.Bucket != null)
                    hashCode = hashCode * 59 + this.Bucket.GetHashCode();
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