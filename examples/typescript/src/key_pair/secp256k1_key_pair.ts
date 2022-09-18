import { KeyPair } from "./key_pair";
import * as CryptoJS from "crypto-js";
import * as secp256k1 from "secp256k1";
import { byteArrayFromHex, hexStringFromByteArray } from "../utils";
import { Buffer } from "buffer";

export default class EcdsaSecp256k1KeyPair implements KeyPair {
  private internal_private_key: Uint8Array;

  constructor(private_key: Uint8Array) {
    if (private_key.length !== 32) {
      throw new Error("A Secp256k1 private key should have a length of 32");
    }
    this.internal_private_key = private_key;
  }

  static newRandom(): EcdsaSecp256k1KeyPair {
    let randomPrivateKey: Uint8Array = new Uint8Array(32);
    self.crypto.getRandomValues(randomPrivateKey);
    return new EcdsaSecp256k1KeyPair(randomPrivateKey);
  }

  static newFromString(private_key: string): EcdsaSecp256k1KeyPair {
    return new EcdsaSecp256k1KeyPair(Uint8Array.from(Buffer.from(private_key, "hex")));
  }

  privateKey(): Uint8Array {
    return this.internal_private_key;
  }

  privateKeyHex(): string {
    return hexStringFromByteArray(this.privateKey());
  }

  publicKey(): Uint8Array {
    return secp256k1.publicKeyCreate(this.privateKey());
  }

  publicKeyHex(): string {
    return hexStringFromByteArray(this.publicKey());
  }

  sign(message: Uint8Array): Uint8Array {
    // Double hash the message and then sign it
    let doubleHashWordArray: CryptoJS.lib.WordArray = CryptoJS.SHA256(
      CryptoJS.SHA256(CryptoJS.enc.Hex.parse(hexStringFromByteArray(message)))
    );
    let doubleHash: Uint8Array = byteArrayFromHex(doubleHashWordArray.toString());
    return this.signHash(doubleHash);
  }

  signHash(hash: Uint8Array): Uint8Array {
    // If the message is not 32 bytes long, then it is not a hash, and we can not accept it
    if (hash.length !== 32) {
      throw new Error("Only 32 byte long arrays can be signed. Consider using the `sign` func");
    }
    let { signature, recid }: { signature: Uint8Array; recid: number } = secp256k1.ecdsaSign(
      hash,
      this.privateKey()
    );
    let recovery_id: Uint8Array = Uint8Array.from([recid]);

    return Uint8Array.from(Buffer.concat([recovery_id, signature]));
  }
}
