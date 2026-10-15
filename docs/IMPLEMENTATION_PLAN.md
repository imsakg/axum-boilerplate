# Authentication and API Implementation Plan

> **For Hermes:** Execute small test-backed slices and independently verify the final tree.

**Goal:** Expand the learning boilerplate into a usable single-node API and a
transparent 80-commit local history without touching the original dirty checkout.

**Architecture:** An Axum library router is shared by the binary and hermetic
in-process tests. A SQLite repository enforces user/ticket ownership, and a typed
service layer handles password hashing, short-lived JWT access tokens, persisted
single-use refresh tokens, account verification, and password reset challenges.

**Tech stack:** Rust 2021, Axum 0.6, Tokio, tower-http, SQLite/rusqlite,
Argon2, jsonwebtoken, serde, tracing.

## Small-slice sequence

1. Repair the stale dependency build; separate manual smoke from automated tests.
2. Extract `src/lib.rs` router; test health and error response contracts first.
3. Add `src/config.rs`, validation, crypto helpers, then `src/db.rs` migrations.
4. Add user registration/login and bearer authentication. Write HTTP tests before
   each route; verify failure before implementation, then run focused and full tests.
5. Add refresh rotation, logout, verification and reset via one-time tokens. Keep
   delivery behind a local outbox, never in public HTTP responses.
6. Add owned ticket create/read/list/update/delete routes, bounded pagination and
   owner isolation. Exercise actual SQLite constraints and persistence in tests.
7. Add defensive middleware (request IDs, body limits, timeouts, rate limits,
   security headers), typed rejections, and regression coverage for malformed input.
8. Document runnable examples, environment, database lifecycle, security boundaries,
   development and deployment. No imaginary production/email infrastructure.
9. Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
   `cargo test --all-targets --all-features`, doctests, and a live HTTP smoke.
10. Verify exact commit count, nonempty commits, chronological weekday/evening dates,
    signatures and clean secondary worktree; hash-check the original dirty checkout.

## Acceptance and deferrals

- Exactly 80 reachable commits on `history/auth-api-buildout`, June 2022–November
  2023 schedule; original history and pending edits remain available on `main`.
- Actual auth, persistence, middleware, endpoints, tests, docs and regression fixes.
- SQLite is single-node, no passwordless/OAuth provider or distributed rate limiter.
- No push, remote mutation, production secrets or network-dependent test suite.
