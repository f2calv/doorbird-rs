# DoorBird API client for Rust

An experimental Rust workspace containing a reusable DoorBird LAN API client and a small
application that exercises it. The project is an early prototype and is not published as a crate.

## Workspace

| Package | Purpose |
| --- | --- |
| `lib` | Models DoorBird responses and calls selected LAN API endpoints. |
| `app` | Loads local configuration and exercises the library. |

The current API surface covers sessions, device information, live images, favourites and door
opening. Refer to the [DoorBird LAN API documentation](https://www.doorbird.com/downloads/api_lan.pdf?rev=0.32)
for the upstream protocol.

## Configuration

Public defaults live in [`appsettings.toml`](appsettings.toml). Put device-specific settings in the
gitignored `appsettings.local.toml`, or use environment variables with the `APP_` prefix and `__`
between sections:

```toml
[doorbird_config]
ip = "192.0.2.10"
username = "example-user"
password = "replace-at-runtime"
```

Do not commit real device addresses or credentials. The sample application currently performs a
write request, so review `app/src/main.rs` and use a test device before running it.

## Development

The Dev Container provides Rust, `pre-commit` and matching VS Code extensions without upgrading
packages at startup. From the repository root:

```console
pre-commit run --all-files
cargo fmt --all --check
cargo clippy -- -D warnings
cargo build --release
```

CI runs linting, GitVersion calculation and the Rust build through reusable workflows. Releases use
immutable `MAJOR.MINOR.PATCH` tags without a `v` prefix or floating aliases.

## Support and license

Use GitHub issues for non-sensitive defects and feature requests. Report vulnerabilities through
the repository's Security tab rather than a public issue.

This project is available under the [MIT License](LICENSE).
