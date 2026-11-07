# Stellar Subscriptions API

REST API for querying indexed subscription and plan data from Stellar.

## Quick Start

```bash
cargo build --release
cargo run
```

API server will start on `http://localhost:3001`

## Endpoints

- `GET /plans` — All active plans
- `GET /plans/:id` — Plan details
- `GET /subscriptions` — All subscriptions
- `GET /subscriptions/:id` — Subscription details
- `GET /merchants/:address/plans` — Merchant's plans
- `GET /merchants/:address/subscriptions` — Merchant's subscribers
- `GET /stats` — Platform statistics

## Configuration

The API connects to Stellar Horizon testnet by default:
- Horizon URL: `https://horizon-testnet.stellar.org`
- Poll interval: 10 seconds

## Architecture

The indexer polls Horizon for new transactions, parses contract events (PlanCreated, Subscribed, Charged, etc.), and maintains an in-memory index of all plans and subscriptions.

**No database** — v1 uses in-memory storage for simplicity.

## Development

```bash
# Run with auto-reload
cargo watch -x run

# Run tests
cargo test

# Linting
cargo clippy
```

## Logging

Set log level with `RUST_LOG`:

```bash
RUST_LOG=debug cargo run
```

## Production Deployment

For production:
1. Add a database (PostgreSQL recommended)
2. Implement rate limiting
3. Add authentication for write endpoints (future)
4. Set up monitoring and alerting
5. Use a reverse proxy (nginx/caddy)

---

**Maintainer-only:** Contributors should not modify this folder. See [CONTRIBUTING.md](../CONTRIBUTING.md).
