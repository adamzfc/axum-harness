# Entry Criteria

## Large-Scale Business Development Gate

The repo is allowed to enter wider parallel business implementation only when the following remain true:

1. `counter-service` and `tenant-service` stay stable as the two primary reference services.
2. At least one synchronous chain, one workflow chain, and one replay/projection chain are still represented by the live repo skeleton.
3. `agent/codemap.yml`, `agent/templates/`, skill docs, and validation scripts agree on the same structure.
4. `just validate-state strict`, `just validate-workflows strict`, and `just verify-replay strict` all pass.
5. Stub or deprecated services are clearly marked and are not used as templates.

## Current Interpretation

1. `counter-service` covers the smallest command/query/event path.
2. `tenant-service` covers multi-entity and workflow semantics.
3. `outbox-relay` and `projector` cover async relay, projection, rebuild, and replay semantics.
