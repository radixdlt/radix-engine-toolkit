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
    /// Map
    /// </summary>
    [DataContract]
    public partial class Map : Value,  IEquatable<Map>, IValidatableObject
    {
        /// <summary>
        /// Gets or Sets KeyType
        /// </summary>
        [DataMember(Name="key_type", EmitDefaultValue=true)]
        public ValueKind KeyType { get; set; }
        /// <summary>
        /// Gets or Sets ValueType
        /// </summary>
        [DataMember(Name="value_type", EmitDefaultValue=true)]
        public ValueKind ValueType { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="Map" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected Map() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="Map" /> class.
        /// </summary>
        /// <param name="keyType">keyType (required).</param>
        /// <param name="valueType">valueType (required).</param>
        /// <param name="elements">elements (required).</param>
        public Map(ValueKind keyType = default(ValueKind), ValueKind valueType = default(ValueKind), List<Value> elements = default(List<Value>)) : base(type)
        {
            // to ensure "keyType" is required (not null)
            if (keyType == null)
            {
                throw new InvalidDataException("keyType is a required property for Map and cannot be null");
            }
            else
            {
                this.KeyType = keyType;
            }

            // to ensure "valueType" is required (not null)
            if (valueType == null)
            {
                throw new InvalidDataException("valueType is a required property for Map and cannot be null");
            }
            else
            {
                this.ValueType = valueType;
            }

            // to ensure "elements" is required (not null)
            if (elements == null)
            {
                throw new InvalidDataException("elements is a required property for Map and cannot be null");
            }
            else
            {
                this.Elements = elements;
            }

        }



        /// <summary>
        /// Gets or Sets Elements
        /// </summary>
        [DataMember(Name="elements", EmitDefaultValue=true)]
        public List<Value> Elements { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class Map {\n");
            sb.Append("  ").Append(base.ToString().Replace("\n", "\n  ")).Append("\n");
            sb.Append("  KeyType: ").Append(KeyType).Append("\n");
            sb.Append("  ValueType: ").Append(ValueType).Append("\n");
            sb.Append("  Elements: ").Append(Elements).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public override string ToJson()
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
            return this.Equals(input as Map);
        }

        /// <summary>
        /// Returns true if Map instances are equal
        /// </summary>
        /// <param name="input">Instance of Map to be compared</param>
        /// <returns>Boolean</returns>
        public bool Equals(Map input)
        {
            if (input == null)
                return false;

            return base.Equals(input) && 
                (
                    this.KeyType == input.KeyType ||
                    (this.KeyType != null &&
                    this.KeyType.Equals(input.KeyType))
                ) && base.Equals(input) && 
                (
                    this.ValueType == input.ValueType ||
                    (this.ValueType != null &&
                    this.ValueType.Equals(input.ValueType))
                ) && base.Equals(input) && 
                (
                    this.Elements == input.Elements ||
                    this.Elements != null &&
                    input.Elements != null &&
                    this.Elements.SequenceEqual(input.Elements)
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
                int hashCode = base.GetHashCode();
                if (this.KeyType != null)
                    hashCode = hashCode * 59 + this.KeyType.GetHashCode();
                if (this.ValueType != null)
                    hashCode = hashCode * 59 + this.ValueType.GetHashCode();
                if (this.Elements != null)
                    hashCode = hashCode * 59 + this.Elements.GetHashCode();
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
            foreach(var x in base.BaseValidate(validationContext)) yield return x;
            yield break;
        }
    }

}