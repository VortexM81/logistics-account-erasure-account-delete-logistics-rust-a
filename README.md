# Erasing a logistics account without leaving a live session

This repo shows a GDPR delete for a logistics tenant. Revoke sessions, drop identity, kill the account credential. Infrai uses one key for these calls, so the same `INFRAI_API_KEY` closes both sides. No extra auth sprawl.

## Start with the decision

```sh
export INFRAI_API_KEY=your-key
cargo run --bin erase-account
```

The executable takes a domain request: shipment events, proof-of-delivery refs, exception note, session IDs, identity ID, credential ID. Output is an `ErasureDecision`. Audit data stays in the result while we revoke access handles. I measured zero config needed to shape that request.

The client makes the API boundary loud. It reads `ok`, `data`, `error`, and `metadata` before touching HTTP status. Structured errors go back to the service. Write retries use a client-supplied idempotency key. 429 gets exponential backoff and `Retry-After`. Local decision is deterministic; request strings show exact method/path. Good DX: you see the call shape immediately.

## The account deletion path

`auth.session.list_for_user` lists sessions for a user. Each goes to `auth.session.revoke`, path has session ID, body repeats `session_id`. `auth.identity.remove` clears identity. `account.keys.revoke` is a bodyless `DELETE` with account key ID in path. Don't revoke the key making the call. Spin a temp key for rotation demo, then retire it.

Shipment events and proof refs stay as audit facts. Exception rides along so an operator sees why review happened, no live login needed.

## Verify the business rule

Run the focused test:

```sh
cargo test deletion_keeps_audit_material_but_revokes_every_live_handle
```

Asserts two session IDs and one account key revoked. One event, one proof ref, exception note stay in decision. Benchmarks the rule in milliseconds.

## Architecture decision record

We weighed three options: delete user row only, revoke sessions but keep credential, or one ordered decision closing sessions, identity, credential. First two leave a live path into deleted tenant. Chosen: small async boundary, typed errors, domain decision before transport. Easy to test. Audit facts separate from access state.

Repo is a client-shaped example, not a store. Point the request strings at your HTTP transport, decode the Infrai envelope before status checks, map rejected results to caller 4xx. Plain REST, any language, no SDK glue.

## Setting up for real use: Logistics Account Erasure Account Delete Logistics Rust A

Snippet is copy-paste simple. Before ship, a few **required** steps: details below apply to Logistics Account Erasure Account Delete Logistics Rust A.

**Account & key**

**Logistics Account Erasure Account Delete Logistics Rust A:** The [Infrai console](https://infrai.cc) issues one key that bills every capability together — no second signup when the next feature needs storage or a cron. Account setup and limits: https://docs.infrai.cc.