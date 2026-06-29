#!/usr/bin/env bash
set -euo pipefail

# Local development fixture helper only. It generates a fresh Ed25519 key pair,
# commits no private key, and signs the local-dev checksum manifest.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bundle_root="${repo_root}/examples/policy-bundles/local-dev"
checksums_path="${bundle_root}/checksums/SHA256SUMS"
signatures_root="${bundle_root}/signatures"
tmpdir="$(mktemp -d)"

cleanup() {
  rm -rf "${tmpdir}"
}

trap cleanup EXIT

python3 - "${bundle_root}" <<'PY'
import hashlib
import sys
from pathlib import Path

root = Path(sys.argv[1])
files = ["manifest.yaml", "gateway_policy.yaml", "risk_matrix.yaml"]
with (root / "checksums" / "SHA256SUMS").open("w", encoding="utf-8") as out:
    for name in files:
        digest = hashlib.sha256((root / name).read_bytes()).hexdigest()
        out.write(f"{digest}  {name}\n")
PY

openssl genpkey -algorithm Ed25519 -out "${tmpdir}/local-dev-private.pem"
openssl pkey -in "${tmpdir}/local-dev-private.pem" -pubout -out "${signatures_root}/public.pem"
openssl pkeyutl -sign -inkey "${tmpdir}/local-dev-private.pem" -rawin -in "${checksums_path}" -out "${tmpdir}/SHA256SUMS.sig"
base64 -i "${tmpdir}/SHA256SUMS.sig" > "${signatures_root}/SHA256SUMS.sig"
