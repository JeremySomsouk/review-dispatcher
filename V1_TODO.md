# PRCtrl v1.0.0 Readiness Checklist

## 🔴 Blockers (must fix)

- [x] **Tests: Add test suite** — 15 tests passing — Zero meaningful tests exist. Only 3 no-op tests in `notifications.rs`. Need unit tests for `github.rs`, `stack.rs`, `config.rs`, `logger.rs`, and integration tests for CLI commands. Create `tests/` directory.
- [x] **Structure: Split `main.rs`** — Groundwork laid in a single file with 57 structs and 50+ commands. Split into command modules under `src/commands/` (e.g. `list.rs`, `delegate.rs`, `mine.rs`, etc.) with a shared `client` module for the `Octocrab` instance.
- [x] **Legal: Add `LICENSE` file** — `Cargo.toml` declares MIT but no `LICENSE` file exists. Required for OSS distribution.
- [x] **CI: Add GitHub Actions CI workflow** — Only `deploy-docs.yml` exists. Need `.github/workflows/ci.yml` running `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` on push/PR.

## 🟡 Significant gaps (should fix)

- [x] **CLI: Add `--version` flag** — `cli.rs` has `#[command(name = "prctrl")]` but no `version`, `author`, or `about` attributes. `prctrl --version` won't work. Use `clap::crate_version!()`.
- [ ] **API: Implement pagination** — Octocrab Page API needs `stream` feature; callers still use `.per_page().send()` — All `Octocrab` calls use `.per_page(50/100).send()` once. No follow-up page fetching. Orgs with 100+ open PRs get truncated results. Use octocrab's pagination or fetch all pages.
- [x] **Perf: Shared `Octocrab` client** — `src/client.rs` with `AppContext` and `OnceCell` — 35 separate `Octocrab::builder().personal_token().build().unwrap()` instantiations across `github.rs` and `main.rs`. Should create one client and pass it by reference. Wastes TCP connections and makes mocking impossible.
- [x] **Robustness: Replace `.unwrap()` on client builds** — Replaced with `.expect("failed to build GitHub client")` — 9 `unwrap()` calls in `github.rs` on `Octocrab::builder().build()`. If token is malformed, this panics. Use `.context()?` or `.map_err()?`.
- [x] **CLI: Add `--json` to `Stack` command** — Most commands have `--json` but `Stack` doesn't. `Stack`/`StackedPR` structs lack `Serialize` derives. Inconsistent for a CLI that claims JSON support.
- [x] **CLI: Remove `--tree` flag from Stack** — Removed dead flag, added `--author` instead — `cli.rs` defines `tree: bool` on `Stack` but `main.rs` discards it with `tree: _`. Dead feature advertised in `--help`.
- [x] **Docs: Add `CHANGELOG.md`** — Required for v1.0.0 so users know what changed and what's stable.
- [ ] **Observability: Add `tracing` crate** — Would touch every `println!`; deferred — All output is `println!`/`eprintln!`. No structured logging. `TROUBLESHOOTING.md` recommends `RUST_LOG=debug` but that doesn't work. Replace with `tracing` spans.
- [x] **Docs: Update for convention-based stack detection** — Updated mine.md, added stack.md, added entry to SUMMARY.md — `README.md` and `docs/src/commands/mine.md` only mention branch-chaining stacks. New convention-based detection (ticket key + `[N/M]` markers) is undocumented. `docs/src/SUMMARY.md` has no `stack` entry.
- [x] **Stack: Fix `Stack`/`StackedPR` missing `Serialize`** — Other output structs derive `Serialize` for `--json`, but stack types don't.
- [x] **Docs: Fix unclosed markdown in `README.md`** — The `crew_members` TOML example is missing a closing `"""`.

## 🟢 Nice-to-haves (polish)

- [ ] **Auth: GitHub App support** — Only PAT (personal access token) is supported. GitHub Apps have higher rate limits and org-level permissions. Noted in `TROUBLESHOOTING.md` but not implemented.
- [x] **Resilience: Add retry/backoff** — `src/retry.rs` with exponential backoff (100ms→5s) on 429/5xx and network timeouts. Non-regressive wrapper, adopts incrementally.
- [ ] **Notifications: Linux/Windows support** — `notifications.rs` is macOS-only. No Linux/Windows notification fallback. Should be documented more prominently or implemented.
- [ ] **Automation: Release workflow** — No `release.yml` for automating `cargo publish` + GitHub release + changelog generation.
- [ ] **Chat: Update hardcoded CLI docs** — `chat.rs` embeds `PRCTRL_DOCS` manually. Already outdated (lists commands that don't exist, misses new ones). Should auto-generate from `cli.rs` or a dedicated file.
- [ ] **Cleanup: Remove committed `reviews/`** — `.gitignore` has `reviews/` but the repo still contains `reviews/` with actual review files. Should be removed from git tracking.
- [x] **CLI: Add `--author` filter to `stack` command** — Currently `mine -d` passes author but `stack` standalone doesn't have `--author` flag.
- [x] **Stack: Deduplicate `detect_stacks` calls in `mine`** — Reduced from 3 calls to 1 with result reuse — `mine` command calls `detect_stacks` up to 3 times (line 438, 460, 478). Should call once and reuse the result.
