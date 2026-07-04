# Digital Signature Strategy

---

## What we use

**Ed25519** (`ed25519-dalek`) — server-held private key signs the SHA-256 hash of canonical report JSON.

| Property | Guarantee |
|----------|-----------|
| Integrity | Any change to payload invalidates signature |
| Authenticity | Proves report originated from this system |
| Cost | Free, open source, no Certificate Authority |

---

## What we do NOT use

**ICP-Brasil** qualified digital signatures require a paid certificate from an accredited Certification Authority.

| Aspect | Ed25519 (this project) | ICP-Brasil |
|--------|------------------------|------------|
| Cost | Free | Paid CA certificate |
| Legal weight (fiscal/judicial) | Not equivalent | Legally recognized in Brazil |
| Use case fit | Trusted sales reports between admin and driver | Official fiscal/legal documents |

For the described use case (reliable sales reports between admin and field staff), Ed25519 integrity + origin proof is sufficient.

If ICP-Brasil becomes required (e.g. fiscal document equivalence), integrate an accredited CA in a future phase — document as ADR.

---

## Canonical JSON (critical)

Same logical content must always produce identical bytes:

- Deterministic key ordering
- No superfluous whitespace
- Stable date/number serialization

Without canonicalization, verification breaks even when data is semantically identical.

---

## Key management

| Rule | Implementation |
|------|----------------|
| Private key storage | Environment variable / secret manager — **never in database** |
| Public key | Stored or derived by `public_key_id` for verification |
| Rotation | `public_key_id` on each Report — old reports verify with old public key |
| Rotation policy | Every 180 days — see [ADR-004](adr/ADR-004-ed25519-key-rotation.md) |

---

## Verification endpoint

`GET /v1/reports/{id}/verify`

1. Load `canonical_payload`, `signature`, `public_key_id`
2. Recompute SHA-256 of canonical bytes
3. Verify Ed25519 signature
4. Return `{ "valid": true | false }` — **public** endpoint, rate limited (ADR-007)

---

## References

| Source | URL |
|--------|-----|
| Ed25519 | https://ed25519.cr.yp.to/ |
| ed25519-dalek | https://docs.rs/ed25519-dalek/ |
| RFC 8032 (EdDSA) | https://www.rfc-editor.org/rfc/rfc8032 |
