# Differentially Private TEE - Backend

This is a Rust implementation of a super basic TCP server running inside an SGX enclave.
The SGX enclave uses the Teaclave SDK.
The server can handle numeric series of data and apply a simple Laplace mechanism to make the dataset differentially private.

## Requirements

- [Intel SGX drivers & SDK](https://github.com/intel/linux-sgx-driver)
- [Teaclave SGX SDK](https://github.com/apache/incubator-teaclave-sgx-sdk)
- Rust nightly toolchain

## How to run the server (on Linux)

```bash
git clone https://github.com/apache/incubator-teaclave-sgx-sdk
cd incubator-teaclave-sgx-sdk/samplecode/
git clone https://github.com/jbgriesner/dptee-backend
cd dptee-backend
./run.bash
```