# Interactive Brokers TWS API for Rust
[![LGPL 3.0](https://img.shields.io/crates/l/ib_tws_core?style=for-the-badge)](https://choosealicense.com/licenses/lgpl-3.0/)

## Usage
```rust
let client = {
	let port = std::env::args()
		.nth(1)
		.and_then(|p| p.parse::<u32>().ok())
		.unwrap_or(4001);
	let transport = ib_tws_tokio::Transport::connect(
		format!("127.0.0.1:{port}").parse().unwrap(),
		Duration::from_secs(5),
	)
	.await
	.into_diagnostic()?;
	ib_tws_core::AsyncClient::setup(transport, 0).await?
};
info!(version = client.server_version(), "connected to client");

```

## Crates
### `ib_tws_core`
[![crates.io](https://img.shields.io/crates/v/ib_tws_core?style=for-the-badge)](https://crates.io/crates/ib_tws_core) [![docs.rs](https://img.shields.io/badge/docs.rs-ib_tws_core-rs?style=for-the-badge)](https://docs.rs/ib_tws_core)

Contains core components including messages and encoding/decoding utilities.

### `ib_tws_tokio`
[![crates.io](https://img.shields.io/crates/v/ib_tws_tokio?style=for-the-badge)](https://crates.io/crates/ib_tws_core) [![docs.rs](https://img.shields.io/badge/docs.rs-ib_tws_tokio-rs?style=for-the-badge)](https://docs.rs/ib_tws_tokio)

A client implementation using Tokio.

## Credits
`ib_tws_core` is forked from [chrisdamba's ib_async](https://github.com/chrisdamba/ib_async).
