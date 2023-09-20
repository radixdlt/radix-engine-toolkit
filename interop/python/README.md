<div align="center">
  <h1><code>Radix Engine Toolkit</code></h1>
  <p>
    <strong>A Python wrapper around the Radix Engine Toolkit that provides Radix Ledger primitives to Python</strong>
  </p>

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
</div>

The (Python) Radix Engine Toolkit is a wrapper around the [Radix Engine Toolkit](https://github.com/radixdlt/radix-engine-toolkit/) library which exposes the Radix Engine and Scrypto primitives to Python. These primitives include: manifests, transactions, transaction construction and building, access rules, metadata, the SBOR codec, derivations, and many others. The purpose this and the other wrappers around the toolkit is to provide developers with the freedom of constructing transactions and interacting with the ledger in their language of their choice instead of being limited to using Rust <!-- Even though I really think you should learn and use Rust!. -->.

This library uses [UniFFI](https://github.com/mozilla/uniffi-rs) for interoperability between the core Rust Radix Engine Toolkit and Radix Engine Toolkit wrappers such as this Python wrapper. Thus, the entire codebase of this library is automatically generated and the Python code does not live in a repo by itself. Instead, this library is published directly to PyPi with each push that's made to the [Radix Engine Toolkit](https://github.com/radixdlt/radix-engine-toolkit/) repo. If you would like to submit an issue or open a PR then head to: https://github.com/radixdlt/radix-engine-toolkit/