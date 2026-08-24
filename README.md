Disclaimer: All files in this repo are AI generated except this README.

<details>
<summary>Models used</summary>

- Used to improve prompt: Grok 4.5 Fast
- Used to generate the files: Gemini 3.1 Pro Extended Thinking and Gemini 3.6 Flash Extended Thinking

</details>

<details>
<summary>Prompts</summary>

<details>
<summary>rustfmt.toml</summary>

```md
Provide the best production-grade rustfmt.toml for a modern Rust workspace (libraries, binaries, multi-crate, multi-target). Target toolchain: rustc 1.98.0 / cargo 1.98.0 / clippy 0.1.98 (2026-08). Prefer modern, idiomatic, safety-oriented settings that maximize consistency, readability, and maintainability. Use only stable options available on this toolchain. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only. I will attach my current rustfmt.toml if I have one.
```
</details>

<details>
<summary>clippy.toml</summary>

```md
Provide the best production-grade .clippy.toml (or clippy.toml) for a modern Rust workspace. Target toolchain: rustc 1.98.0 / cargo 1.98.0 / clippy 0.1.98 (2026-08). Prefer the strictest practical lints from pedantic, restriction, nursery, and style groups that still allow productive day-to-day work. Explicitly mark any lint left as warn or allow and briefly explain why. Never put [lints.clippy] inside Cargo.toml. Use only options available on this toolchain. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only. I will attach my current .clippy.toml if I have one.
```
</details>

<details>
<summary>.cargo/config.toml</summary>

```md
Provide the best production-grade .cargo/config.toml for a modern Rust workspace supporting multi-target cross-compilation (Android, Windows MSVC, Linux GNU/musl, wasm, etc.). Target toolchain: rustc 1.98.0 / cargo 1.98.0 (2026-08). Strongly prioritize the mold linker on Linux for faster linking, with clean configuration and sensible fallbacks or notes for other platforms. Include useful linker settings, build performance improvements, and target-specific flags where beneficial. Use only stable options. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only. I will attach my current .cargo/config.toml if I have one.
```
</details>

<details>
<summary>deny.toml</summary>

```md
Provide the best production-grade deny.toml for cargo-deny in a modern Rust workspace. Target toolchain: rustc 1.98.0 / cargo 1.98.0 (2026-08). Maximize safety, license compliance, advisory checking, and ban of problematic crates while remaining practical for day-to-day development. Cover advisories, bans, licenses, and sources. Use only options compatible with current cargo-deny. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only. I will attach my current deny.toml if I have one.
```
</details>

<details>
<summary>rust-toolchain.toml</summary>

```md
Provide the best production-grade rust-toolchain.toml for a modern Rust workspace. Pin exactly to: rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0, clippy 0.1.98, rustfmt, and any other essential components. Support multi-target cross-compilation. Keep it minimal, reproducible, and fully compatible with rustup 1.29.0. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only. I will attach my current rust-toolchain.toml if I have one.
```
</details>

<details>
<summary>Makefile</summary>

```md
Rewrite and improve the provided Makefile for a modern multi-target Rust workspace. Goals: faster builds, more secure defaults, better optimization, cleaner structure, parallelism, clearer targets, and mold integration where applicable on Linux. Preserve multi-target support for: aarch64-linux-android, aarch64-pc-windows-msvc, aarch64-unknown-linux-gnu, armv7-linux-androideabi, i686-linux-android, wasm32-unknown-unknown, x86_64-linux-android, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl. Target toolchain: rustc 1.98.0 / cargo 1.98.0 (2026-08). Keep cargo xwin for MSVC targets. Explain every non-obvious change with a short comment. Output the complete improved Makefile only.
```
</details>

<details>
<summary>.editorconfig</summary>

```md
Provide the best production-grade .editorconfig for a modern Rust workspace. Enforce consistent indentation, line endings, charset, and trailing whitespace rules that align with idiomatic Rust and the recommended rustfmt settings. Keep it editor-agnostic so it works equally with VS Code, IntelliJ IDEA, RustRover, Neovim, and others. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only.
```
</details>

<details>
<summary>nextest.toml</summary>

```md
If clearly beneficial, provide a production-grade .config/nextest.toml (or nextest.toml) for cargo-nextest in a modern multi-crate Rust workspace. Focus on faster, more reliable, parallel test execution with sensible defaults for CI and local development. Target toolchain: rustc 1.98.0 / cargo 1.98.0 (2026-08). Use only stable options. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only, or state that it is not needed.
```
</details>

<details>
<summary>.vscode/settings.json</summary>

```md
Provide optional, non-exclusive VS Code / rust-analyzer settings.json improvements that enhance the developer experience for a modern Rust workspace. Keep core project configs fully editor-agnostic so the project works equally well with IntelliJ IDEA, RustRover, Neovim, and others. Mark this file clearly as optional. Target toolchain: rustc 1.98.0 / rust-analyzer compatible with 1.98. Explain every non-obvious setting with a short comment. Output the complete ready-to-use file contents only.
```
</details>

<details>
<summary>.gitattributes</summary>

```md
Provide the best production-grade .gitattributes for a modern multi-crate Rust workspace that also contains CI configs, documentation, and potential binary artifacts. Enforce correct line endings (LF for source, handle Windows files properly), mark generated or binary files appropriately, set linguist overrides for accurate language detection, and prevent accidental diffs on lockfiles or build outputs where beneficial. Keep it simple, portable, and compatible with GitHub, Codeberg/Forgejo, and standard Git clients. Explain every non-obvious rule with a short comment. Output the complete ready-to-use file contents only. I will attach my current .gitattributes if I have one.
```
</details>

<details>
<summary>.github/workflows/ci.yml</summary>

```md
Provide the best production-grade GitHub Actions workflow file (.github/workflows/ci.yml) for a modern multi-crate Rust workspace. It must run on every pull request and on pushes to the default branch. Required checks that must pass before a PR can be merged: cargo check, cargo test (including nextest if beneficial), cargo clippy with the project's strict settings, cargo fmt --check, and cargo deny check. Support the project's multi-target nature where practical without making CI excessively slow. Use the exact toolchain from rust-toolchain.toml (rustc 1.98.0 / cargo 1.98.0 / clippy 0.1.98). Cache dependencies aggressively, prefer mold on Linux runners, run in parallel jobs where sensible, and fail fast on errors. Keep the workflow secure (pin actions by SHA or major version carefully, minimal permissions). Explain every non-obvious step with a short comment. Output the complete ready-to-use YAML file contents only. I will attach my current workflow files if I have any.
```
</details>

<details>
<summary>.forgejo/workflows/ci.yml</summary>

```md
Provide the best production-grade Forgejo Actions / Codeberg CI workflow file (.forgejo/workflows/ci.yml) that is as close as possible to a GitHub Actions equivalent. It must run on every pull request and on pushes to the default branch. Required checks that must pass before a PR can be merged: cargo check, cargo test (including nextest if beneficial), cargo clippy with the project's strict settings, cargo fmt --check, and cargo deny check. Support the project's multi-target nature where practical without making CI excessively slow. Use the exact toolchain from rust-toolchain.toml (rustc 1.98.0 / cargo 1.98.0 / clippy 0.1.98). Cache dependencies aggressively, prefer mold on Linux runners, run in parallel jobs where sensible, and fail fast on errors. Keep the workflow secure and compatible with Codeberg/Forgejo runners. Explain every non-obvious step with a short comment. Output the complete ready-to-use YAML file contents only. I will attach my current CI files if I have any.
```
</details>

<details>
<summary>.woodpecker.yml</summary>

```md
If the project prefers classic Woodpecker CI on Codeberg instead of (or in addition to) Forgejo Actions, provide the best production-grade .woodpecker.yml. It must run on every pull request and on pushes to the default branch. Required checks that must pass: cargo check, cargo test, cargo clippy (strict), cargo fmt --check, and cargo deny check. Use the exact toolchain from rust-toolchain.toml (rustc 1.98.0). Cache aggressively, keep pipelines fast and parallel where possible, and fail if any required step fails. Explain every non-obvious step with a short comment. Output the complete ready-to-use file contents only, or state that Forgejo Actions is preferred and this is optional.
```
</details>

<details>
<summary>.gitignore</summary>

```md
Provide an improved production-grade .gitignore tailored for a modern multi-crate Rust workspace with multi-target builds, Android, Windows, Linux, wasm, CI artifacts, and editor files. Cover target/, Cargo.lock rules if needed, IDE folders, OS junk, mold/lld caches, and common secrets or local overrides. Keep it comprehensive yet not overly aggressive. Explain any non-obvious entries with a short comment. Output the complete ready-to-use file contents only. I will attach my current .gitignore if I have one.
```
</details>

<details>
<summary>PR / branch protection notes (documentation)</summary>

```md
Provide a short, clear Markdown snippet (suitable for CONTRIBUTING.md or a docs/ci.md section) that explains the required CI checks for pull requests on both GitHub and Codeberg. State that all of the following must pass before a PR is mergeable: formatting, Clippy (strict), tests, cargo-deny, and basic check. Mention that the same quality gates apply on both platforms. Keep it concise and editor-agnostic. Output the complete ready-to-use Markdown contents only.
```
</details>
</details>