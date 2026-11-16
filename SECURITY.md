# Security policy

## Supported versions

Only the latest released version receives fixes.

## Reporting a vulnerability

Please report it privately through GitHub:
[Security → Report a vulnerability](https://github.com/julien-langlois/bevel/security/advisories/new)
on this repository. Do not open a public issue.

You will get an answer within a week. Once fixed, the advisory is published on GitHub and
submitted to the [RustSec advisory database](https://rustsec.org).

## Dependencies

`cargo deny check advisories` runs daily in CI against `Cargo.lock`
(`.github/workflows/security.yml`), and Dependabot proposes lockfile updates weekly.
