# Cloud IP Range API

## Overview

The Cloud IP Range API is a Rust-based application designed to provide endpoints for retrieving IP ranges of various cloud service providers such as AWS. This API can be integrated into applications or used independently to fetch cloud IP ranges dynamically.

## Features

- Fetch IP ranges for cloud service providers.
- Flexible endpoint design for easy integration.
- Lightweight and scalable Rust implementation.

## Installation and Setup

### Prerequisites

- Rust (v1.5 or later)
- Cargo (Rust's package manager)

### Build

To build the project, navigate to the project directory and run:

```
cargo build --release
```

### Run

After building the project, you can run the API server using:

```
cargo run --release
```

This command starts the API server, and it will be accessible at `http://localhost:8000` by default.

### Usage

Once the API server is running, you can make HTTP requests to the available endpoints to retrieve cloud IP ranges. For example:

```
curl "http://localhost:8000/aws?region=us-east-2&service=s3"
```

This endpoint retrieves the IP ranges for AWS in the `us-east-2` region and the `s3` service.

Example response:
```json
["52.219.212.0/22","52.219.143.0/24","52.219.141.0/24","18.34.72.0/21","3.5.128.0/22","52.219.142.0/24","52.219.96.0/20","3.5.132.0/23","52.219.232.0/22","18.34.252.0/22","16.12.64.0/22","52.219.176.0/22","16.12.60.0/22","52.219.224.0/22","52.219.80.0/20","52.219.228.0/22","3.141.102.208/28","3.141.102.224/28"]