# ADR-052 — Seller catalog share link

**Status:** Accepted  
**Date:** 2026-07-19  
**Phase:** 19

## Context

Field sellers need a personal catalog URL so portal WhatsApp / “Contact seller” reaches their phone. The tenant already has a single `sales_contact_phone`. Changing the shared catalog per seller is out of scope.

## Decision

1. Extend `identity.seller_profiles` with `public_code`, `contact_phone`, `share_link_active` (additive, nullable where safe).
2. Public resolve: `GET /v1/public/sellers/{publicCode}` — returns display name + optional phone only.
3. Portal entry: `/s/{publicCode}` resolves, stores attribution in `sessionStorage`, redirects into the normal catalog routes.
4. Contact precedence: seller phone → tenant `sales_contact_phone` → CTA disabled.
5. Order attribution deferred (YAGNI until commission/CRM is requested).
6. Absolute share URLs come from the API (`shareUrl` via `PORTAL_PUBLIC_ORIGIN`) — clients never hardcode the catalog host.

## Consequences

- Default portal (`/` without `/s/…`) unchanged.
- Existing `PUT /v1/users/{id}/seller-profile` remains backward compatible when new fields are omitted.
- Phone numbers never appear in share URLs.
- Ops sets `PORTAL_PUBLIC_ORIGIN` to the public catalog frontend (LAN IP in local dev).
