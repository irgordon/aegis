# AEGIS v0.4.2 Developer Preview Refresh Artifact

This artifact is part of the planned AEGIS v0.4.2 Developer Preview Refresh.

It is unsigned, not notarized, archive-based, and developer-oriented.

It is not production-ready or enterprise-hardened.

It is provided as an archive, not an installer.

It contains binary build output, a bundled local development policy bundle, and a safe `health.check` request fixture for a gateway smoke test.

It does not include installer packaging, app bundle packaging, production configuration, signing, notarization, or auto-update.

## First Five Minutes

Use this flow from the directory where you downloaded the archive and `SHA256SUMS`.

1. Verify the selected archive.

For Apple Silicon Macs:

```bash
grep 'aegis-v0.4.2-macos-arm64.tar.gz' SHA256SUMS | shasum -a 256 -c -
```

For Intel Macs:

```bash
grep 'aegis-v0.4.2-macos-x64.tar.gz' SHA256SUMS | shasum -a 256 -c -
```

If you downloaded both macOS archives and `SHA256SUMS`, verify the full manifest:

```bash
shasum -a 256 -c SHA256SUMS
```

2. Extract the archive.

For Apple Silicon Macs:

```bash
tar -xzf aegis-v0.4.2-macos-arm64.tar.gz
cd aegis-v0.4.2-macos-arm64
```

For Intel Macs:

```bash
tar -xzf aegis-v0.4.2-macos-x64.tar.gz
cd aegis-v0.4.2-macos-x64
```

3. Launch the desktop binary.

```bash
./desktop/aegis-desktop
```

4. Run the gateway smoke test.

```bash
./bin/aegis-gateway --bundle policy-bundles/local-dev examples/health-check-request.json
```

For gateway help, run:

```bash
./bin/aegis-gateway --help
```

5. Review the structured output.

Look for evidence that the request was validated, policy was verified, execution was authorized, the `health.check` wrapper ran, audit evidence was produced, state evidence was produced, and execution completed.

The smoke test is read-only. It does not run mutation wrappers, issue credentials, approve work, replay executions, or recover executions.

## Artifact Layout

```text
README.md
ARTIFACT-CONTENTS.md
examples/health-check-request.json
policy-bundles/local-dev/
bin/aegis-gateway
desktop/aegis-desktop
```

Validate the SHA-256 checksum before use.

Do not treat this artifact as a trusted production distribution.
