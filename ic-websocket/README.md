# IC WebSocket App (chat)

Goal: build a real-time chat application on the Internet Computer using the WebSocket gateway pattern. Rust canister + TS client + self-hosted gateway. Learning project — the point is understanding IC's execution model, not shipping fast.

## Architecture

```
Browser ⇄—— WebSocket (push) ——⇄ Gateway (Rust, Docker on VPS) ⇄—— polling (pull, 100ms) ——⇄ Canister
```

- **Client → canister:** WS msg → gateway → ingress call to `ws_message()`. Clients sign with Internet Identity → canister sees the real principal.
- **Canister → client:** `send_message()` writes outbox in canister state → gateway polls outbox (100ms default) → pushes over WS.
- Gateway is a **pull→push converter**. "Push-shaped, poll-powered."

## Stack

- Gateway: https://github.com/omnia-network/ic-websocket-gateway (Docker: `omniadevs/ic-websocket-gateway`)
- Canister CDK (Rust): https://github.com/omnia-network/ic-websocket-cdk-rs
- Working chat example: https://github.com/omnia-network/ic_websocket_example
- Getting started: https://medium.com/@ilbert/websockets-on-the-ic-getting-started-5f8bcdfaabdc

## How IC works (condensed)

- **Canister** = WASM + persistent heap + prepaid cycles. Stateful actor, async `await`, no reentrancy hell.
- **Query calls** = read from one node, fast, FREE. **Update calls** = consensus, ~1–2s, cost cycles.
- Scaling via **subnets** (each ~13–40 machines, own consensus), governed by NNS DAO. Cross-subnet async messaging.
- **Chain-key**: threshold BLS sig per subnet → verify current state with ONE signature, no history replay, blocks garbage-collected.
- Reverse gas: devs pay cycles, users transact free. Canister freezes if cycles run out.
- No native WebSockets (PoC'd, roadmap requires NNS adoption) — hence the gateway.
- VM is WASM → TypeScript canisters exist (Azle). Why "JS for smart contracts" never happened on EVM but works here.

## Key facts / caveats

- Polling won over canister-push (timers + HTTPS outcalls) because queries are free; timers/outcalls burn cycles.
- Gateway is centralized — same trust shape as a rollup sequencer. Fine for chat.
- Effort: weekend-to-a-week with existing Rust + Docker + VPS skills.

## Kickoff (first session)

1. Skim the Medium getting-started article
2. Clone `ic_websocket_example`
3. `dfx start` + deploy the chat demo locally
4. Run the gateway in Docker against local replica

## Related notes

- Task with full plan: Google Tasks → Projects → "IC: Websocket Application"
- Career-lane context: Obsidian → `job-application/2026-09-22-rust-systems-career-lanes.md`
- Content ideas (build-in-public angle): Obsidian → `publicity/crypto-infra/ideas.md`
