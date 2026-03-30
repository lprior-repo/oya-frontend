# OYA-FRONTEND Quality Gate Instructions

```jsonl
{"kind":"meta","project":"oya-frontend","version":"1.0.0","updated":"2026-02","format":"markdown-with-embedded-jsonl"}
{"kind":"visibility","role":"agent","can_see":["specs/flow-wasm-v1.yaml","tests/","src/","specs/linter/rules.yaml","ACCEPTANCE CRITERIA"],"cannot_see":["../scenarios-vault/","holdout assertions","step sequences","raw validation results"]}
{"kind":"framework","name":"dioxus","version":"0.7","reactivity":"use_signal, use_memo, use_resource","state_hierarchy":"use_signal (atomic), use_store + #[derive(Store)] (nested)","props":"ReadOnlySignal<T>","styling":"Tailwind class: attribute","events":"move |_| closures","architecture":"flat RSX, #[component] modular","assets":"asset!() macro"}
{"kind":"code_intelligence","tool":"codanna","required":"mcp__codanna__*","forbidden":["grep","rg","ripgrep","find","cat for search"],"workflow":["semantic_search_with_context → anchor","find_symbol/search_symbols → lock","get_calls/find_callers/analyze_impact → hints","Read tool → confirm"]}
{"kind":"build_system","tool":"moon","allowed":[":serve",":check",":test",":clippy",":fmt",":build-web",":ci --force"],"forbidden":["dx directly","cargo directly","npm directly"]}
{"kind":"quality_gate","approach":"hidden behavioral scenarios","feedback":"sanitized (category, spec reference, hints, no exact values)","purpose":"prevent teaching to test"}
{"kind":"invariants","rules":["never access ../scenarios-vault/","never ask about holdout scenarios","build genuine spec implementation","all acceptance criteria satisfied"]}
{"kind":"directive","text":"Use Dioxus 0.7 Signals and Stores. Never use use_state.","reason":"prevents legacy API, ensures reactivity"}
{"kind":"directive","text":"Use ReadOnlySignal<T> for component props","reason":"proper prop typing, reactive propagation"}
{"kind":"directive","text":"Use use_store with #[derive(Store)] for nested state","reason":"fine-grained reactivity for complex structures"}
{"kind":"directive","text":"Keep RSX trees flat — modularize into components","reason":"maintainability, hot-reloading"}
{"kind":"directive","text":"Use Tailwind utility classes in class: attributes","reason":"standardized styling, rapid prototyping"}
{"kind":"directive","text":"Handle events with move |_| closures","reason":"Signals are Copy, cloning unnecessary"}
{"kind":"directive","text":"Act as v0 for Dioxus","reason":"modern, accessible, perfectly-styled RSX"}
{"kind":"tool","name":"semantic_search_with_context","purpose":"Search by concept with full context","when":"Start here for exploration"}
{"kind":"tool","name":"find_symbol","purpose":"Exact symbol lookup","when":"Know the exact name"}
{"kind":"tool","name":"search_symbols","purpose":"Fuzzy text search","when":"Partial name matches"}
{"kind":"tool","name":"get_calls","purpose":"What function calls","when":"Call graph outbound"}
{"kind":"tool","name":"find_callers","purpose":"What calls function","when":"Call graph inbound"}
{"kind":"tool","name":"analyze_impact","purpose":"Full dependency graph","when":"Change impact analysis"}
{"kind":"tool","name":"search_documents","purpose":"Search markdown/docs","when":"Find project docs"}
{"kind":"tool","name":"get_index_info","purpose":"Index statistics","when":"Verify index health"}
```

## Quick Reference

| Area | Rule |
|------|------|
| **Workspace** | specs, tests, src visible; scenarios-vault hidden |
| **State** | `use_signal` / `use_store`, NEVER `use_state` |
| **Build** | Moon only, `moon run :ci --force` |
| **Code Search** | Codanna MCP tools ONLY |
| **Quality** | Hidden scenarios, sanitized feedback |

<!-- BEGIN BEADS INTEGRATION v:1 profile:full hash:d4f96305 -->
## Issue Tracking with bd (beads)

**IMPORTANT**: This project uses **bd (beads)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why bd?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: Dolt-powered version control with native sync
- Agent-optimized: JSON output, ready work detection, discovered-from links
- Prevents duplicate tracking systems and confusion

### Quick Start

**Check for ready work:**

```bash
bd ready --json
```

**Create new issues:**

```bash
bd create "Issue title" --description="Detailed context" -t bug|feature|task -p 0-4 --json
bd create "Issue title" --description="What this issue is about" -p 1 --deps discovered-from:bd-123 --json
```

**Claim and update:**

```bash
bd update <id> --claim --json
bd update bd-42 --priority 1 --json
```

**Complete work:**

```bash
bd close bd-42 --reason "Completed" --json
```

### Issue Types

- `bug` - Something broken
- `feature` - New functionality
- `task` - Work item (tests, docs, refactoring)
- `epic` - Large feature with subtasks
- `chore` - Maintenance (dependencies, tooling)

### Priorities

- `0` - Critical (security, data loss, broken builds)
- `1` - High (major features, important bugs)
- `2` - Medium (default, nice-to-have)
- `3` - Low (polish, optimization)
- `4` - Backlog (future ideas)

### Workflow for AI Agents

1. **Check ready work**: `bd ready` shows unblocked issues
2. **Claim your task atomically**: `bd update <id> --claim`
3. **Work on it**: Implement, test, document
4. **Discover new work?** Create linked issue:
   - `bd create "Found bug" --description="Details about what was found" -p 1 --deps discovered-from:<parent-id>`
5. **Complete**: `bd close <id> --reason "Done"`

### Auto-Sync

bd automatically syncs via Dolt:

- Each write auto-commits to Dolt history
- Use `bd dolt push`/`bd dolt pull` for remote sync
- No manual export/import needed!

### Important Rules

- ✅ Use bd for ALL task tracking
- ✅ Always use `--json` flag for programmatic use
- ✅ Link discovered work with `discovered-from` dependencies
- ✅ Check `bd ready` before asking "what should I work on?"
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT use external issue trackers
- ❌ Do NOT duplicate tracking systems

For more details, see README.md and docs/QUICKSTART.md.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd dolt push
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

<!-- END BEADS INTEGRATION -->
