# WeilGuard — Demo Script for Judges
## Xpecto Weilliptic Hackathon 2026

---

## Demo Flow (8 minutes)

### Minute 0–1 | Problem Setup
"Enterprise AI agents — email bots, data analysts, code deployers — 
are executing with zero accountability. When they go wrong, there's 
no trail. WeilGuard fixes this on WeilChain."

---

### Minute 1–2 | Show the WIDL File
Open `weilguard.widl` in VS Code.

Key points to highlight:
- `register_agent` — onboards an agent with a JSON governance policy
- `check_policy` — **pre-execution gate** before any action
- `log_event` — tamper-proof immutable action log with auto-policy-check
- `compliance_score` — real-time risk score per agent

"WIDL is the single source of truth. From here the Weilliptic compiler 
generates our Rust bindings. We fill in the logic. We deploy."

---

### Minute 2–3 | Show the Applet (lib.rs)
Highlight the policy engine in `check_policy_internal`:
- Explicit deny list (checked first)
- Human-approval gate
- Allow-list enforcement

"All of this runs inside WeilChain — cryptographically verified, 
not on someone's laptop."

---

### Minute 3–5 | Live Demo via Icarus MCP

Open Icarus in VS Code. Connect to the running WeilGuard MCP server.

**Step 1 — Register Agent:**
> "Register agent `email-bot` with policy: allow send_email and schedule_meeting, 
> deny delete_inbox and forward_all, require human approval for send_bulk_email"

Expected: Returns agent_id confirmation.

**Step 2 — Pre-Execution Policy Check (ALLOW):**
> "Check if email-bot can perform send_email"

Expected: `ALLOW`

**Step 3 — Pre-Execution Policy Check (DENY):**
> "Check if email-bot can perform delete_inbox"

Expected: `DENY: Action 'delete_inbox' is explicitly prohibited`

**Step 4 — Log a Compliant Action:**
> "Log that email-bot sent an email to team@company.com about Q3 report"

Expected: Returns sha256 event_id (e.g. `a3f2c1...`)

**Step 5 — Simulate a Policy Breach:**
> "Log that email-bot tried to perform delete_inbox"

Expected: Logged AND auto-flagged as HIGH violation.

**Step 6 — Check Compliance Score:**
> "What is email-bot's compliance score?"

Expected: `90` (one HIGH violation = -10)

**Step 7 — Get Full Audit Trail:**
> "Show me the complete audit log for email-bot"

Expected: JSON array with both events, each with immutable hash.

**Step 8 — Revoke Misbehaving Agent:**
> "Revoke email-bot — reason: violated data privacy policy"

Expected: Confirmed revoked. All future actions blocked.

---

### Minute 5–7 | Architecture Slide
Show the architecture diagram:

```
Enterprise LLM / Icarus
        │
        ▼  MCP (9 tools)
 WeilGuard MCP Server
        │
        ▼  weil-sdk
  WeilGuard Applet (Rust/WASM)
  deployed on WeilChain Asia Pod
        │
        ▼
  Immutable On-Chain State
```

Key talking points:
1. "WIDL-first development — the interface is the contract"
2. "Policy checked ON-CHAIN, not on a mutable server"
3. "Any Cerebrum agent can call check_policy before acting"
4. "Icarus gives enterprises a live governance dashboard"

---

### Minute 7–8 | Why This Matters for Enterprises
- SOC2 / ISO 27001 compliance requires audit trails
- AI agents are now subject to EU AI Act requirements  
- WeilGuard makes compliance **automatic and verifiable by design**
- Deployed on Asia Pod for low-latency access from India/APAC

---

## Key Differentiators vs. Traditional Logging
| Feature | Traditional DB Log | WeilGuard on WeilChain |
|---|---|---|
| Tamper-proof | ❌ Admin can delete | ✅ Cryptographic |
| Pre-execution gate | ❌ Log after the fact | ✅ check_policy before |
| Decentralised | ❌ Single point of failure | ✅ Distributed |
| Verifiable by third party | ❌ Requires trust | ✅ On-chain proof |
| Kill-switch | ❌ Manual process | ✅ revoke_agent() |
