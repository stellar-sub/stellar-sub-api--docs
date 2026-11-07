# Stellar Subscriptions API + Documentation

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Next.js](https://img.shields.io/badge/next.js-14-black)](https://nextjs.org/)

**Recurring payments on Stellar with bounded, revocable authorizations.**

This repository contains:
1. **REST API** — Rust/Axum service that indexes subscription activity from Stellar
2. **Documentation Site** — Next.js/MDX comprehensive platform documentation

## 🔗 Related Repositories

- **Contracts:** [stellar-subscriptions-contracts](https://github.com/<your-org>/stellar-subscriptions-contracts) — Soroban smart contracts
- **Web App:** [stellar-subscriptions-web](https://github.com/<your-org>/stellar-subscriptions-web) — Frontend dApp for users

## 🚀 Quick Start

### API (Rust)

```bash
cd api
cargo build --release
cargo run
```

API runs on `http://localhost:3001`

### Documentation Site (Next.js)

```bash
cd docs
npm install
npm run dev
```

Docs site runs on `http://localhost:3000`

## 📖 Documentation

Visit [stellar-subscriptions.io/docs](https://stellar-subscriptions.io/docs) or run locally with `npm run dev` in the `docs/` folder.

Key pages:
- [Authorization Model](./docs/app/docs/concepts/authorization-model/page.mdx) — How bounded authorizations work
- [Security Model](./docs/app/docs/concepts/security-model/page.mdx) — What merchants can and cannot do
- [API Reference](./docs/app/docs/api/overview/page.mdx) — REST API endpoints
- [Integration Guide](./docs/app/docs/integration/page.mdx) — Building on the platform

## 🏗️ Architecture

### API Service

The API polls Horizon every 10 seconds for new transactions, parses contract events, and maintains an in-memory index of:
- Plans (name, merchant, amount, interval, status)
- Subscriptions (cap, charged amount, status, next charge ledger)
- Platform statistics (total plans, subscriptions, charged volume)

**Tech Stack:**
- Rust 1.75+
- Axum web framework
- Tokio async runtime
- Reqwest for Horizon API calls
- Serde for JSON serialization

**Endpoints:**
- `GET /plans` — All active plans
- `GET /plans/:id` — Plan details
- `GET /subscriptions` — All subscriptions
- `GET /subscriptions/:id` — Subscription details with next charge info
- `GET /merchants/:address/plans` — Plans by merchant
- `GET /merchants/:address/subscriptions` — Subscribers to a merchant
- `GET /stats` — Platform statistics

### Documentation Site

Next.js 14 app with:
- MDX for content-rich documentation
- Tailwind CSS for styling
- Custom components (CalloutBox, CodeBlock, ApiEndpoint)
- Sidebar navigation
- Dark mode support

## 🛠️ Development

### API Development

```bash
cd api
cargo watch -x run  # auto-reload on changes
cargo test          # run tests
cargo clippy        # linting
```

### Docs Development

```bash
cd docs
npm run dev         # dev server with hot reload
npm run build       # production build
npm run lint        # ESLint
```

## 🧪 Testing

### API Tests

```bash
cd api
cargo test --all
```

### Docs Build Verification

```bash
cd docs
npm run build  # ensures all MDX compiles
```

## 📊 Project Status

- ✅ API: Fully implemented, all endpoints functional
- ✅ Docs: Complete documentation for all concepts, contracts, and API
- ⚠️ Testnet only (mainnet contracts not yet deployed)
- ⚠️ Unaudited (professional audit planned before mainnet)

## 🤝 Contributing

**Important:** Contributors should focus on documentation improvements only. The `api/` folder is maintainer-managed, and PRs touching API code will be closed.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on:
- Improving documentation clarity
- Adding code examples
- Fixing typos and formatting
- Translating docs (future)

## 📄 License

MIT License - see [LICENSE](./LICENSE) for details

## 🔒 Security

For security concerns, please email security@stellar-subscriptions.io. Do not open public issues for vulnerabilities.

## 📞 Support

- **Discord:** [stellar-subscriptions.io/discord](https://stellar-subscriptions.io/discord)
- **Issues:** [GitHub Issues](https://github.com/<your-org>/stellar-subscriptions-api-docs/issues)
- **Docs:** [stellar-subscriptions.io/docs](https://stellar-subscriptions.io/docs)

## 🙏 Acknowledgments

Built with:
- [Stellar](https://stellar.org) blockchain
- [Soroban](https://soroban.stellar.org) smart contract platform
- [Axum](https://github.com/tokio-rs/axum) web framework
- [Next.js](https://nextjs.org) React framework

---

Made with ❤️ for the Stellar ecosystem
