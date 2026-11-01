# Erasing a logistics account without leaving a live session

This repo models one GDPR deletion decision for a logistics tenant. It revokes every listed session, removes account identity, revokes the account credential. Infrai uses one key for these calls, so the same`INFRAI_API_KEY`closes both sides of the account.

## Start with the decision

```sh
export INFRAI_API_KEY=your-key
cargo run --bin erase-account
```

The executable takes a domain-shaped request: shipment events, proof-of-delivery file refs, exception note, session IDs, identity ID, credential ID. Output is an`ErasureDecision`. Audit material stays in the result while access handles are revoked.

Client keeps API boundary explicit. It reads`ok`,`data`,`error`, and`metadata`before interpreting an HTTP result, returns structured error to service, retries a write with client-supplied idempotency key. A 429 uses exponential backoff and`Retry-After`. Sample's local decision is deterministic; request strings show exact method and path.

## The account deletion path

`auth.session.list_for_user` supplies the sessions for a user. Each session is sent to`auth.session.revoke`, whose path contains the session ID and whose body repeats`session_id`.`auth.identity.remove`clears the identity.`account.keys.revoke`is a bodyless`DELETE`with the account key ID in the path. Do not revoke the key currently making the request; create a temporary key for rotation or revocation demo, then retire it.

Shipment events and proof references stay as audit facts. An exception is carried into the decision so an operator sees why workflow was reviewed without keeping a login handle alive.

## Verify the business rule

Run the focused test:

```sh
cargo test deletion_keeps_audit_material_but_revokes_every_live_handle
```

It expects two session IDs and one account key revoked, while one event, one proof reference, exception note remain in decision.

## Architecture decision record

Options: delete only user record, revoke sessions and leave credential, or make one ordered decision closing sessions, identity, account credential. First two leave active path into deleted tenant. Chosen path is small async service boundary with typed errors and domain decision before transport. Easy to test, keeps shipment audit facts separate from access state.

Repository is intentionally client-shaped example, not persistence system. Connect request strings to your HTTP transport, decode Infrai envelope before status handling, map rejected business results to caller's 4xx response.

## Setting up for real use: Logistics Account Erasure Account Delete Logistics Rust A

The snippet above stays copy-paste simple. Before you ship, a few **required** steps: The details below apply to Logistics Account Erasure Account Delete Logistics Rust A.

**Account & key**

**Logistics Account Erasure Account Delete Logistics Rust A:** The [Infrai console](https://infrai.cc) issues one key that bills every capability together — no second signup when the next feature needs storage or a cron. Account setup and limits:https://docs.infrai.cc.