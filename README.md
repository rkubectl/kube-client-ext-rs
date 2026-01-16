# kube-client-ext

Extension traits (helpers) for [`kube::Client`](https://docs.rs/kube/latest/kube/struct.Client.html) to make working with Kubernetes resources more ergonomic and convenient.

[![Crates.io](https://img.shields.io/crates/v/kube-client-ext.svg)](https://crates.io/crates/kube-client-ext)
[![Documentation](https://docs.rs/kube-client-ext/badge.svg)](https://docs.rs/kube-client-ext)
[![Apache 2.0 Licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![Rust](https://github.com/rkubectl/kube-client-ext-rs/workflows/CI/badge.svg)](https://github.com/rkubectl/kube-client-ext-rs/actions)

## Overview

This crate provides extension traits for `kube::Client` that add convenience methods for:

- **Resource API access**: Direct access to common Kubernetes APIs (pods, deployments, services, etc.)
- **Parameter builders**: Pre-configured parameter objects for common operations
- **Async helpers**: High-level async methods for fetching and manipulating resources
- **Resource relationships**: Methods to find related resources (pods by deployment, etc.)
- **Error handling**: Utilities for handling common error scenarios like "not found"

## Features

### `KubeClientExt` - Synchronous Extensions

The `KubeClientExt` trait adds methods for:

**Parameter Builders:**
```rust
use kube_client_ext::KubeClientExt;

let client = Client::try_default().await?;

// Pre-configured parameter objects
let delete_params = client.delete_params(); // with grace_period(0)
let post_params = client.post_params_with_manager("my-controller");
let patch_params = client.patch_params_with_manager("my-controller");
```

**Direct API Access:**
```rust
// Cluster-scoped resources
let nodes_api = client.nodes();
let crds_api = client.crds();
let cluster_roles_api = client.clusterroles();

// Namespaced resources (with optional namespace)
let pods_api = client.pods("my-namespace");
let pods_api_default = client.pods(None); // uses default namespace
let deployments_api = client.deployments("kube-system");
let services_api = client.services("default");
```

### `KubeClientExt2` - Async Extensions

The `KubeClientExt2` trait provides high-level async methods:

**Resource Fetching:**
```rust
use kube_client_ext::KubeClientExt2;

// Get resources with optional return (None if not found)
let pod = client.get_pod_opt("my-pod", "default").await?;
let deployment = client.get_deployment_opt("my-app", None).await?;

// Get resources (returns error if not found)
let secret = client.get_secret("my-secret", "default").await?;
let configmap = client.get_configmap("my-config", None).await?;
```

**Resource Listing:**
```rust
// List resources in namespace
let pods = client.list_pods("default").await?;
let deployments = client.list_deployments(None).await?;
let jobs = client.list_jobs("batch-namespace").await?;
let secrets = client.list_secrets("default").await?;
let services = client.list_services("kube-system").await?;
let statefulsets = client.list_statefulsets("database-ns").await?;
let configmaps = client.list_configmaps("default").await?;
let serviceaccounts = client.list_serviceaccounts("default").await?;
```

**Resource Relationships:**
```rust
// Get pods controlled by a deployment
let pods = client.get_pods_by_deployment_name("my-app", "default").await?;

// Get owner resource from owner reference
let owner = client.get_owner_k::<corev1::Pod, appsv1::Deployment>(&pod).await?;
```

### Error Handling Utilities

```rust
use kube_client_ext::not_found_ok;

// Convert "not found" errors into Ok(Right(status))
client.namespaces()
    .delete("nonexistent", &client.delete_params())
    .await
    .or_else(not_found_ok)?;
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
kube-client-ext = "3.0"
kube = { version = "0.96", features = ["client"] }
k8s-openapi = { version = "0.27", features = ["latest"] }
k8s-openapi-ext = "0.27"
```

### Basic Example

```rust
use kube::Client;
use kube_client_ext::{KubeClientExt, KubeClientExt2};
use k8s_openapi_ext::{corev1, appsv1};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    // List all pods in the default namespace
    let pods = client.list_pods(None).await?;
    println!("Found {} pods", pods.len());

    // List secrets and services in a specific namespace
    let secrets = client.list_secrets("default").await?;
    let services = client.list_services("default").await?;
    println!("Found {} secrets and {} services", secrets.len(), services.len());

    // Get a specific deployment
    if let Some(deployment) = client.get_deployment_opt("my-app", "default").await? {
        println!("Deployment found: {}", deployment.name_any());

        // Get all pods controlled by this deployment
        if let Some(pods) = client.get_pods_by_deployment(&deployment).await? {
            println!("Deployment controls {} pods", pods.len());
        }
    }

    // Create a service using builder patterns
    let service_api = client.services("default");
    let post_params = client.post_params_with_manager("my-controller");

    // service creation logic here...

    Ok(())
}
```

### Advanced Usage

```rust
use kube_client_ext::{KubeClientExt, KubeClientExt2, not_found_ok};
use k8s_openapi_ext::{corev1, appsv1};

async fn cleanup_deployment(
    client: &Client,
    name: &str,
    namespace: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Delete deployment and ignore if not found
    client.deployments(namespace)
        .delete(name, &client.foreground_delete())
        .await
        .or_else(not_found_ok)?;

    // Clean up any secrets owned by this deployment
    let secrets = client.list_secrets(namespace).await?;
    for secret in secrets {
        if secret.owner_references()
            .iter()
            .any(|owner| owner.name == name && owner.kind == "Deployment")
        {
            client.secrets(namespace)
                .delete(&secret.name_any(), &client.delete_params())
                .await
                .or_else(not_found_ok)?;
        }
    }

    // Also clean up any associated services
    let services = client.list_services(namespace).await?;
    for service in services {
        if service.owner_references()
            .iter()
            .any(|owner| owner.name == name && owner.kind == "Deployment")
        {
            client.services(namespace)
                .delete(&service.name_any(), &client.delete_params())
                .await
                .or_else(not_found_ok)?;
        }
    }

    Ok(())
}
```

## Supported Kubernetes Versions

This crate supports the same Kubernetes versions as the underlying `kube` and `k8s-openapi` crates:

- Kubernetes 1.24+
- Latest tested: Kubernetes 1.31

## Features

- **`default`**: Enables `k8s-openapi/latest` feature for the most recent Kubernetes APIs

## Why Use This Crate?

### Before (using kube directly):
```rust
use kube::{Api, Client};
use kube::api::{DeleteParams, PostParams, ListParams};
use k8s_openapi_ext::corev1;

let client = Client::try_default().await?;
let pods: Api<corev1::Pod> = Api::namespaced(client, "default");
let lp = ListParams::default();
let pod_list = pods.list(&lp).await?.items;

let mut delete_params = DeleteParams::default();
delete_params.grace_period = Some(0);
```

### After (using kube-client-ext):
```rust
use kube_client_ext::{KubeClientExt, KubeClientExt2};
use k8s_openapi_ext::corev1;

let client = Client::try_default().await?;
let pods = client.list_pods("default").await?;

let delete_params = client.delete_params(); // grace_period(0) included
```

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

This project follows the same license as the Kubernetes project and the `kube` ecosystem.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development

This project uses [`just`](https://github.com/casey/just) for task automation:

```bash
# Run tests
just test

# Run clippy
just clippy

# Run all checks (format, update, test, clippy)
just check

# Build project
just build

# Clean and rebuild
just cbuild
```

## Related Projects

- [`kube`](https://github.com/kube-rs/kube) - The main Kubernetes client library for Rust
- [`k8s-openapi`](https://github.com/Arnavion/k8s-openapi) - Kubernetes API bindings for Rust
- [`k8s-openapi-ext`](https://crates.io/crates/k8s-openapi-ext) - Extension traits for Kubernetes resource types
