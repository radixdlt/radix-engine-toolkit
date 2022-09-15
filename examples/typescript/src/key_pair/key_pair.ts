export interface KeyPair {
  publicKey(): Uint8Array;
  privateKey(): Uint8Array;

  publicKeyHex(): string;
  privateKeyHex(): string;

  sign(message: Uint8Array): Uint8Array;
  signHash(hash: Uint8Array): Uint8Array;
}
