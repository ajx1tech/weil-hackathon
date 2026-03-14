# 🚀 GitHub Submission Guide — WeilGuard
## Xpecto Weilliptic Hackathon 2026

---

## PART 1 — GitHub Repository Setup & PR Submission

### Step 1: Fork the Hackathon Repository
1. Go to: https://github.com/weilliptic-public/weil-hackathon
2. Click **Fork** (top-right)
3. Fork to YOUR personal GitHub account
4. You now have: `https://github.com/YOUR_USERNAME/weil-hackathon`

---

### Step 2: Clone Your Fork Locally
```bash
git clone https://github.com/YOUR_USERNAME/weil-hackathon.git
cd weil-hackathon
```

---

### Step 3: Create Your Team Folder
The README says to use your team name as the folder. Example: if your team is "TeamWeilGuard":

```bash
mkdir TeamWeilGuard
```

---

### Step 4: Copy All WeilGuard Files Into the Folder
Copy the entire WeilGuard project into your team folder:

```
weil-hackathon/
└── TeamWeilGuard/
    ├── README.md
    ├── weilguard.widl
    ├── applet/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── bindings.rs
    │       └── lib.rs
    ├── mcp/
    │   ├── package.json
    │   ├── tsconfig.json
    │   └── src/
    │       └── index.ts
    └── docs/
        ├── demo_script.md
        ├── architecture.html
        └── example_policies.json
```

---

### Step 5: Commit and Push
```bash
git add TeamWeilGuard/
git commit -m "Submission #TeamWeilGuard - WeilGuard: On-Chain AI Agent Security"
git push origin main
```

---

### Step 6: Create the Pull Request
1. Go to your fork: `https://github.com/YOUR_USERNAME/weil-hackathon`
2. Click **"Contribute"** → **"Open pull request"**
3. Set:
   - **Base repository**: `weilliptic-public/weil-hackathon`  
   - **Base**: `main`
   - **Head**: your fork → `main`
4. **PR Title** (EXACT format required):
   ```
   Submission #TeamWeilGuard
   ```
5. **PR Description**: Copy-paste everything from `PR_DESCRIPTION.md`
6. Click **Create Pull Request**
7. 📋 **Copy the PR URL** — you'll need this for the Google Form

---

## PART 2 — Google Form Submission

Form URL: https://forms.gle/Ai99SdGHX6BWSvaA8

Fill in the following:

| Field | Your Answer |
|---|---|
| **Team Name** | TeamWeilGuard (or your actual team name) |
| **Project Name** | WeilGuard |
| **PR Link** | https://github.com/weilliptic-public/weil-hackathon/pull/[YOUR_PR_NUMBER] |
| **Short Description** | On-chain AI agent security, policy enforcement & compliance monitoring on WeilChain |
| **GitHub Repo Link** | https://github.com/YOUR_USERNAME/weil-hackathon/tree/main/TeamWeilGuard |
| **Demo Video** | [Your Loom / YouTube link — record the demo_script.md walkthrough] |
| **Technology Used** | WeilChain, WIDL, Rust/WASM Applet, MCP Server, Icarus, TypeScript |

---

## PART 3 — Optional Demo Video (Recommended)

Record an 8-minute video following `docs/demo_script.md`:

**Free screen recording tools:**
- Loom (https://loom.com) — free, instant share link
- OBS Studio — free open source
- Windows: Win+G Game Bar
- Mac: Cmd+Shift+5

**What to show:**
1. The `weilguard.widl` file in VS Code
2. The Rust `lib.rs` policy engine
3. Icarus MCP demo (steps in demo_script.md)
4. Architecture diagram (`docs/architecture.html`)

Upload to YouTube (unlisted) or Loom and paste link in the form.

---

## PART 4 — Checklist Before Submitting

- [ ] Forked `weilliptic-public/weil-hackathon`
- [ ] Created `TeamWeilGuard/` folder (use your real team name)
- [ ] Added all project files inside the folder
- [ ] Committed with message matching the required format
- [ ] PR title is exactly: `Submission #TeamWeilGuard`
- [ ] PR description contains project summary + run instructions
- [ ] Copied PR URL
- [ ] Filled Google Form at https://forms.gle/Ai99SdGHX6BWSvaA8
- [ ] Added demo video link (recommended)
- [ ] Submitted before 14th EOD ✅

---

## PART 5 — Quick Commands Reference

```bash
# All at once — copy-paste this block

git clone https://github.com/YOUR_USERNAME/weil-hackathon.git
cd weil-hackathon
mkdir TeamWeilGuard

# [copy all WeilGuard files into TeamWeilGuard/]

git add .
git commit -m "Submission #TeamWeilGuard - WeilGuard On-Chain AI Agent Security"
git push origin main

# Then go to GitHub and open the PR manually
```
