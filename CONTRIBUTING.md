# Contributing to Stellar Subscriptions API + Documentation

Thank you for your interest in contributing! This document outlines how you can help improve the project.

## 🚨 Important: Contribution Scope

**Contributors may ONLY work on documentation in the `docs/` folder.**

The `api/` folder (Rust backend) is **maintainer-managed**. Pull requests that modify, add, or delete files in `api/` will be closed immediately without review.

This policy ensures API consistency and security. If you find bugs or have feature requests for the API, please open an issue instead.

## ✅ What You CAN Contribute

### Documentation Improvements

- **Clarity:** Simplify explanations, fix confusing wording
- **Examples:** Add code examples, tutorials, or use cases
- **Accuracy:** Correct technical errors or outdated information
- **Formatting:** Fix typos, grammar, broken links, markdown formatting
- **Completeness:** Fill gaps, add missing details, expand on concepts
- **Accessibility:** Improve screen reader compatibility, alt text for images
- **Translations:** (Future) Translate docs to other languages

### New Documentation

You can add:
- Guides for specific use cases (e.g., "Building a subscription reminder bot")
- FAQ entries
- Troubleshooting tips
- Integration examples for popular frameworks
- Diagrams and visual aids

### Component Improvements

The `docs/components/` folder contains reusable React components. You can:
- Fix bugs in existing components
- Add new components for documentation (e.g., Mermaid diagrams, interactive demos)
- Improve styling and responsiveness

## ❌ What You CANNOT Contribute

- Changes to `api/` — This includes all Rust code, Cargo.toml, dependencies, tests
- Changes to API endpoints, routes, or indexer logic
- Infrastructure changes (Docker, CI/CD for API)

## 📋 How to Contribute

### 1. Fork and Clone

```bash
git clone https://github.com/<your-username>/stellar-subscriptions-api-docs.git
cd stellar-subscriptions-api-docs
```

### 2. Create a Branch

```bash
git checkout -b docs/improve-authorization-model
```

Use branch naming:
- `docs/` prefix for documentation changes
- `fix/` for bug fixes in docs components
- `feat/` for new documentation pages or components

### 3. Make Changes

Edit files in `docs/`:

```bash
cd docs
npm install
npm run dev  # start dev server at http://localhost:3000
```

Test your changes locally before submitting.

### 4. Commit Your Changes

Write clear commit messages:

```bash
git add docs/
git commit -m "docs: clarify cap exhaustion example in authorization model"
```

Commit message format:
- `docs:` for documentation content
- `fix:` for bug fixes
- `style:` for formatting changes
- `chore:` for non-code changes (e.g., updating .gitignore)

### 5. Push and Create PR

```bash
git push origin docs/improve-authorization-model
```

Open a pull request on GitHub with:
- **Clear title:** "Improve authorization model explanation"
- **Description:** What you changed and why
- **Before/After:** Screenshots if applicable

## 🔍 Pull Request Guidelines

### Before Submitting

- [ ] Run `npm run build` in `docs/` to ensure it builds without errors
- [ ] Check for typos and broken links
- [ ] Verify all code examples are correct and tested
- [ ] Ensure changes don't break existing navigation or links
- [ ] Test responsiveness (mobile, tablet, desktop)

### PR Template

```markdown
## What does this PR do?

Brief description of the changes.

## Type of change

- [ ] Documentation improvement
- [ ] New documentation page
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] Component improvement

## Checklist

- [ ] Builds successfully (`npm run build`)
- [ ] No broken links
- [ ] Code examples tested
- [ ] Responsive on mobile
- [ ] Only touches `docs/` folder

## Screenshots (if applicable)

Before:
[image]

After:
[image]
```

## 🎨 Style Guide

### Writing Style

- **Be concise:** Clear and direct is better than verbose
- **Active voice:** "The contract validates the cap" not "The cap is validated by the contract"
- **Code first:** Show code examples, then explain
- **Beginner-friendly:** Assume readers are new to Stellar/subscriptions

### Code Examples

```typescript
// ✅ Good: Complete, runnable example with context
import { Contract } from "@stellar/stellar-sdk";

const contract = new Contract(SUBSCRIPTION_ADDRESS);
const result = await contract.call("charge", subscriptionId);

// ❌ Bad: Incomplete fragment without context
contract.call("charge");
```

### Markdown Formatting

- Use `###` for subsections, `####` for sub-subsections
- Code blocks must specify language: ` ```typescript `
- Use CalloutBoxes for important notes:

```mdx
import CalloutBox from "@/components/CalloutBox";

<CalloutBox type="warning">
This is a warning message.
</CalloutBox>
```

Types: `info`, `warning`, `danger`, `success`

## 🐛 Reporting Issues

### Documentation Issues

Open an issue with:
- **Page/section affected:** Direct link or path
- **Problem description:** What's wrong or unclear
- **Suggested improvement:** How you'd fix it (optional)

### API Bugs or Feature Requests

For API-related issues:
- Label as `api` or `backend`
- Describe the bug or feature
- Maintainers will handle implementation

## 📜 Code of Conduct

Be respectful and professional:
- ✅ Constructive feedback on contributions
- ✅ Helping other contributors
- ✅ Focusing on what's best for the project
- ❌ Personal attacks or harassment
- ❌ Spam or off-topic discussions

Violations will result in removal from the project.

## 📞 Questions?

- **Documentation questions:** Open a GitHub issue
- **Contribution questions:** Ask in PR comments or Discord
- **General support:** See [SUPPORT.md](./SUPPORT.md)

## 🙏 Recognition

Contributors will be added to:
- `docs/app/contributors/page.mdx` (if we create it)
- GitHub's automatic contributor list
- Release notes for significant contributions

Thank you for helping make Stellar Subscriptions better! 🚀
