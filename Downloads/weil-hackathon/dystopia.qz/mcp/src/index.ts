/**
 * WeilGuard MCP Server
 * Connects Icarus / any LLM agent to the WeilGuard on-chain applet.
 *
 * Tools exposed:
 *  - register_agent      — register a new AI agent with a governance policy
 *  - log_event           — record an agent action on-chain (immutable)
 *  - check_policy        — evaluate a proposed action BEFORE execution
 *  - get_audit_log       — retrieve the full cryptographic audit trail
 *  - flag_violation      — manually raise a compliance violation
 *  - get_violations      — list all violations for an agent
 *  - compliance_score    — get the current compliance score (0–100)
 *  - revoke_agent        — permanently deactivate a misbehaving agent
 *  - update_policy       — update an agent's governance policy
 */

import { Server }   from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";

// WeilChain SDK — from @weilliptic/weil-sdk
// The contract_id below must be set to the deployed WeilGuard applet address.
const CONTRACT_ID   = process.env.WEILGUARD_CONTRACT_ID ?? "weilguard.weil";
const WEIL_ENDPOINT = process.env.WEIL_ENDPOINT         ?? "https://asia-pod.weilliptic.ai";

// ─── Thin WeilChain client helper ────────────────────────────────────────────
async function callApplet(
  method: string,
  args: Record<string, unknown>,
  isQuery = false
): Promise<unknown> {
  const { WeilClient } = await import("@weilliptic/weil-sdk");
  const client = new WeilClient({ endpoint: WEIL_ENDPOINT });

  if (isQuery) {
    return client.query(CONTRACT_ID, method, args);
  } else {
    return client.mutate(CONTRACT_ID, method, args);
  }
}

// ─── Tool definitions ────────────────────────────────────────────────────────
const TOOLS = [
  {
    name:        "register_agent",
    description: "Register a new AI agent on WeilChain with its governance policy.",
    inputSchema: {
      type:       "object",
      properties: {
        agent_id:    { type: "string", description: "Unique agent identifier" },
        policy_json: {
          type:        "string",
          description: `JSON governance policy with keys:
            allowed_actions (string[]),
            denied_actions (string[]),
            require_human_approval (string[])`,
        },
      },
      required: ["agent_id", "policy_json"],
    },
  },
  {
    name:        "log_event",
    description: "Record an agent action on-chain. Returns a tamper-proof event_id (sha256 hash). Automatically checks policy and flags violations.",
    inputSchema: {
      type:       "object",
      properties: {
        agent_id: { type: "string" },
        action:   { type: "string", description: "Action key, e.g. 'send_email', 'read_db'" },
        payload:  { type: "string", description: "Stringified details / context of the action" },
      },
      required: ["agent_id", "action", "payload"],
    },
  },
  {
    name:        "check_policy",
    description: "Check whether a proposed action is allowed by the agent's policy BEFORE executing it. Returns 'ALLOW' or 'DENY:<reason>'.",
    inputSchema: {
      type:       "object",
      properties: {
        agent_id:        { type: "string" },
        proposed_action: { type: "string" },
      },
      required: ["agent_id", "proposed_action"],
    },
  },
  {
    name:        "get_audit_log",
    description: "Retrieve the complete, cryptographically-signed audit trail for an agent (JSON array).",
    inputSchema: {
      type:       "object",
      properties: { agent_id: { type: "string" } },
      required:   ["agent_id"],
    },
  },
  {
    name:        "flag_violation",
    description: "Manually flag a compliance violation for an agent.",
    inputSchema: {
      type:       "object",
      properties: {
        agent_id: { type: "string" },
        reason:   { type: "string" },
        severity: {
          type: "string",
          enum: ["LOW", "MEDIUM", "HIGH", "CRITICAL"],
        },
      },
      required: ["agent_id", "reason", "severity"],
    },
  },
  {
    name:        "get_violations",
    description: "Retrieve all compliance violations for an agent.",
    inputSchema: {
      type:       "object",
      properties: { agent_id: { type: "string" } },
      required:   ["agent_id"],
    },
  },
  {
    name:        "compliance_score",
    description: "Get the current compliance score (0–100) for an agent. 100 = fully compliant, penalties for violations.",
    inputSchema: {
      type:       "object",
      properties: { agent_id: { type: "string" } },
      required:   ["agent_id"],
    },
  },
  {
    name:        "revoke_agent",
    description: "Permanently deactivate a misbehaving agent. All future actions will be blocked.",
    inputSchema: {
      type:       "object",
      properties: {
        agent_id: { type: "string" },
        reason:   { type: "string" },
      },
      required: ["agent_id", "reason"],
    },
  },
  {
    name:        "update_policy",
    description: "Update the governance policy for a registered agent.",
    inputSchema: {
      type:       "object",
      properties: {
        agent_id:        { type: "string" },
        new_policy_json: { type: "string" },
      },
      required: ["agent_id", "new_policy_json"],
    },
  },
];

// ─── MCP Server ──────────────────────────────────────────────────────────────
const server = new Server(
  { name: "weilguard-mcp", version: "0.1.0" },
  { capabilities: { tools: {} } }
);

server.setRequestHandler(ListToolsRequestSchema, async () => ({ tools: TOOLS }));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args = {} } = request.params;

  try {
    let result: unknown;

    switch (name) {
      // ── queries (read-only) ──
      case "check_policy":
        result = await callApplet("check_policy", args, true);
        break;
      case "get_audit_log":
        result = await callApplet("get_audit_log", args, true);
        break;
      case "get_violations":
        result = await callApplet("get_violations", args, true);
        break;
      case "compliance_score":
        result = await callApplet("compliance_score", args, true);
        break;

      // ── mutates (write to chain) ──
      case "register_agent":
        result = await callApplet("register_agent", args);
        break;
      case "log_event":
        result = await callApplet("log_event", args);
        break;
      case "flag_violation":
        result = await callApplet("flag_violation", args);
        break;
      case "revoke_agent":
        result = await callApplet("revoke_agent", args);
        break;
      case "update_policy":
        result = await callApplet("update_policy", args);
        break;

      default:
        return {
          content: [{ type: "text", text: `Unknown tool: ${name}` }],
          isError: true,
        };
    }

    return {
      content: [
        {
          type: "text",
          text: typeof result === "string" ? result : JSON.stringify(result, null, 2),
        },
      ],
    };
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    return {
      content: [{ type: "text", text: `WeilGuard error: ${msg}` }],
      isError: true,
    };
  }
});

// ─── Start ───────────────────────────────────────────────────────────────────
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("WeilGuard MCP server running (stdio transport)");
}

main().catch(console.error);
