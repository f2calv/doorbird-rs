# Copilot Instructions

## Shared instructions

Shared Copilot instructions, skills and prompts are maintained in the public
[account-level `.github` repository](https://github.com/f2calv/.github). They are deliberately not
copied here.

If those shared files are unavailable, stop rather than guessing the conventions.

## Repository role

- Preserve the Cargo workspace containing the reusable `lib` crate and the `app` executable.
- Keep DoorBird protocol and model code in `lib`; keep configuration and executable wiring in `app`.
- Never commit DoorBird credentials, device addresses, response payloads or captured images.
- Treat actuator calls, including door opening and favourite changes, as device-changing operations.
- This is not a container-image repository. The Docker-related files under `.devcontainer` are
  development tooling only.
- Releases use immutable plain semantic-version tags without a `v` prefix or floating aliases.
