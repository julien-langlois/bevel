# Releasing

The five crates share one version and are released together with
[cargo-release](https://github.com/crate-ci/cargo-release) (`release.toml`):

1. **On your machine**, `cargo release` writes the changelog section, makes a GPG-signed
   commit and a GPG-signed `vX.Y.Z` tag with your key, and pushes them.
2. **On GitHub**, the tag starts `.github/workflows/release.yml`, which publishes the crates
   to crates.io with the `CARGO_REGISTRY_TOKEN` secret and creates the GitHub release.

`cargo release` never publishes by itself (`publish = false`): only the tag's workflow does.

## Setup, once

### On your machine

1. **Tools:** `cargo install cargo-release git-cliff`.
2. **GPG signing.** cargo-release signs with git's own setup:

   ```sh
   git config user.signingkey          # the key to sign with (its long ID)
   gpg --list-secret-keys --keyid-format long <KEYID>   # present, not expired
   echo test | gpg --clearsign --local-user <KEYID> > /dev/null   # asks for the passphrase
   ```

   Set the key once for this repository with `git config user.signingkey <KEYID>`. gpg asks
   for the passphrase through `gpg-agent` (pinentry) during `cargo release --execute`; if it
   fails with "Inappropriate ioctl for device", run `export GPG_TTY=$(tty)` first.
3. **"Verified" on GitHub:** add the key's public part at
   <https://github.com/settings/keys> (`gpg --armor --export <KEYID>`), with the e-mail
   address of your commits among its identities.
4. **The `main` branch:** releases are only cut from `main` (`allow-branch`), which is also
   the branch the CI and the Pages deployment follow.

### On crates.io

<https://crates.io/settings/tokens> → New token:

- name: `bevel GitHub Actions`;
- scopes: `publish-new` and `publish-update`;
- crates: `bevel-*` (the pattern also covers the first publication);
- an expiry date, noted somewhere: the release fails with a 403 once it passes.

### On GitHub

1. Settings → Environments → New environment **`release`**:
   - *Deployment branches and tags*: selected tags, rule `v*`;
   - optionally *Required reviewers*: yourself, to approve each publication;
   - *Environment secrets*: **`CARGO_REGISTRY_TOKEN`** = the crates.io token.

   An environment secret is only given to jobs that run in that environment (the publish job,
   on a `v*` tag). A repository secret with the same name also works, but every workflow
   could then read it.
2. Settings → Pages → Source: **GitHub Actions** (the guide, `.github/workflows/docs.yml`).

## First release (0.1.0)

The manifests are already at 0.1.0: run `cargo release` **without a level**, which releases
the current version as it is, with no bump.

```sh
git switch main && git pull         # clean, up to date, CI green
cargo release                       # dry run: prints the changelog section, the commit and the tag
cargo release --execute             # gpg asks for your passphrase (commit, then tag), then it pushes
```

What `--execute` does, in order:

1. prepends the `0.1.0` section to `CHANGELOG.md` (git-cliff, from the commits since the start);
2. commits it, signed: `chore(release): v0.1.0`;
3. tags `v0.1.0`, signed, with the message `Release v0.1.0`;
4. pushes the commit and the tag to `origin`.

The tag starts the release workflow on GitHub. With a required reviewer on `release`, approve
the run in the Actions tab; the crates are then on crates.io a few minutes later.

## Every later release

```sh
cargo release minor                 # or patch, major: dry run
cargo release minor --execute
```

The same steps, plus the version bump of every crate and of the `[workspace.dependencies]`
versions. At 0.x, a breaking change is a `minor`, anything else a `patch`; the commits'
[Conventional Commits](https://www.conventionalcommits.org) types (`feat:`, `fix:`, `feat!:`)
make up the changelog.

## Checking a release

```sh
git verify-tag v0.1.0               # "Good signature from …"
git log --show-signature -1 v0.1.0
```

On GitHub the commit and the tag show "Verified" once the public key is on your account.

## If something fails

- **Before the push** (signing refused, hook failed): nothing is pushed. Undo the local
  commit and tag (`git tag -d vX.Y.Z && git reset --hard HEAD~1`) and run it again.
- **The publish job failed** (expired token, crates.io down): fix the cause and re-run the
  workflow from the Actions tab. Crates already published at that version are skipped.
- **A bad version is published:** `cargo yank --version X.Y.Z <crate>` for each affected
  crate, then a patch release.

## Renewing the token

Create a new token as above, replace the `CARGO_REGISTRY_TOKEN` environment secret, then
revoke the old token on crates.io.
