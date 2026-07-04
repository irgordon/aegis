# AEGIS v0.4.1 Developer Preview Artifact

This artifact is part of the public AEGIS v0.4.1 Developer Preview.

It is unsigned, not notarized, archive-based, and developer-oriented.

It is not production-ready or enterprise-hardened.

It is provided as an archive, not an installer.

It contains binary build output and the bundled local development policy bundle used for fixed health-check evidence.

It does not include request fixture files, installer packaging, app bundle packaging, or production configuration.

Validate the SHA-256 checksum before use.

From the directory where you downloaded the archive and `SHA256SUMS`, verify the selected archive:

```bash
grep 'aegis-v0.4.1-macos-arm64.tar.gz' SHA256SUMS | shasum -a 256 -c -
```

Use `aegis-v0.4.1-macos-x64.tar.gz` in the command for the Intel macOS archive.

If you downloaded both macOS archives and `SHA256SUMS`, verify the full manifest:

```bash
shasum -a 256 -c SHA256SUMS
```

Launch the desktop binary from the extracted archive:

```bash
./desktop/aegis-desktop
```

Launch the gateway binary to see its current command shape:

```bash
./bin/aegis-gateway
```

Do not treat this artifact as a trusted production distribution.
