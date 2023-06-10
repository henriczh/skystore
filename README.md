# SkyStore: A Global Object Store

SkyStore is a global object store built on top of the object stores in the cloud.
It currently support S3, Azure Blob, and GCS.
You interact with SkyStore through S3 API.
SkyStore automatically place your data in the right cloud provider and region to optimize performance and cost.

Currently, this repo is still in a prototype phase. We are preparing for an alpha at the moment.

## Getting Started

TODO

## Development

To setup the environment:

- Ensure that you have Python and Rust toolchain installed.
- `cargo install just`. We use `just` as a task runner.

```bash
cd store-server
pip install -r requirements.txt
```

```bash
cd s3-proxy
just install-local-s3

# run the following commands in separate windows as they are blocking.
just run-skystore-server
just run-local-s3
just run
```

The S3 proxy should now be serving requests at `http://localhost:8002`.

You can use the AWS CLI or any S3 client to interact with the proxy. Do note that you will need to set `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` to some dummy values. Checkout `s3-proxy/justfile` for reference.

Run some sample commands:

```bash
cd s3-proxy
just run-cli-put
just run-cli-get
just run-cli-list
just run-cli-multipart
```
