// lib.rs — WeilGuard Applet
// On-Chain AI Agent Security & Compliance Monitor
// Deployed on WeilChain | Xpecto Weilliptic Hackathon 2026

mod bindings;

use bindings::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use weil_macros::{constructor, mutate, query, smart_contract};
use weil_sdk::env;

#[smart_contract]
impl WeilGuard for WeilGuardContractState {
    // ─── Constructor ────────────────────────────────────────────────────────
    #[constructor]
    fn new() -> Result<Self, String> {
        Ok(WeilGuardContractState {
            agents:     weil_sdk::collections::WeilMap::new("agents"),
            audit_logs: weil_sdk::collections::WeilMap::new("audit_logs"),
            violations: weil_sdk::collections::WeilMap::new("violations"),
        })
    }

    // ─── Register Agent ─────────────────────────────────────────────────────
    #[mutate]
    async fn register_agent(
        &mut self,
        agent_id: String,
        policy_json: String,
    ) -> Result<String, String> {
        // Validate policy JSON
        let _: Value = serde_json::from_str(&policy_json)
            .map_err(|_| "Invalid policy JSON".to_string())?;

        if self.agents.contains(&agent_id) {
            return Err(format!("Agent '{}' is already registered", agent_id));
        }

        let record = AgentRecord {
            agent_id:      agent_id.clone(),
            policy_json,
            active:        true,
            registered_at: env::block_timestamp(),
        };

        self.agents.insert(agent_id.clone(), record);
        // Initialise empty logs
        self.audit_logs.insert(agent_id.clone(), "[]".to_string());
        self.violations.insert(agent_id.clone(), "[]".to_string());

        Ok(agent_id)
    }

    // ─── Log Event ──────────────────────────────────────────────────────────
    #[mutate]
    async fn log_event(
        &mut self,
        agent_id: String,
        action: String,
        payload: String,
    ) -> Result<String, String> {
        self.assert_agent_active(&agent_id)?;

        let ts = env::block_timestamp();

        // Build a deterministic event_id = sha256(agent_id+action+payload+ts)
        let raw = format!("{}:{}:{}:{}", agent_id, action, payload, ts);
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let event_id = hex::encode(hasher.finalize());

        let event = AuditEvent {
            event_id:  event_id.clone(),
            agent_id:  agent_id.clone(),
            action:    action.clone(),
            payload,
            timestamp: ts,
        };

        // Append to existing log
        let log_json = self.audit_logs.get(&agent_id).unwrap_or("[]".to_string());
        let mut log: Vec<Value> = serde_json::from_str(&log_json).unwrap_or_default();
        log.push(json!(event));
        self.audit_logs.insert(agent_id.clone(), serde_json::to_string(&log).unwrap());

        // Auto-check policy — flag if action is prohibited
        let verdict = self.check_policy_internal(&agent_id, &action);
        if verdict.starts_with("DENY") {
            let _ = self
                .flag_violation(
                    agent_id.clone(),
                    format!("Policy breach: {}", verdict),
                    "HIGH".to_string(),
                )
                .await;
        }

        Ok(event_id)
    }

    // ─── Check Policy (read-only) ────────────────────────────────────────────
    #[query]
    async fn check_policy(&self, agent_id: String, proposed_action: String) -> String {
        self.check_policy_internal(&agent_id, &proposed_action)
    }

    // ─── Get Audit Log ──────────────────────────────────────────────────────
    #[query]
    async fn get_audit_log(&self, agent_id: String) -> String {
        self.audit_logs.get(&agent_id).unwrap_or("[]".to_string())
    }

    // ─── Flag Violation ─────────────────────────────────────────────────────
    #[mutate]
    async fn flag_violation(
        &mut self,
        agent_id: String,
        reason: String,
        severity: String,
    ) -> Result<String, String> {
        self.assert_agent_active(&agent_id)?;

        let ts = env::block_timestamp();
        let raw = format!("{}:{}:{}", agent_id, reason, ts);
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let violation_id = hex::encode(hasher.finalize());

        let v = Violation {
            violation_id: violation_id.clone(),
            agent_id:     agent_id.clone(),
            reason,
            severity:     severity.to_uppercase(),
            timestamp:    ts,
        };

        let viol_json = self.violations.get(&agent_id).unwrap_or("[]".to_string());
        let mut viols: Vec<Value> = serde_json::from_str(&viol_json).unwrap_or_default();
        viols.push(json!(v));
        self.violations.insert(agent_id, serde_json::to_string(&viols).unwrap());

        Ok(violation_id)
    }

    // ─── Get Violations ──────────────────────────────────────────────────────
    #[query]
    async fn get_violations(&self, agent_id: String) -> String {
        self.violations.get(&agent_id).unwrap_or("[]".to_string())
    }

    // ─── Compliance Score ────────────────────────────────────────────────────
    #[query]
    async fn compliance_score(&self, agent_id: String) -> u32 {
        let viol_json = self.violations.get(&agent_id).unwrap_or("[]".to_string());
        let viols: Vec<Value> = serde_json::from_str(&viol_json).unwrap_or_default();

        let log_json = self.audit_logs.get(&agent_id).unwrap_or("[]".to_string());
        let logs: Vec<Value> = serde_json::from_str(&log_json).unwrap_or_default();

        if logs.is_empty() {
            return 100;
        }

        // Penalty: CRITICAL=20, HIGH=10, MEDIUM=5, LOW=2
        let penalty: u32 = viols
            .iter()
            .map(|v| {
                let sev = v["severity"].as_str().unwrap_or("LOW");
                match sev {
                    "CRITICAL" => 20,
                    "HIGH"     => 10,
                    "MEDIUM"   => 5,
                    _          => 2,
                }
            })
            .sum();

        100u32.saturating_sub(penalty)
    }

    // ─── Revoke Agent ────────────────────────────────────────────────────────
    #[mutate]
    async fn revoke_agent(&mut self, agent_id: String, reason: String) -> Result<bool, String> {
        let mut record = self
            .agents
            .get(&agent_id)
            .ok_or_else(|| format!("Unknown agent '{}'", agent_id))?;

        record.active = false;
        self.agents.insert(agent_id.clone(), record);

        // Log the revocation itself
        let _ = self
            .flag_violation(agent_id, reason, "CRITICAL".to_string())
            .await;

        Ok(true)
    }

    // ─── Update Policy ───────────────────────────────────────────────────────
    #[mutate]
    async fn update_policy(
        &mut self,
        agent_id: String,
        new_policy_json: String,
    ) -> Result<bool, String> {
        let _: Value = serde_json::from_str(&new_policy_json)
            .map_err(|_| "Invalid policy JSON".to_string())?;

        let mut record = self
            .agents
            .get(&agent_id)
            .ok_or_else(|| format!("Unknown agent '{}'", agent_id))?;

        record.policy_json = new_policy_json;
        self.agents.insert(agent_id, record);
        Ok(true)
    }
}

// ─── Helper Methods ─────────────────────────────────────────────────────────
impl WeilGuardContractState {
    fn assert_agent_active(&self, agent_id: &str) -> Result<(), String> {
        let record = self
            .agents
            .get(&agent_id.to_string())
            .ok_or_else(|| format!("Agent '{}' not registered", agent_id))?;
        if !record.active {
            return Err(format!("Agent '{}' has been revoked", agent_id));
        }
        Ok(())
    }

    /// Core policy engine: evaluates proposed_action against the agent's policy JSON.
    /// Policy JSON schema:
    /// {
    ///   "allowed_actions": ["read_db", "send_email", ...],
    ///   "denied_actions":  ["delete_all", "escalate_privileges", ...],
    ///   "require_human_approval": ["deploy_contract", ...]
    /// }
    fn check_policy_internal(&self, agent_id: &str, proposed_action: &str) -> String {
        let record = match self.agents.get(&agent_id.to_string()) {
            Some(r) => r,
            None    => return format!("DENY:Agent '{}' not found", agent_id),
        };
        if !record.active {
            return format!("DENY:Agent '{}' is revoked", agent_id);
        }

        let policy: Value = match serde_json::from_str(&record.policy_json) {
            Ok(v)  => v,
            Err(_) => return "DENY:Malformed policy".to_string(),
        };

        // Explicit deny list — checked first
        if let Some(denied) = policy["denied_actions"].as_array() {
            for a in denied {
                if a.as_str() == Some(proposed_action) {
                    return format!("DENY:Action '{}' is explicitly prohibited", proposed_action);
                }
            }
        }

        // Human-approval list
        if let Some(approval_required) = policy["require_human_approval"].as_array() {
            for a in approval_required {
                if a.as_str() == Some(proposed_action) {
                    return format!(
                        "DENY:Action '{}' requires human approval before execution",
                        proposed_action
                    );
                }
            }
        }

        // Allowed list — if present, only listed actions pass
        if let Some(allowed) = policy["allowed_actions"].as_array() {
            if !allowed.is_empty() {
                let ok = allowed.iter().any(|a| a.as_str() == Some(proposed_action));
                if ok {
                    return "ALLOW".to_string();
                } else {
                    return format!(
                        "DENY:Action '{}' not in allowed list",
                        proposed_action
                    );
                }
            }
        }

        "ALLOW".to_string()
    }
}
