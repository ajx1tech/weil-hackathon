# Submission #WeilGuard

## Short Description

**WeilGuard** is an on-chain AI agent security, policy enforcement, and compliance monitoring platform built on WeilChain.

It solves a critical problem in enterprise AI: autonomous agents (email bots, data analysts, deployment pipelines) have **zero accountability** — when they act, there is no tamper-proof record of what they did, whether they were authorised, or who approved it.

WeilGuard provides:
- A **pre-execution policy gate** (`check_policy`) — agents ask "am I allowed?" BEFORE acting
- An **immutable audit log** on WeilChain — every action gets a SHA-256 event ID
- A **compliance score** (0–100) calculated from real-time violation history
- A **kill-switch** (`revoke_agent`) to immediately deactivate rogue agents
- A **9-tool MCP server** that connects the above to any LLM via Icarus

Built using: **WIDL → Rust/WASM Applet → WeilChain (Asia Pod) → TypeScript MCP Server → Icarus**

---

## How to Run / Test

### 1. Deploy the Applet

```bash
# Install wcli from https://github.com/weilliptic-public/wadk/releases

# Generate bindings from WIDL
./widlc WeilGuard/weilguard.widl --lang rust --out WeilGuard/applet/src/bindings.rs

# Build the WASM applet
cd WeilGuard/applet
cargo build --release --target wasm32-unknown-unknown

# Deploy to Asia Pod (get pod ID from marauder.weilliptic.ai)
wcli deploy pkg/weilguard_bg.wasm -w <asia_pod_id>
```

### 2. Start the MCP Server

```bash
cd WeilGuard/mcp
npm install && npm run build

export WEILGUARD_CONTRACT_ID="weilguard.weil"
export WEIL_ENDPOINT="https://asia-pod.weilliptic.ai"
npm start
```

### 3. Test via Icarus

1. Open Icarus (Weilliptic VS Code extension)
2. Add MCP server pointing to the running weilguard-mcp process
3. Try in chat:
   - *"Register agent data-analyst-01 with policy: allow read_db and generate_report, deny drop_table"*
   - *"Check if data-analyst-01 can perform drop_table"* → returns `DENY:...`
   - *"Log that data-analyst-01 ran read_db on sales table"* → returns SHA-256 event ID
   - *"What is data-analyst-01's compliance score?"* → returns 100
   - *"Get the full audit log for data-analyst-01"*

### 4. Direct CLI testing (no LLM needed)

```bash
wcli query weilguard.weil check_policy \
  --agent_id "data-analyst-01" \
  --proposed_action "drop_table"
# → DENY:Action 'drop_table' is explicitly prohibited

wcli query weilguard.weil compliance_score --agent_id "data-analyst-01"
# → 100
```

---

## Files Submitted

```
WeilGuard/
├── README.md                  ← Full documentation
├── weilguard.widl             ← WIDL interface (9 methods)
├── applet/
│   ├── Cargo.toml
│   └── src/
│       ├── bindings.rs        ← Generated skeleton
│       └── lib.rs             ← Full policy engine + audit logic
├── mcp/
│   ├── package.json
│   ├── tsconfig.json
│   └── src/index.ts           ← MCP server (9 tools)
└── docs/
    ├── demo_script.md         ← 8-min judge walkthrough
    ├── architecture.html      ← Visual architecture diagram
    └── example_policies.json  ← 4 example governance policies
```
