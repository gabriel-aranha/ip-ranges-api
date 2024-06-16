# Cloud IP Ranges API

## Overview

The Cloud IP Ranges API is a Rust-based application designed to provide endpoints for retrieving IP ranges of various cloud service providers such as AWS. This API can be integrated into applications or used independently to fetch cloud IP ranges dynamically.

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

Once the API server is running, you can make HTTP requests to the available endpoints to retrieve cloud IP ranges.

### AWS Usage

#### Endpoint

```
GET /v1/aws?region=<region>&service=<service>&network_border_group=<network_border_group>
```

#### Parameters

- `region` (optional): Filter by AWS region.
- `service` (optional): Filter by AWS service.
- `network_border_group` (optional): Filter by AWS network border group.

#### Example Request

```
curl "http://localhost:8000/v1/aws?region=us-east-2&service=s3"
```

### Azure Usage

#### Endpoint

```
GET /v1/azure?region=<region>&system_service=<system_service>&ipv4=<true|false>&ipv6=<true|false>
```

#### Parameters

- `region` (optional): Filter by Azure region.
- `system_service` (optional): Filter by Azure system service.
- `ipv4` (required): Include IPv4 prefixes if true (default: false).
- `ipv6` (required): Include IPv6 prefixes if true (default: false).

#### Example Request

```
curl "http://localhost:8000/v1/azure?region=westus2&ipv4=true"
```

### GCP Usage

#### Endpoint

```
GET /v1/gcp?scope=<scope>&service=<service>&ipv4=<true|false>&ipv6=<true|false>
```

#### Parameters

- `scope` (optional): Filter by GCP scope.
- `service` (optional): Filter by GCP service.
- `ipv4` (required): Include IPv4 prefixes if true (default: false).
- `ipv6` (required): Include IPv6 prefixes if true (default: false).

#### Example Request

```
curl "http://localhost:8000/v1/gcp?scope=africa-south1&ipv4=true"
```