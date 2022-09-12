import * as Value from './value';
import * as Instruction from './instruction';

export default class ManifestBuilder {
    instructions: Instruction.Instruction[];
    
    constructor() {
        this.instructions = [];
    }

    callFunction(
        package_address: string,
        blueprint_name: string,
        function_name: string,
        args?: Value.Value[]
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.CallFunction,
            package_address: {
                type: Value.ValueKind.PackageAddress,
                value: package_address
            },
            blueprint_name: {
                type: Value.ValueKind.String,
                value: blueprint_name
            },
            function_name: {
                type: Value.ValueKind.String,
                value: function_name
            },
            arguments: args
        })
        return this
    }
    
    callMethod(
        component_address: string,
        method_name: string,
        args?: Value.Value[]
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.CallMethod,
            component_address: {
                type: Value.ValueKind.ComponentAddress,
                value: component_address
            },
            method_name: {
                type: Value.ValueKind.String,
                value: method_name
            },
            arguments: args
        })
        return this
    }

    callMethodWithAllResources(
        component_address: string,
        method_name: string,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.CallMethodWithAllResources,
            component_address: {
                type: Value.ValueKind.ComponentAddress,
                value: component_address
            },
            method_name: {
                type: Value.ValueKind.String,
                value: method_name
            },
        })
        return this
    }
    
    takeFromWorktop(
        resource_address: string,
        bucket_identifier: string | number,
    ): this {
        // TODO: If the bucket identifier is a number, check that number is an unsigned integer in 
        // the bounds of a u32.
        this.instructions.push({
            instruction: Instruction.InstructionKind.TakeFromWorktop,
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
            into_bucket: {
                type: Value.ValueKind.Bucket,
                identifier: bucket_identifier
            },
        })
        return this
    }

    takeFromWorktopByAmount(
        amount: number,
        resource_address: string,
        bucket_identifier: string | number,
    ): this {
        // TODO: If the bucket identifier is a number, check that number is an unsigned integer in 
        // the bounds of a u32.
        this.instructions.push({
            instruction: Instruction.InstructionKind.TakeFromWorktopByAmount,
            amount: {
                type: Value.ValueKind.Decimal,
                value: amount.toLocaleString('fullwide', {useGrouping:false}),
            },
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
            into_bucket: {
                type: Value.ValueKind.Bucket,
                identifier: bucket_identifier
            },
        })
        return this
    }
    
    takeFromWorktopByIds(
        ids: Value.NonFungibleId[],
        resource_address: string,
        bucket_identifier: string | number,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.TakeFromWorktopByIds,
            ids: ids,
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
            into_bucket: {
                type: Value.ValueKind.Bucket,
                identifier: bucket_identifier
            },
        })
        return this
    }
    
    returnToWorktop(
        bucket_identifier: string | number,
    ): this {
        // TODO: If the bucket identifier is a number, check that number is an unsigned integer in 
        // the bounds of a u32.
        this.instructions.push({
            instruction: Instruction.InstructionKind.ReturnToWorktop,
            bucket: {
                type: Value.ValueKind.Bucket,
                identifier: bucket_identifier
            },
        })
        return this
    }

    assertWorktopContains(
        resource_address: string,
    ): this {
        // TODO: If the bucket identifier is a number, check that number is an unsigned integer in 
        // the bounds of a u32.
        this.instructions.push({
            instruction: Instruction.InstructionKind.AssertWorktopContains,
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
        })
        return this
    }

    assertWorktopContainsByAmount(
        amount: number,
        resource_address: string,
    ): this {
        // TODO: If the bucket identifier is a number, check that number is an unsigned integer in 
        // the bounds of a u32.
        this.instructions.push({
            instruction: Instruction.InstructionKind.AssertWorktopContainsByAmount,
            amount: {
                type: Value.ValueKind.Decimal,
                value: amount.toLocaleString('fullwide', {useGrouping:false}),
            },
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
        })
        return this
    }
    
    assertWorktopContainsByIds(
        ids: Value.NonFungibleId[],
        resource_address: string,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.AssertWorktopContainsByIds,
            ids: ids,
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
        })
        return this
    }

    popFromAuthZone(
        proof_identifier: string | number,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.PopFromAuthZone,
            into_proof: {
                type: Value.ValueKind.Proof,
                identifier: proof_identifier
            },
        })
        return this
    }
    pushToAuthZone(
        proof_identifier: string | number,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.PushToAuthZone,
            proof: {
                type: Value.ValueKind.Proof,
                identifier: proof_identifier
            },
        })
        return this
    }
    
    clearAuthZone(): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.ClearAuthZone,
        })
        return this
    }

    createProofFromAuthZone(
        resource_address: string,
        proof_identifier: string | number,
    ): this {
        // TODO: If the proof identifier is a number, check that number is an unsigned integer in 
        // the bounds of a u32.
        this.instructions.push({
            instruction: Instruction.InstructionKind.CreateProofFromAuthZone,
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
            into_proof: {
                type: Value.ValueKind.Proof,
                identifier: proof_identifier
            },
        })
        return this
    }

    createProofFromAuthZoneByAmount(
        amount: number,
        resource_address: string,
        proof_identifier: string | number,
    ): this {
        // TODO: If the proof identifier is a number, check that number is an unsigned integer in 
        // the bounds of a u32.
        this.instructions.push({
            instruction: Instruction.InstructionKind.CreateProofFromAuthZoneByAmount,
            amount: {
                type: Value.ValueKind.Decimal,
                value: amount.toLocaleString('fullwide', {useGrouping:false}),
            },
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
            into_proof: {
                type: Value.ValueKind.Proof,
                identifier: proof_identifier
            },
        })
        return this
    }
    
    createProofFromAuthZoneByIds(
        ids: Value.NonFungibleId[],
        resource_address: string,
        proof_identifier: string | number,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.CreateProofFromAuthZoneByIds,
            ids: ids,
            resource_address: {
                type: Value.ValueKind.ResourceAddress,
                value: resource_address
            },
            into_proof: {
                type: Value.ValueKind.Proof,
                identifier: proof_identifier
            },
        })
        return this
    }

    createProofFromBucket(
        bucket_identifier: string | number,
        proof_identifier: string | number,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.CreateProofFromBucket,
            bucket: {
                type: Value.ValueKind.Bucket,
                identifier: bucket_identifier
            },
            into_proof: {
                type: Value.ValueKind.Proof,
                identifier: proof_identifier
            },
        })
        return this
    }

    cloneProof(
        original_proof_identifier: string | number,
        new_proof_identifier: string | number,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.CloneProof,
            proof: {
                type: Value.ValueKind.Proof,
                identifier: original_proof_identifier,
            },
            into_proof: {
                type: Value.ValueKind.Proof,
                identifier: new_proof_identifier,
            },
        })
        return this
    }
    
    dropProof(
        proof_identifier: string | number,
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.DropProof,
            proof: {
                type: Value.ValueKind.Proof,
                identifier: proof_identifier
            }
        })
        return this
    }
    dropAllProofs(): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.DropAllProofs,
        })
        return this
    }
    
    publishPackage(
        package_bytes: string
    ): this {
        this.instructions.push({
            instruction: Instruction.InstructionKind.PublishPackage,
            package: {
                type: Value.ValueKind.Bytes,
                value: package_bytes
            }
        })
        return this
    }
}