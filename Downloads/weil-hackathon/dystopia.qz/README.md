# WeilGuard 🛡️
#On-Chain AI Agent Security, Policy Enforcement & Compliance Monitoring

## Live Demo
Watch WeilGuard in action: https://www.loom.com/share/eb55932e052845a88d2a1f430068bbba
  
> Platform: **WeilChain** (Weilliptic)  
> Track: Autonomous Agent Security


---

## 📌 Problem Statement

Autonomous AI agents are being deployed to read databases, send emails, trigger deployments, and execute financial transactions. Yet today, there is **no cryptographically verifiable record** of what an agent did, why it was allowed, or who approved it. If an agent misbehaves, the enterprise has no audit trail, no compliance proof, and no kill-switch.

---

## 💡 Solution — WeilGuard

WeilGuard is an **on-chain AI agent governance layer** deployed on WeilChain. It gives every AI agent:

| Feature | What it does |
|---|---|
| **Cryptographic Audit Log** | Every agent action is hashed (SHA-256) and stored immutably on-chain |
| **Policy Engine** | JSON-based governance policy → pre-execution `check_policy` call returns `ALLOW` or `DENY:<reason>` |
| **Compliance Score** | 0–100 real-time score calculated from violation history |
| **Violation Registry** | Automatic and manual flagging with severity levels (LOW / MEDIUM / HIGH / CRITICAL) |
| **Revocation** | Permanently deactivate a rogue agent with a single on-chain call |
| **MCP Server** | Exposes all of the above as tools to any LLM via Icarus or Claude |

---

## 🏗️ Architecture

```
Enterprise LLM Agent
        │
        ▼  (MCP protocol)
 WeilGuard MCP Server  ◄──── Icarus UI
        │
        ▼  (WeilChain SDK)
  WeilGuard Applet  ◄──── Deployed on WeilChain (Asia Pod)
        │
        ▼
  Immutable On-Chain State
  (agents, audit_logs, violations)
```

### Technology Stack
- **WIDL** → Interface definition (`weilguard.widl`)
- **Rust + WASM** → Applet business logic compiled via `wasm-pack`
- **WeilChain** → Distributed cryptographically-secure execution layer
- **TypeScript MCP Server** → Exposes 9 tools to LLMs via Icarus
- **`@weilliptic/weil-sdk`** → On-chain interaction

---

## 📂 Project Structure

```
WeilGuard/
├── weilguard.widl          ← WIDL interface (source of truth)
├── applet/
│   ├── Cargo.toml
│   └── src/
│       ├── bindings.rs     ← WIDL-generated skeleton
│       └── lib.rs          ← Business logic
├── mcp/
│   ├── package.json
│   ├── tsconfig.json
│   └── src/
│       └── index.ts        ← MCP server (9 tools)
├── docs/
│   └── demo_script.md
└── README.md
```

---

## 🚀 Setup & Deployment

### Prerequisites
- Rust + `wasm-pack`
- `wcli` (Weilliptic CLI) — from [wadk releases](https://github.com/weilliptic-public/wadk/releases)
- Node.js 18+
- WeilChain Chrome extension installed

### Step 1 — Write WIDL & Generate Bindings
```bash
# Install WIDL compiler (download binary from wadk releases)
chmod +x widlc

# Generate Rust bindings from WIDL
./widlc weilguard.widl --lang rust --out applet/src/bindings.rs
```

### Step 2 — Build the Applet
```bash
cd applet
cargo build --release --target wasm32-unknown-unknown
# Or use wasm-pack for optimised output:
wasm-pack build --target web
```

### Step 3 — Deploy to WeilChain (Asia Pod)
```bash
# Get Asia Pod weilpod_id from marauder.weilliptic.ai
wcli deploy applet/pkg/weilguard_bg.wasm -w <asia_pod_id>

# The CLI returns your contract address, e.g.:
# Deployed: weilguard.weil (contract_id)
```

### Step 4 — Run the MCP Server
```bash
cd mcp
npm install
npm run build

# Set the deployed contract address
export WEILGUARD_CONTRACT_ID="weilguard.weil"
export WEIL_ENDPOINT="https://asia-pod.weilliptic.ai"

npm start
```

### Step 5 — Test via Icarus
1. Open Icarus in VS Code (Weilliptic extension)
2. Add the MCP server: point to the running `weilguard-mcp` process
3. In Icarus chat, try:
   > "Register an agent called `email-bot` with policy: allow send_email, deny delete_all"

---

## 🔬 Example Walkthrough

```bash
# 1. Register an agent
wcli call weilguard.weil register_agent \
  --agent_id "data-analyst-01" \
  --policy_json '{"allowed_actions":["read_db","generate_report"],"denied_actions":["drop_table","export_all"]}'

# 2. Pre-check a proposed action
wcli query weilguard.weil check_policy \
  --agent_id "data-analyst-01" \
  --proposed_action "read_db"
# → ALLOW

wcli query weilguard.weil check_policy \
  --agent_id "data-analyst-01" \
  --proposed_action "drop_table"
# → DENY:Action 'drop_table' is explicitly prohibited

# 3. Log an approved action
wcli call weilguard.weil log_event \
  --agent_id "data-analyst-01" \
  --action "read_db" \
  --payload '{"table":"sales_q1","rows_read":1200}'

# 4. Get compliance score
wcli query weilguard.weil compliance_score --agent_id "data-analyst-01"
# → 100

# 5. Simulate a violation
wcli call weilguard.weil log_event \
  --agent_id "data-analyst-01" \
  --action "drop_table" \
  --payload '{"table":"users"}'
# → auto-flagged as CRITICAL violation

wcli query weilguard.weil compliance_score --agent_id "data-analyst-01"
# → 80  (penalty applied)
```

---

## 🌐 Why WeilChain Makes This Possible

Traditional logging can be deleted, altered, or faked. WeilGuard's audit log is:
- **Immutable** — stored in WeilChain's tamper-proof state
- **Cryptographically verified** — each event has a deterministic SHA-256 ID
- **Globally observable** — any node can verify the execution history
- **Replayable** — Cerebrum can replay and audit every decision

This directly leverages WeilChain's core promise: *provable trust, verifiable execution*.

---

## 👥 Team
- [Ajit Sharma] - Leader
- [Ram Gangul]
- [Atharva Sheshgiri]
- [Namika Sharma]

## 📄 License
MIT
