USE `oya_frontend`;
START TRANSACTION;
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-117', 'ui: Fix ParallelGroup field name mismatches in edges.rs', 'In src/ui/edges.rs, the code references non-existent fields on ParallelGroup struct:

Code uses (wrong):        Struct has (correct):
- group.source_node        - group.parallel_node_id
- group.target_nodes       - group.branch_node_ids
- group.bounds             - group.bounding_box

Evidence:
- edges.rs:117 references group.source_node
- edges.rs:302, 307, 897 reference group.target_nodes
- edges.rs:308-311, 1184-1194 reference group.bounds

But ParallelGroup struct (parallel_group_overlay.rs:15-21) has:
- parallel_node_id
- branch_node_ids
- bounding_box

This causes compilation errors:
- error[E0560]: struct ParallelGroup has no field named source_node
- error[E0560]: struct ParallelGroup has no field named target_nodes
- error[E0609]: no field bounds on type

Fix: Update all references in edges.rs to use correct field names.', 'open', 1, 'bug', '2026-03-05T11:47:30.945529600Z', 'lewis', '2026-03-05T11:47:30.945529600Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-17v', 'ci-gate: restore clippy-compliant build in graph modules', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260220074859-peslrkrm.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260220074859-peslrkrm.cue


#EnhancedBead: {
  id: "oya-frontend-20260220074859-peslrkrm"
  title: "ci-gate: restore clippy-compliant build in graph modules"
  type: "bug"
  priority: 0
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL keep :clippy and :ci green on every commit.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN moon run :clippy is executed\\", shall: \\"THE SYSTEM SHALL exit with code 0 and no denied lint violations.\\"}
    ]
    unwanted: [
      {condition: \\"IF graph module lint violations exist\\", shall_not: \\"THE SYSTEM SHALL NOT pass CI gating\\", because: \\"it permits regressions and prevents predictable release flow.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Repository is in a compilable state under moon tasks.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"moon run :clippy exits 0.\\",
        \\"moon run :ci --force exits 0 without lint failures.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No #[allow] suppression added for the reported clippy denies.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/graph/layout.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"What refactor split preserves layout determinism while satisfying line-count lint?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Inspect exact clippy diagnostics and hotspots.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add or adjust regression assertions for fit_view behavior if needed.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Refactor apply into helper functions and move constants to module scope.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260220074859-peslrkrm/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/layout.rs:23\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/mod.rs:422\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/mod.rs:423\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"tests/graph_regressions.rs\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 0, 'bug', '2026-02-20T13:48:59.810360775Z', 'lewis', '2026-02-22T05:20:27.160443829Z', '2026-02-22T05:20:27.160428220Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-184', 'engine: Async IO Bridge', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-0c1bci7j.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-0c1bci7j.cue


#EnhancedBead: {
  id: "new-app-20260219105843-0c1bci7j"
  title: "engine: Async IO Bridge"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL execute network requests asynchronously.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN an HTTP node executes\\", shall: \\"THE SYSTEM SHALL hit the configured URL.\\"}
    ]
    unwanted: [
      {condition: \\"IF a request fails\\", shall_not: \\"THE SYSTEM SHALL NOT crash\\", because: \\"errors are captured\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow::step is synchronous\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Workflow::step is asynchronous\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Network state is tracked\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219105843-0c1bci7j/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T16:58:43.754504876Z', 'lewis', '2026-02-19T17:10:01.586572604Z', '2026-02-19T17:10:01.586563284Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-18z', 'ui: Global Node Search and CMD+K interface', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-xalysero.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-xalysero.cue


#EnhancedBead: {
  id: "new-app-20260219080116-xalysero"
  title: "ui: Global Node Search and CMD+K interface"
  type: "feature"
  priority: 3
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL provide fuzzy search for 400+ potential node types.\\",
      \\"THE SYSTEM SHALL allow adding nodes via keyboard search.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN CMD+K is pressed\\", shall: \\"THE SYSTEM SHALL open the global search modal.\\"}
    ]
    unwanted: [
      {condition: \\"IF no results found\\", shall_not: \\"THE SYSTEM SHALL NOT show a blank screen\\", because: \\"a ''Request Node'' link should be shown\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Node library metadata loaded\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Selected node dropped at mouse position\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Search index is always available\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219080116-xalysero/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 3, 'feature', '2026-02-19T14:01:17.265823263Z', 'lewis', '2026-02-19T19:17:25.852843041Z', '2026-02-19T19:17:25.852837441Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1ao', 'restate_client: stuck_invocations accepts negative timestamp', 'The stuck_invocations function accepts negative i64 values for cutoff_epoch_ms but produces semantically incorrect SQL.', 'open', 0, 'bug', '2026-03-05T12:11:55.758420576Z', 'lewis', '2026-03-05T12:11:55.758420576Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1bv', 'flow-extender: Add idempotent apply semantics with extension fingerprints', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-mxl3j7li.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-mxl3j7li.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-mxl3j7li"
  title: "flow-extender: Add idempotent apply semantics with extension fingerprints"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Add idempotent apply semantics with extension fingerprints with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Add idempotent apply semantics with extension fingerprints has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/execution.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-004.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Add idempotent apply semantics with extension fingerprints behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-mxl3j7li/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/execution.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:01.953913785Z', 'lewis', '2026-02-22T05:16:06.986949769Z', '2026-02-22T05:16:06.986924669Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `assignee`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1c8', 'types: Add ExecutionState enum for node runtime status', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-slzkflku.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-slzkflku.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-slzkflku"
  title: "types: Add ExecutionState enum for node runtime status"
  type: "feature"
  priority: 1
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL track execution state for every node\\",
      \\"THE SYSTEM SHALL provide state transitions that match workflow execution semantics\\"
    ]
    event_driven: [
      {trigger: \\"WHEN workflow execution starts\\", shall: \\"THE SYSTEM SHALL set first node to Running\\"},
      {trigger: \\"WHEN node execution completes\\", shall: \\"THE SYSTEM SHALL transition to Succeeded or Failed\\"}
    ]
    unwanted: [
      {condition: \\"IF execution state is invalid transition\\", shall_not: \\"THE SYSTEM SHALL NOT apply invalid state\\", because: \\"maintains execution state machine integrity\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Node struct exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"ExecutionState enum with 6 states\\",
        \\"Node has execution_state field\\",
        \\"State machine transitions defined\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"State transitions are deterministic\\",
      \\"Failed nodes show error message\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement ExecutionState enum\\", done_when: \\"Tests pass\\"},
        {task: \\"Add execution_state to Node\\", done_when: \\"Tests pass\\"},
        {task: \\"Implement state transition methods\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-slzkflku/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/execution.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"AWS Step Functions execution status\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', 'self', '2026-02-22T14:33:40.557177245Z', 'lewis', '2026-03-05T11:46:25.824135231Z', '2026-03-05T11:46:25.823669725Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1cl', 'run_code: Language draft not synced when config changes', 'Bug: Language draft initialization only runs once on mount

Location: src/ui/workflow_nodes/run_code.rs lines 54-73

Issue:
The drafts signal is initialized from initial_config only ONCE when the component mounts. 
If the config signal changes externally (e.g., loading a saved workflow), the drafts 
wont update, causing stale drafts to be shown when switching languages.

Evidence:
The use_signal with move closure only runs once at component mount time.

Expected Behavior:
When config.signal changes externally, drafts should synchronize to reflect the new config.

Fix Approach:
Add a use_effect to watch config changes and update drafts accordingly', 'open', 0, 'bug', '2026-03-05T11:43:02.265834202Z', 'lewis', '2026-03-05T11:43:02.265834202Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1cu', 'engine: Robustness (Cycles & Timeouts)', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219114321-ql99gohc.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219114321-ql99gohc.cue


#EnhancedBead: {
  id: "new-app-20260219114321-ql99gohc"
  title: "engine: Robustness (Cycles & Timeouts)"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL detect cyclic dependencies during preparation.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN execution exceeds 30 seconds\\", shall: \\"THE SYSTEM SHALL terminate the run and report a timeout.\\"}
    ]
    unwanted: [
      {condition: \\"IF a node execution fails\\", shall_not: \\"THE SYSTEM SHALL NOT crash the engine\\", because: \\"errors should be isolated to the node\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Execution either completes or times out safely\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"The engine must remain deterministic\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add error field to Node struct\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Update prepare_run with DFS cycle detection\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219114321-ql99gohc/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T17:43:21.211323965Z', 'lewis', '2026-02-19T17:47:03.432170973Z', '2026-02-19T17:47:03.432158123Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1dd', 'flow-extender: Define extension rule registry and typed contracts', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-kqzzk1me.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-kqzzk1me.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-kqzzk1me"
  title: "flow-extender: Define extension rule registry and typed contracts"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Define extension rule registry and typed contracts with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Define extension rule registry and typed contracts has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-001.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Define extension rule registry and typed contracts behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-kqzzk1me/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:01.865503954Z', 'lewis', '2026-02-22T05:16:06.879233741Z', '2026-02-22T05:16:06.879218761Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `description`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1ea', 'queue:ready label', 'tombstone', 2, 'task', '2026-02-20T04:01:29.174403660Z', 'lewis', '2026-02-20T04:01:34.469063510Z', '.', 0, 0, '', '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1eg', 'ui: Add expression autocomplete for config fields', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-zkvtglqu.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-zkvtglqu.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-zkvtglqu"
  title: "ui: Add expression autocomplete for config fields"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL provide autocomplete for expression syntax\\",
      \\"THE SYSTEM SHALL show available variables from preceding nodes\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user types {{ in config field\\", shall: \\"THE SYSTEM SHALL show autocomplete dropdown\\"},
      {trigger: \\"WHEN user selects autocomplete item\\", shall: \\"THE SYSTEM SHALL insert expression at cursor\\"}
    ]
    unwanted: [
      {condition: \\"IF expression references non-existent node\\", shall_not: \\"THE SYSTEM SHALL NOT suggest invalid paths\\", because: \\"prevents runtime errors\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Config fields accept string input\\",
        \\"Workflow has node graph\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Dropdown shows preceding node outputs\\",
        \\"Tab or Enter completes selection\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Only shows nodes that execute before current\\",
      \\"Handles nested object paths\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design autocomplete component\\", done_when: \\"Tests pass\\"},
        {task: \\"Define expression parser\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-zkvtglqu/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/config_panel/\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/expressions.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"N8n expression editor\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 2, 'feature', '2026-02-22T14:33:40.635018829Z', 'lewis', '2026-02-22T14:33:40.635018829Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1ew', 'shell: Version control, snapshots, and JSON portability', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-lz2ml0gh.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-lz2ml0gh.cue


#EnhancedBead: {
  id: "new-app-20260219080116-lz2ml0gh"
  title: "shell: Version control, snapshots, and JSON portability"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL allow exporting workflows as a JSON file.\\",
      \\"THE SYSTEM SHALL maintain a history of the last 10 versions locally.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN ''Import'' is clicked\\", shall: \\"THE SYSTEM SHALL validate the JSON schema before applying state.\\"}
    ]
    unwanted: [
      {condition: \\"IF importing malformed JSON\\", shall_not: \\"THE SYSTEM SHALL NOT overwrite the current workflow\\", because: \\"data loss must be avoided\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"LocalStorage digital twin available\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"JSON schema is compliant\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Version IDs are unique\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219080116-lz2ml0gh/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T14:01:17.357182325Z', 'lewis', '2026-02-22T11:53:43.134429045Z', '2026-02-22T11:53:43.134417495Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1jc', 'core: node graph data structures', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219075335-cup48ef3.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219075335-cup48ef3.cue


#EnhancedBead: {
  id: "new-app-20260219075335-cup48ef3"
  title: "core: node graph data structures"
  type: "feature"
  priority: 1
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL define a Workflow as a collection of Nodes and Connections.\\",
      \\"THE SYSTEM SHALL ensure Node IDs are unique.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN an edge is added\\", shall: \\"THE SYSTEM SHALL verify both nodes exist.\\"}
    ]
    unwanted: [
      {condition: \\"IF a node is deleted\\", shall_not: \\"THE SYSTEM SHALL NOT leave dangling connections\\", because: \\"it breaks graph integrity\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"None\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Graph state is valid\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No dangling edges\\",
      \\"Unique IDs\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Implement Workflow, Node, and Edge structs\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Add pure logic for node/edge mutations\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219075335-cup48ef3/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T13:53:35.775805140Z', 'lewis', '2026-02-19T19:17:25.851695520Z', '2026-02-19T19:17:25.851681550Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1k0', 'types: Add ServiceKind and ContextType enums for Restate alignment', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-uhork9mt.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-uhork9mt.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-uhork9mt"
  title: "types: Add ServiceKind and ContextType enums for Restate alignment"
  type: "feature"
  priority: 0
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL distinguish between Service VirtualObject and Workflow service kinds\\",
      \\"THE SYSTEM SHALL provide context type for each node based on its capabilities\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a durable node is created\\", shall: \\"THE SYSTEM SHALL infer appropriate ServiceKind\\"},
      {trigger: \\"WHEN validating node connections\\", shall: \\"THE SYSTEM SHALL check ServiceKind compatibility\\"}
    ]
    unwanted: [
      {condition: \\"IF state operations used in Stateless Service\\", shall_not: \\"THE SYSTEM SHALL NOT allow invalid combination\\", because: \\"Restate enforces this at runtime\\"},
      {condition: \\"IF promise operations used outside Workflow\\", shall_not: \\"THE SYSTEM SHALL NOT allow promise in non-Workflow context\\", because: \\"only WorkflowContext has promises\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"WorkflowNode enum exists\\",
        \\"graph/mod.rs has Node struct\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"ServiceKind enum with 3 variants\\",
        \\"ContextType enum with 3 variants\\",
        \\"ServiceKind derivable from WorkflowNode\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"ServiceKind matches Restate SDK semantics\\",
      \\"VirtualObject always has key\\",
      \\"Workflow always has promises\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/graph/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"docs/10_RESTATE_SDK.md\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Review Restate SDK service types\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map node types to ServiceKind\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement ServiceKind enum\\", done_when: \\"Tests pass\\"},
        {task: \\"Implement ContextType enum\\", done_when: \\"Tests pass\\"},
        {task: \\"Add inference fn from WorkflowNode\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-uhork9mt/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Restate SDK ServiceOptions\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 0, 'feature', '2026-02-22T14:33:40.539254982Z', 'lewis', '2026-02-22T14:33:40.539254982Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1kr', 'docs: Author flow-extender architecture and operator guide', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-7aracnlh.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-7aracnlh.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-7aracnlh"
  title: "docs: Author flow-extender architecture and operator guide"
  type: "task"
  priority: 3
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete docs: Author flow-extender architecture and operator guide with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"docs: Author flow-extender architecture and operator guide has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"README.md\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"docs/03_WORKFLOW.md\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-020.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify docs: Author flow-extender architecture and operator guide behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-7aracnlh/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"README.md\\", relevance: \\"Related implementation\\"},
      {path: \\"docs/03_WORKFLOW.md\\", relevance: \\"Related implementation\\"},
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 3, 'task', '2026-02-21T21:26:02.501470262Z', 'lewis', '2026-02-22T11:53:31.866592349Z', '2026-02-22T11:53:31.866572729Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1l5', 'validation: Add node connection type checking', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-sh0m8xfj.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-sh0m8xfj.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-sh0m8xfj"
  title: "validation: Add node connection type checking"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL validate connection compatibility\\",
      \\"THE SYSTEM SHALL show visual feedback for incompatible connections\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user attempts connection\\", shall: \\"THE SYSTEM SHALL check type compatibility\\"},
      {trigger: \\"WHEN types are incompatible\\", shall: \\"THE SYSTEM SHALL show warning and allow or reject\\"}
    ]
    unwanted: [
      {condition: \\"IF connection would cause type error at runtime\\", shall_not: \\"THE SYSTEM SHALL NOT silently allow incompatible connection\\", because: \\"prevents confusing runtime failures\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"WorkflowNode variants have input/output types\\",
        \\"Connection struct exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Type compatibility function exists\\",
        \\"Warning shown for mismatches\\",
        \\"Option to allow anyway\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Type checking is best-effort\\",
      \\"Unknown types treated as Any\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Define type system for node inputs/outputs\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement type compatibility checker\\", done_when: \\"Tests pass\\"},
        {task: \\"Define warning UI\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-sh0m8xfj/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/connectivity.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/main.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"TypeScript type checking\\",
      \\"N8n type validation\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 1, 'feature', '2026-02-22T14:33:40.732873750Z', 'lewis', '2026-02-22T14:33:40.732873750Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1m5', 'gate: Scenario Runner Integration', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-ztzkolqy.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-ztzkolqy.cue


#EnhancedBead: {
  id: "new-app-20260219105843-ztzkolqy"
  title: "gate: Scenario Runner Integration"
  type: "task"
  priority: 4
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL run behavioral tests against the UI.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a scenario is run\\", shall: \\"THE SYSTEM SHALL execute the defined actions.\\"}
    ]
    unwanted: [
      {condition: \\"IF a test fails\\", shall_not: \\"THE SYSTEM SHALL NOT report success\\", because: \\"integrity\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Scenario runner is initialized\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Test results are reported\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Test state is clean\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219105843-ztzkolqy/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 4, 'task', '2026-02-19T16:58:43.955198570Z', 'lewis', '2026-02-19T17:10:01.587393627Z', '2026-02-19T17:10:01.587387567Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1mh', 'ui: Property Editor', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-ikk19pdr.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-ikk19pdr.cue


#EnhancedBead: {
  id: "new-app-20260219105843-ikk19pdr"
  title: "ui: Property Editor"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL show structured inputs for node config.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a node is selected\\", shall: \\"THE SYSTEM SHALL display its configuration fields.\\"}
    ]
    unwanted: [
      {condition: \\"IF config is invalid\\", shall_not: \\"THE SYSTEM SHALL NOT crash the UI\\", because: \\"robustness\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Node is selected\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Config fields are visible\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Data binding is consistent\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219105843-ikk19pdr/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T16:58:43.802802429Z', 'lewis', '2026-02-19T17:10:01.586821092Z', '2026-02-19T17:10:01.586813952Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1n8', 'config: apply_config_update silently fails and leaves stale state', 'In src/graph/mod.rs apply_config_update (lines 180-194): 1) Deserialization errors are silently discarded with .ok() 2) Config is updated BEFORE validating the node was successfully updated 3) If type mismatch causes failure, config becomes stale and out of sync with node. This causes state inconsistency.', 'open', 1, 'bug', '2026-03-05T11:48:03.475288803Z', 'lewis', '2026-03-05T11:48:03.475288803Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1nu', 'quality: Add scenario-runner cases for extension behaviors', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-wwuyzpfj.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-wwuyzpfj.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-wwuyzpfj"
  title: "quality: Add scenario-runner cases for extension behaviors"
  type: "task"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete quality: Add scenario-runner cases for extension behaviors with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"quality: Add scenario-runner cases for extension behaviors has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/scenario_runner/runner.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/scenario_runner/types.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"specs/flow-wasm-v1.yaml\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-014.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify quality: Add scenario-runner cases for extension behaviors behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-wwuyzpfj/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/scenario_runner/runner.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/scenario_runner/types.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"specs/flow-wasm-v1.yaml\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'task', '2026-02-21T21:26:02.302942945Z', 'lewis', '2026-02-22T05:20:22.777165932Z', '2026-02-22T05:20:22.777150212Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1qt', 'quality: Add flow_extender contract tests for each rule', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-989zqtfm.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-989zqtfm.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-989zqtfm"
  title: "quality: Add flow_extender contract tests for each rule"
  type: "task"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete quality: Add flow_extender contract tests for each rule with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"quality: Add flow_extender contract tests for each rule has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"tests/graph_regressions.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-013.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify quality: Add flow_extender contract tests for each rule behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-989zqtfm/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"tests/graph_regressions.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'task', '2026-02-21T21:26:02.233434244Z', 'lewis', '2026-02-22T05:15:50.763565221Z', '2026-02-22T05:15:50.763549571Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1r2', 'telemetry: Track suggestion acceptance and rejection metrics', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-wfckozoz.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-wfckozoz.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-wfckozoz"
  title: "telemetry: Track suggestion acceptance and rejection metrics"
  type: "chore"
  priority: 3
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete telemetry: Track suggestion acceptance and rejection metrics with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"telemetry: Track suggestion acceptance and rejection metrics has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/metrics/model.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/metrics/store.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-019.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify telemetry: Track suggestion acceptance and rejection metrics behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-wfckozoz/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/metrics/model.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/metrics/store.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 3, 'chore', '2026-02-21T21:26:02.469126991Z', 'lewis', '2026-02-22T11:53:00.034036348Z', '2026-02-22T11:53:00.034018618Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1r3', 'restate-mapping: Enforce ctx.run / timeout / compensation recommendation bundle', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-pbreabet.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-pbreabet.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-pbreabet"
  title: "restate-mapping: Enforce ctx.run / timeout / compensation recommendation bundle"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete restate-mapping: Enforce ctx.run / timeout / compensation recommendation bundle with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"restate-mapping: Enforce ctx.run / timeout / compensation recommendation bundle has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"docs/10_RESTATE_SDK.md\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-016.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify restate-mapping: Enforce ctx.run / timeout / compensation recommendation bundle behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-pbreabet/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"docs/10_RESTATE_SDK.md\\", relevance: \\"Related implementation\\"},
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:02.370014397Z', 'lewis', '2026-02-22T05:16:06.856713096Z', '2026-02-22T05:16:06.856690676Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1tt', 'ui: Add one-click apply and multi-select apply actions', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-n5vzk9h5.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-n5vzk9h5.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-n5vzk9h5"
  title: "ui: Add one-click apply and multi-select apply actions"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete ui: Add one-click apply and multi-select apply actions with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"ui: Add one-click apply and multi-select apply actions has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/ui/selected_node_panel.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/hooks/use_workflow_state.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-010.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify ui: Add one-click apply and multi-select apply actions behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-n5vzk9h5/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/selected_node_panel.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/hooks/use_workflow_state.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:02.136304591Z', 'lewis', '2026-02-22T11:52:59.950464250Z', '2026-02-22T11:52:59.950447720Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1x5', 'flow-extender: Add dry-run graph patch preview model', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-omlzgqkm.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-omlzgqkm.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-omlzgqkm"
  title: "flow-extender: Add dry-run graph patch preview model"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Add dry-run graph patch preview model with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Add dry-run graph patch preview model has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/connectivity.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-003.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Add dry-run graph patch preview model behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-omlzgqkm/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/connectivity.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:01.923149777Z', 'lewis', '2026-02-22T05:16:06.902566599Z', '2026-02-22T05:16:06.902551419Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1x9', 'ui: Add extension history timeline and undo integration', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-oh9fl1id.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-oh9fl1id.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-oh9fl1id"
  title: "ui: Add extension history timeline and undo integration"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete ui: Add extension history timeline and undo integration with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"ui: Add extension history timeline and undo integration has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/ui/config_panel/execution.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/hooks/use_workflow_state.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-012.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify ui: Add extension history timeline and undo integration behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-oh9fl1id/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/config_panel/execution.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/hooks/use_workflow_state.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-21T21:26:02.199522170Z', 'lewis', '2026-02-22T11:52:59.978090871Z', '2026-02-22T11:52:59.978075522Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-1xl', 'schema: Optional field None vs empty string semantics inconsistent', 'Bug: Optional field semantics inconsistent between UI and serde

Location: 
- src/ui/workflow_nodes/service_call.rs lines 114-118
- src/ui/workflow_nodes/schema.rs
- src/ui/workflow_nodes/delayed_message.rs lines 110-114

Issue:
1. UI forms convert empty input to None:
   config.write().key = if value.trim().is_empty() { None } else { Some(value) };

2. But serde deserialization preserves empty strings as Some(""):
   "key": "" deserializes to Some("") not None

This creates an inconsistency:
- Serialized JSON with "key": "" loads as Some("")
- User edits the field and clears it becomes None
- Same field has different representations

Expected Behavior:
Either:
- Add deserialize with to None for empty strings
- Or document that empty string and None are equivalent

Severity: P2 - Data consistency issue', 'open', 0, 'bug', '2026-03-05T11:43:15.473099350Z', 'lewis', '2026-03-05T11:43:15.473099350Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-237', 'data-panel: Add NDV-lite input/output inspector for extension nodes', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-ptnyucna.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-ptnyucna.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-ptnyucna"
  title: "data-panel: Add NDV-lite input/output inspector for extension nodes"
  type: "feature"
  priority: 2
  effort_estimate: "4hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete data-panel: Add NDV-lite input/output inspector for extension nodes with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"data-panel: Add NDV-lite input/output inspector for extension nodes has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/ui/config_panel/execution.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/execution.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-017.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify data-panel: Add NDV-lite input/output inspector for extension nodes behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-ptnyucna/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/config_panel/execution.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/execution.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-21T21:26:02.400945434Z', 'lewis', '2026-02-22T11:53:43.149190807Z', '2026-02-22T11:53:43.149178238Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-25v', 'graph: Branching & Named Ports', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-y66lqfqy.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-y66lqfqy.cue


#EnhancedBead: {
  id: "new-app-20260219105843-y66lqfqy"
  title: "graph: Branching & Named Ports"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL support named ports on connections.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN an If node is encountered\\", shall: \\"THE SYSTEM SHALL branch execution based on condition.\\"}
    ]
    unwanted: [
      {condition: \\"IF a port is missing\\", shall_not: \\"THE SYSTEM SHALL NOT crash\\", because: \\"robustness\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow struct exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Connection has port fields\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Ports are non-empty strings\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/graph/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"How to represent ports in JSON?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Update models\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Update step logic\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219105843-y66lqfqy/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T16:58:43.704912165Z', 'lewis', '2026-02-19T17:10:01.586219727Z', '2026-02-19T17:10:01.586204387Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2co', 'delay: Zero duration silently converted without user feedback', 'Bug: Zero duration silently converted to 1 without notification

Location: src/ui/workflow_nodes/delay.rs line 84

Issue:
When user enters 0 for duration_ms, it is silently converted to 1:
  config.write().duration_ms = if v == 0 { 1 } else { v };

This differs from delayed_message.rs which shows an error message:
  delay_error.set(Some("Delay must be greater than 0 ms".to_string()));

Expected Behavior:
User should be notified when their input is modified, or input should be rejected.

Severity: P2 - UX inconsistency', 'open', 0, 'bug', '2026-03-05T11:43:07.821452911Z', 'lewis', '2026-03-05T11:43:07.821452911Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2cu', 'delayed_message: delay_ms lacks default, allows 0 on init', 'Bug: DelayedMessageConfig.delay_ms has no default value

Location:
- src/ui/workflow_nodes/schema.rs line 68
- src/ui/workflow_nodes/delayed_message.rs lines 187-194

Issue:
The schema defines delay_ms as:
  pub delay_ms: u64,

Without a default. If a DelayedMessageConfig is created programmatically without 
setting delay_ms, it will be 0. The UI validation only triggers when user edits 
the field (parsed > 0 check), but 0 is allowed in the type.

Compare to DelayConfig which has similar issue but is handled better in UI.

Expected Behavior:
Add #[serde(default = "default_delay_ms")] and a function that returns a valid 
delay value (e.g., 60000 for 1 minute).

Severity: P2', 'open', 0, 'bug', '2026-03-05T11:43:21.528707116Z', 'lewis', '2026-03-05T11:43:21.528707116Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2de', 'restate_client: unclear error for NULL in required fields', 'When a required field contains NULL in the database, the error message says column X is not a string instead of column X is NULL.', 'open', 2, 'bug', '2026-03-05T12:12:00.693227530Z', 'lewis', '2026-03-05T12:12:00.693227530Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2gq', 'fmt-gate: enforce rustfmt-clean graph module state', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260220074859-fmfql6y3.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260220074859-fmfql6y3.cue


#EnhancedBead: {
  id: "oya-frontend-20260220074859-fmfql6y3"
  title: "fmt-gate: enforce rustfmt-clean graph module state"
  type: "bug"
  priority: 0
  effort_estimate: "30min"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL keep source files rustfmt-clean before merge.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN moon run :fmt is executed\\", shall: \\"THE SYSTEM SHALL return exit code 0 with no file diffs.\\"}
    ]
    unwanted: [
      {condition: \\"IF committed code diverges from rustfmt output\\", shall_not: \\"THE SYSTEM SHALL NOT pass formatting gate\\", because: \\"non-deterministic formatting slows review and hides meaningful diffs.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Formatter configuration is present and loadable.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"moon run :fmt exits 0.\\",
        \\"No unstaged formatting-only changes remain after check.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Formatting gate behavior remains consistent across Linux developer environments.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/graph/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"moon.yml\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Can formatting-sensitive patterns be rewritten for readability without semantic change?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Capture exact rustfmt diff regions.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add regression check step in developer workflow docs if absent.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Apply formatting and optionally simplify wrapped expressions.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260220074859-fmfql6y3/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/mod.rs:133\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/mod.rs:402\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"tests/graph_regressions.rs\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 0, 'bug', '2026-02-20T13:48:59.894823071Z', 'lewis', '2026-02-22T05:20:27.174851654Z', '2026-02-22T05:20:27.174837914Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2jo', 'schema: UI and graph WorkflowNode types are completely mismatched', 'The UI schema in src/ui/workflow_nodes/schema.rs has completely different WorkflowNode variants than the graph schema in src/graph/workflow_node.rs.

Graph variants: HttpHandler, KafkaHandler, CronTrigger, WorkflowSubmit, Run, ServiceCall, ObjectCall, WorkflowCall, SendMessage, DelayedSend, GetState, SetState, ClearState, Condition, Switch, Loop, Parallel, Compensate, Sleep, Timeout, DurablePromise, Awakeable, ResolvePromise, SignalHandler

UI variants: HttpTrigger, ScheduleTrigger, ServiceCall, SendMessage, DelayedMessage, SaveToMemory, LoadFromMemory, Delay, Router, WaitForWebhook, WaitForSignal, RunCode

Even shared variants have different config structs. ServiceCallConfig in graph: {durable_step_name, service, endpoint}. ServiceCallConfig in UI: {target_type, service_name, key, handler_name, input, condition}

This causes serialization failures in apply_config_update.', 'open', 0, 'bug', '2026-03-05T11:47:44.222765763Z', 'lewis', '2026-03-05T11:47:44.222765763Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2t9', 'ui: Add parallel branch visual grouping', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-pgc0d01l.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-pgc0d01l.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-pgc0d01l"
  title: "ui: Add parallel branch visual grouping"
  type: "feature"
  priority: 3
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL visually group parallel branches\\",
      \\"THE SYSTEM SHALL show aggregate status for parallel group\\"
    ]
    event_driven: [
      {trigger: \\"WHEN parallel node has multiple outputs\\", shall: \\"THE SYSTEM SHALL draw container around branch nodes\\"},
      {trigger: \\"WHEN all branches complete\\", shall: \\"THE SYSTEM SHALL show merged result indicator\\"}
    ]
    unwanted: [
      {condition: \\"IF parallel group would overlap other nodes\\", shall_not: \\"THE SYSTEM SHALL NOT draw overlapping containers\\", because: \\"maintains canvas clarity\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Parallel node type exists\\",
        \\"FlowEdges draws connections\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Dashed container around parallel branches\\",
        \\"Branch count badge on container\\",
        \\"Aggregate status shown\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Container encompasses all branch nodes\\",
      \\"Join point clearly marked\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design parallel container SVG\\", done_when: \\"Tests pass\\"},
        {task: \\"Define branch detection algorithm\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-pgc0d01l/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/edges.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/connectivity.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"AWS Step Functions Parallel state visual\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 3, 'feature', '2026-02-22T14:33:40.713471639Z', 'lewis', '2026-02-22T14:33:40.713471639Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2tf', 'canvas: Multi-select & Lasso', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219114427-36ltzkqp.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219114427-36ltzkqp.cue


#EnhancedBead: {
  id: "new-app-20260219114427-36ltzkqp"
  title: "canvas: Multi-select & Lasso"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL allow users to select multiple nodes using a selection box.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN multiple nodes are selected\\", shall: \\"THE SYSTEM SHALL move all of them when one is dragged.\\"}
    ]
    unwanted: [
      {condition: \\"IF selection box is empty\\", shall_not: \\"THE SYSTEM SHALL NOT change selection\\", because: \\"it prevents accidental deselects\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Canvas is visible\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Multiple nodes can be moved at once\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Selection state must be consistent\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Implement selection_box signal\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Update NodeCard selection logic\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219114427-36ltzkqp/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T17:44:27.628146210Z', 'lewis', '2026-02-19T17:47:03.432474121Z', '2026-02-19T17:47:03.432465831Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2v8', 'flow-extender: Build compound plan generator from multiple suggestions', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-6rq5dqmt.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-6rq5dqmt.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-6rq5dqmt"
  title: "flow-extender: Build compound plan generator from multiple suggestions"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Build compound plan generator from multiple suggestions with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Build compound plan generator from multiple suggestions has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/scenario_runner/types.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-007.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Build compound plan generator from multiple suggestions behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-6rq5dqmt/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/scenario_runner/types.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:02.045061394Z', 'lewis', '2026-02-22T05:16:07.061869804Z', '2026-02-22T05:16:07.061841364Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-2x0', 'shell: localstorage persistence', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219070624-sovoxsnm.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219070624-sovoxsnm.cue


#EnhancedBead: {
  id: "new-app-20260219070624-sovoxsnm"
  title: "shell: localstorage persistence"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL serialize tasks to JSON for storage.\\",
      \\"THE SYSTEM SHALL use the key "dioxus-tasks" for storage.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN the app loads\\", shall: \\"THE SYSTEM SHALL attempt to restore state from LocalStorage.\\"}
    ]
    unwanted: [
      {condition: \\"IF LocalStorage is full\\", shall_not: \\"THE SYSTEM SHALL NOT crash\\", because: \\"persistence failures should be handled gracefully\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Window/LocalStorage available\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Data persisted to "dioxus-tasks" key\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Data remains consistent between sessions\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219070624-sovoxsnm/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T13:06:24.644726362Z', 'lewis', '2026-02-19T19:17:25.852514893Z', '2026-02-19T19:17:25.852508903Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-306', 'core: multi-project task organization', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219073540-znhyftdp.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219073540-znhyftdp.cue


#EnhancedBead: {
  id: "new-app-20260219073540-znhyftdp"
  title: "core: multi-project task organization"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL support multiple Projects with unique names.\\",
      \\"THE SYSTEM SHALL track the active Project for the UI.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a project is deleted\\", shall: \\"THE SYSTEM SHALL remove all associated tasks.\\"}
    ]
    unwanted: [
      {condition: \\"IF no projects exist\\", shall_not: \\"THE SYSTEM SHALL NOT crash\\", because: \\"a default "Inbox" project should be created\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"None (pure)\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Returned collection contains the new project\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Project names are unique\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Update Task model for project association\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement ProjectCollection logic\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219073540-znhyftdp/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T13:35:40.341356884Z', 'lewis', '2026-02-22T11:53:43.088533541Z', '2026-02-22T11:53:43.088515871Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-30w', 'engine: Execution History & Logs', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219114551-er4obyoi.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219114551-er4obyoi.cue


#EnhancedBead: {
  id: "new-app-20260219114551-er4obyoi"
  title: "engine: Execution History & Logs"
  type: "feature"
  priority: 3
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL store the results of the last 10 execution runs.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN an execution completes\\", shall: \\"THE SYSTEM SHALL append a new RunRecord to the history.\\"}
    ]
    unwanted: [
      {condition: \\"IF history exceeds 10 records\\", shall_not: \\"THE SYSTEM SHALL NOT grow indefinitely\\", because: \\"it would bloat LocalStorage\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Execution engine is functional\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"History is visible in the sidebar\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Each history record has a unique ID\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add RunRecord model to graph/mod.rs\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Update run() to record results\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219114551-er4obyoi/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 3, 'feature', '2026-02-19T17:45:51.387825831Z', 'lewis', '2026-02-19T17:47:03.432694319Z', '2026-02-19T17:47:03.432687469Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-32u', 'graph: Silent failure when config merge/parse fails in apply_config_update', 'In src/graph/mod.rs:180-194, apply_config_update has a silent failure mode:

1. It saves the normalized config to self.config (line 182)
2. Then tries to merge with node JSON and parse as WorkflowNode (lines 184-193)
3. If the merge/parse fails, it silently ignores the error

This leads to inconsistent state where:
- self.config has the new values
- But self.node, self.node_type, self.category, self.icon, self.description still have old values

The issue is that if the user makes config changes that can''t be parsed into a valid WorkflowNode, the config is saved but the node metadata is not updated, causing UI inconsistency.

Fix: Either:
a) Return a Result from apply_config_update and handle errors explicitly
b) Or ensure the config is always valid before saving

Evidence: Lines 184-193 use .ok() to silently ignore parse errors.', 'open', 1, 'bug', '2026-03-05T11:47:37.640330750Z', 'lewis', '2026-03-05T11:47:37.640330750Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-33d', 'storage: Workflow load silently falls back to default on deserialization failure', 'In src/hooks/use_workflow_state.rs lines 423-431, when loading from localStorage:

1. If JSON deserialization fails (e.g., schema version mismatch), it silently falls back to default_workflow
2. No error is logged or shown to user
3. User loses their workflow data without warning

Additionally, line 428 calls apply_config_update with the stored config which may have a different schema than the current code expects, causing silent failures.', 'open', 1, 'bug', '2026-03-05T11:47:10.122101691Z', 'lewis', '2026-03-05T11:47:10.122101691Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-35j', 'ui: dashboard and task components', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219070624-tabqh9hv.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219070624-tabqh9hv.cue


#EnhancedBead: {
  id: "new-app-20260219070624-tabqh9hv"
  title: "ui: dashboard and task components"
  type: "feature"
  priority: 3
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL use a responsive layout with a progress indicator.\\",
      \\"THE SYSTEM SHALL follow accessibility standards for task controls.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a task is completed\\", shall: \\"THE SYSTEM SHALL update the global progress bar.\\"}
    ]
    unwanted: [
      {condition: \\"IF on mobile\\", shall_not: \\"THE SYSTEM SHALL NOT hide essential task controls\\", because: \\"UX must be consistent\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Core logic implemented\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"UI reflects current state of tasks\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No state mutation inside render functions\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219070624-tabqh9hv/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 3, 'feature', '2026-02-19T13:06:24.687900286Z', 'lewis', '2026-02-22T11:53:43.073247553Z', '2026-02-22T11:53:43.073235053Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-388', 'engine: Rich Expression Resolver', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219114929-wk1oc1eg.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219114929-wk1oc1eg.cue


#EnhancedBead: {
  id: "new-app-20260219114929-wk1oc1eg"
  title: "engine: Rich Expression Resolver"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL evaluate arithmetic expressions in node configurations.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN an expression contains string methods\\", shall: \\"THE SYSTEM SHALL transform the string accordingly.\\"}
    ]
    unwanted: [
      {condition: \\"IF an expression is malformed\\", shall_not: \\"THE SYSTEM SHALL NOT crash the resolver\\", because: \\"it should return the raw expression string\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Basic path resolution exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Expressions can compute values from multiple node outputs\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"The resolver must remain side-effect free\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Create ExpressionParser calculation module\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement math and string method handlers\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219114929-wk1oc1eg/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T17:49:29.229627605Z', 'lewis', '2026-02-19T17:54:04.663439802Z', '2026-02-19T17:54:04.663419872Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-39q', 'types: Create unified WorkflowNode enum with typed configs', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-1txmyyhs.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-1txmyyhs.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-1txmyyhs"
  title: "types: Create unified WorkflowNode enum with typed configs"
  type: "feature"
  priority: 0
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL represent every node as a typed WorkflowNode variant\\",
      \\"THE SYSTEM SHALL derive category from WorkflowNode variant type\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a node is created\\", shall: \\"THE SYSTEM SHALL validate config against variant schema\\"},
      {trigger: \\"WHEN a workflow is serialized\\", shall: \\"THE SYSTEM SHALL preserve type information\\"}
    ]
    unwanted: [
      {condition: \\"IF node_type string does not match any variant\\", shall_not: \\"THE SYSTEM SHALL NOT create unknown node\\", because: \\"prevents runtime errors from invalid data\\"},
      {condition: \\"IF required config field is missing\\", shall_not: \\"THE SYSTEM SHALL NOT serialize node\\", because: \\"prevents data corruption\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"graph/mod.rs exists\\",
        \\"serde derive on all types\\",
        \\"NODE_TEMPLATES array defined\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Node contains WorkflowNode variant\\",
        \\"Category derivable from variant\\",
        \\"No serde_json::Value in Node.config\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"All 24 node types have corresponding WorkflowNode variants\\",
      \\"Variant names match NODE_TEMPLATES keys\\",
      \\"Round-trip serialization preserves types\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/graph/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/ui/workflow_nodes/schema.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/ui/sidebar/model.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/metadata.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"How to handle migration from old localStorage format?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read existing schema.rs and graph/mod.rs\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map 24 NODE_TEMPLATES to WorkflowNode variants\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Design migration strategy for localStorage\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write unit tests for typed config\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Write serialization roundtrip tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Write migration tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement WorkflowNode enum with all 24 variants\\", done_when: \\"Tests pass\\"},
        {task: \\"Update Node struct to contain WorkflowNode\\", done_when: \\"Tests pass\\"},
        {task: \\"Implement category derivation from variant\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-1txmyyhs/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/metadata.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/workflow_nodes/schema.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/sidebar/model.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Restate SDK service types\\",
      \\"AWS Step Functions state machine types\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 0, 'feature', '2026-02-22T14:33:40.520769023Z', 'lewis', '2026-02-22T14:33:40.520769023Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-39w', 'ui: SVG-based canvas and node rendering', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219075335-komc7q4d.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219075335-komc7q4d.cue


#EnhancedBead: {
  id: "new-app-20260219075335-komc7q4d"
  title: "ui: SVG-based canvas and node rendering"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL render nodes as SVG groups with labels.\\",
      \\"THE SYSTEM SHALL render connections as Bezier curves.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a node is dragged\\", shall: \\"THE SYSTEM SHALL update its position in state.\\"}
    ]
    unwanted: [
      {condition: \\"IF outside canvas bounds\\", shall_not: \\"THE SYSTEM SHALL NOT hide the node completely\\", because: \\"users need to pan back\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Graph model exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"UI reflects graph state\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"SVG coordinates match state positions\\",
      \\"Consistent scaling\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219075335-komc7q4d/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T13:53:35.856147825Z', 'lewis', '2026-02-22T11:53:43.119503494Z', '2026-02-22T11:53:43.119488664Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3a5', 'restate-mapping: Align extensions to Restate service/object/workflow semantics', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-a9ww83m0.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-a9ww83m0.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-a9ww83m0"
  title: "restate-mapping: Align extensions to Restate service/object/workflow semantics"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete restate-mapping: Align extensions to Restate service/object/workflow semantics with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"restate-mapping: Align extensions to Restate service/object/workflow semantics has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"docs/10_RESTATE_SDK.md\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/ui/sidebar/model.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-015.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify restate-mapping: Align extensions to Restate service/object/workflow semantics behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-a9ww83m0/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"docs/10_RESTATE_SDK.md\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/sidebar/model.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:02.335756014Z', 'lewis', '2026-02-22T05:16:06.831301188Z', '2026-02-22T05:16:06.831285269Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3b3', 'config_panel: Fix missing node parameter in config component calls', 'DurableConfig, StateConfig, FlowConfig, TimingConfig, and SignalConfig all expect node: WorkflowNode parameter but mod.rs call site doesn''t pass it, causing build failure.

Evidence:
- src/ui/config_panel/config_sections.rs:87 defines DurableConfig with node: WorkflowNode parameter
- src/ui/config_panel/mod.rs:129-133 calls these components WITHOUT the node parameter
- cargo build shows error: no method named icon found for struct DurableConfigPropsBuilder

Fix needed: Pass node.clone() to each config component call in mod.rs

Affected lines:
- mod.rs:129 (DurableConfig)
- mod.rs:130 (StateConfig)
- mod.rs:131 (FlowConfig)
- mod.rs:132 (TimingConfig)
- mod.rs:133 (SignalConfig)', 'open', 0, 'bug', '2026-03-05T11:47:15.343336280Z', 'lewis', '2026-03-05T11:47:15.343336280Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3bb', 'ui: Add node execution state badges and styling', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-mfkgrjpy.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-mfkgrjpy.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-mfkgrjpy"
  title: "ui: Add node execution state badges and styling"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL display execution state on every node\\",
      \\"THE SYSTEM SHALL use consistent color coding for all states\\"
    ]
    event_driven: [
      {trigger: \\"WHEN node state changes to Running\\", shall: \\"THE SYSTEM SHALL show animated spinner badge\\"},
      {trigger: \\"WHEN node state changes to Failed\\", shall: \\"THE SYSTEM SHALL show red error badge with icon\\"}
    ]
    unwanted: [
      {condition: \\"IF multiple states shown simultaneously\\", shall_not: \\"THE SYSTEM SHALL NOT show conflicting badges\\", because: \\"confuses user about actual state\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"ExecutionState enum exists\\",
        \\"FlowNodeComponent exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Visual badge for each state\\",
        \\"Animations for Running state\\",
        \\"Accessible color contrast\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Badge position consistent across all nodes\\",
      \\"Colors match design system\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design badge component\\", done_when: \\"Tests pass\\"},
        {task: \\"Implement state-to-style mapping\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-mfkgrjpy/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/node.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/icons/\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"AWS Step Functions node status\\",
      \\"N8n execution indicators\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 1, 'feature', '2026-02-22T14:33:40.575628593Z', 'lewis', '2026-02-22T14:33:40.575628593Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3cq', 'sync: LocalStorage Sync', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-qhaewazo.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-qhaewazo.cue


#EnhancedBead: {
  id: "new-app-20260219105843-qhaewazo"
  title: "sync: LocalStorage Sync"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL persist workflow to browser storage.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN the workflow state changes\\", shall: \\"THE SYSTEM SHALL save to storage after a debounce.\\"}
    ]
    unwanted: [
      {condition: \\"IF storage is full\\", shall_not: \\"THE SYSTEM SHALL NOT lose in-memory state\\", because: \\"it fallback to memory\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"LocalStorage is available\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"State is saved periodically\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Saved JSON is valid\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219105843-qhaewazo/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T16:58:43.853989176Z', 'lewis', '2026-02-19T17:10:01.587028260Z', '2026-02-19T17:10:01.587021620Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3e2', 'node: Function Node Implementation', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219114929-zvww08m5.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219114929-zvww08m5.cue


#EnhancedBead: {
  id: "new-app-20260219114929-zvww08m5"
  title: "node: Function Node Implementation"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL provide a Function node type.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a Function node executes\\", shall: \\"THE SYSTEM SHALL produce a new JSON object based on the defined mapping.\\"}
    ]
    unwanted: [
      {condition: \\"IF the function output is invalid JSON\\", shall_not: \\"THE SYSTEM SHALL NOT pass it to downstream nodes\\", because: \\"it would break downstream nodes\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Node model supports dynamic types\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Function node output matches the evaluated template\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Function node must not access browser globals\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add Function variant to execute_node_type\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement template evaluation logic\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219114929-zvww08m5/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T17:49:29.276843611Z', 'lewis', '2026-02-19T17:54:04.663764799Z', '2026-02-19T17:54:04.663755389Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3ei', 'validation: Add workflow-level validation panel', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-f5d3vspx.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-f5d3vspx.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-f5d3vspx"
  title: "validation: Add workflow-level validation panel"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL validate workflow structure\\",
      \\"THE SYSTEM SHALL show all validation issues in one place\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user clicks Run\\", shall: \\"THE SYSTEM SHALL validate and show errors if any\\"},
      {trigger: \\"WHEN user clicks validation error\\", shall: \\"THE SYSTEM SHALL navigate to problematic node\\"}
    ]
    unwanted: [
      {condition: \\"IF workflow has validation errors\\", shall_not: \\"THE SYSTEM SHALL NOT execute workflow\\", because: \\"prevents confusing runtime failures\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow has nodes and connections\\",
        \\"Validation functions exist\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Validation panel lists all issues\\",
        \\"Click navigates to node\\",
        \\"Run blocked on errors\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Validation runs on every workflow change\\",
      \\"Issues categorized by severity\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design validation rule set\\", done_when: \\"Tests pass\\"},
        {task: \\"Create validation engine\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-f5d3vspx/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/connectivity.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/toolbar.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"TypeScript error panel\\",
      \\"ESLint warnings\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 2, 'feature', '2026-02-22T14:33:40.752626569Z', 'lewis', '2026-02-22T14:33:40.752626569Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3g5', 'ui: sidebar navigation and fluid animations', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219073540-dgl92iff.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219073540-dgl92iff.cue


#EnhancedBead: {
  id: "new-app-20260219073540-dgl92iff"
  title: "ui: sidebar navigation and fluid animations"
  type: "feature"
  priority: 3
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL use CSS transitions for smooth task completion and deletion.\\",
      \\"THE SYSTEM SHALL display a project-specific progress indicator.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a project is selected\\", shall: \\"THE SYSTEM SHALL update the task list view with a fade-in effect.\\"}
    ]
    unwanted: [
      {condition: \\"IF sidebar is empty\\", shall_not: \\"THE SYSTEM SHALL NOT hide project creation controls\\", because: \\"users must be able to add the first project\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Multi-project logic implemented\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"UI correctly reflects the selected project state\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No state mutation inside render functions\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219073540-dgl92iff/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 3, 'feature', '2026-02-19T13:35:40.389104246Z', 'lewis', '2026-02-22T11:53:43.104433424Z', '2026-02-22T11:53:43.104421684Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3gv', '[test] E2E Playwright tests timeout due to Chromium version compatibility', '## Context
The Playwright E2E UI tests (`tests/ui_test.rs`) are implemented and logically correct. However, they consistently time out in the current environment because the Chromium version required by the `playwright-rust` driver (v878941) fails to load/execute the modern Dioxus 0.7 WASM bundle.

## Requirements (EARS)
- While the app is served via `dx serve`, the E2E tests shall be able to interact with the UI.
- If the browser fails to load the WASM bundle, then the tests shall provide descriptive failure logs.

## Acceptance Criteria
1. `cargo test --test ui_test` passes consistently.
2. Screenshots of the running application are successfully captured.', 'closed', 3, 'task', '2026-02-19T19:16:54.185073234Z', 'lewis', '2026-02-22T11:53:43.164618784Z', '2026-02-22T11:53:43.164607264Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3jx', 'config: Incorrect alias mapping for service_name and handler_name', 'In src/graph/mod.rs line 125, the alias mapping incorrectly maps multiple keys to the same target: targetService | service_name | handler_name all map to target. This causes data loss because service_name and handler_name should remain as separate fields, not be overwritten to target.', 'open', 1, 'bug', '2026-03-05T11:47:54.373940827Z', 'lewis', '2026-03-05T11:47:54.373940827Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-3uj', 'ui: Add edge animation during execution', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-c3hjyfjq.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-c3hjyfjq.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-c3hjyfjq"
  title: "ui: Add edge animation during execution"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL visualize data flow direction on edges\\",
      \\"THE SYSTEM SHALL animate active execution paths\\"
    ]
    event_driven: [
      {trigger: \\"WHEN node transitions to Running\\", shall: \\"THE SYSTEM SHALL animate incoming edge\\"},
      {trigger: \\"WHEN workflow execution stops\\", shall: \\"THE SYSTEM SHALL stop all edge animations\\"}
    ]
    unwanted: [
      {condition: \\"IF animations cause performance issues\\", shall_not: \\"THE SYSTEM SHALL NOT animate more than 10 edges simultaneously\\", because: \\"prevents browser lag\\"},
      {condition: \\"IF reduced motion preference is set\\", shall_not: \\"THE SYSTEM SHALL NOT play animations\\", because: \\"accessibility requirement\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"FlowEdges component exists\\",
        \\"ExecutionState tracks running nodes\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"CSS animation on active edges\\",
        \\"Animation respects prefers-reduced-motion\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Only one edge animated per running node\\",
      \\"Animation speed consistent\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design edge animation CSS\\", done_when: \\"Tests pass\\"},
        {task: \\"Define active edge detection\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-c3hjyfjq/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/edges.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/main.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"N8n edge animation\\",
      \\"Step Functions path highlighting\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 2, 'feature', '2026-02-22T14:33:40.653951654Z', 'lewis', '2026-02-22T14:33:40.653951654Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-7z8', 'templates: Add extension presets library (webhook, approval, retry saga)', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-s5txqqnb.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-s5txqqnb.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-s5txqqnb"
  title: "templates: Add extension presets library (webhook, approval, retry saga)"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete templates: Add extension presets library (webhook, approval, retry saga) with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"templates: Add extension presets library (webhook, approval, retry saga) has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/ui/sidebar/model.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-018.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify templates: Add extension presets library (webhook, approval, retry saga) behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-s5txqqnb/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/sidebar/model.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-21T21:26:02.434511980Z', 'lewis', '2026-02-22T05:20:31.396716858Z', '2026-02-22T05:20:31.396701618Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-85p', 'flow-extender: Add extension dependency graph and ordering', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-5znamkay.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-5znamkay.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-5znamkay"
  title: "flow-extender: Add extension dependency graph and ordering"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Add extension dependency graph and ordering with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Add extension dependency graph and ordering has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/layout.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-006.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Add extension dependency graph and ordering behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-5znamkay/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/layout.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:02.014672454Z', 'lewis', '2026-02-22T05:16:07.039960322Z', '2026-02-22T05:16:07.039945732Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-9lh', 'core: task model and pure logic', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219070624-e2akt0jt.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219070624-e2akt0jt.cue


#EnhancedBead: {
  id: "new-app-20260219070624-e2akt0jt"
  title: "core: task model and pure logic"
  type: "feature"
  priority: 1
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL define a Task with ID, content, and completion state.\\",
      \\"THE SYSTEM SHALL ensure Task IDs are unique within a collection.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a task is toggled\\", shall: \\"THE SYSTEM SHALL return a new task state with flipped completion.\\"}
    ]
    unwanted: [
      {condition: \\"IF task content is empty\\", shall_not: \\"THE SYSTEM SHALL NOT allow task creation\\", because: \\"empty tasks clutter the UI\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"None (pure)\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Returned state is valid Task\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Task IDs are unique\\",
      \\"Task content is non-empty\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write unit tests for task transitions\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement Task struct and pure functions\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219070624-e2akt0jt/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T13:06:24.602332032Z', 'lewis', '2026-02-22T11:53:43.058250872Z', '2026-02-22T11:53:43.058237782Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-a2w', 'ui: Add Extend Flow panel in selected node sidebar', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-3bq72tkg.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-3bq72tkg.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-3bq72tkg"
  title: "ui: Add Extend Flow panel in selected node sidebar"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete ui: Add Extend Flow panel in selected node sidebar with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"ui: Add Extend Flow panel in selected node sidebar has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/ui/selected_node_panel.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/ui/config_panel/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-009.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify ui: Add Extend Flow panel in selected node sidebar behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-3bq72tkg/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/selected_node_panel.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/config_panel/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:02.106760558Z', 'lewis', '2026-02-22T11:52:59.922630330Z', '2026-02-22T11:52:59.922615330Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-b6f', 'flow-extender: Add rollback snapshot support for extension batches', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-jxt18zrg.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-jxt18zrg.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-jxt18zrg"
  title: "flow-extender: Add rollback snapshot support for extension batches"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Add rollback snapshot support for extension batches with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Add rollback snapshot support for extension batches has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/hooks/use_workflow_state.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-008.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Add rollback snapshot support for extension batches behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-jxt18zrg/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/hooks/use_workflow_state.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-21T21:26:02.076683128Z', 'lewis', '2026-02-22T11:53:00.006271898Z', '2026-02-22T11:53:00.006257078Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-bd6', 'ui: Add keyboard navigation between nodes', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-w3nz7rao.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-w3nz7rao.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-w3nz7rao"
  title: "ui: Add keyboard navigation between nodes"
  type: "feature"
  priority: 3
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL support keyboard-only navigation\\",
      \\"THE SYSTEM SHALL show focus indicator on selected node\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user presses Tab\\", shall: \\"THE SYSTEM SHALL move to next connected node\\"},
      {trigger: \\"WHEN user presses Arrow keys\\", shall: \\"THE SYSTEM SHALL move selected node by grid units\\"}
    ]
    unwanted: [
      {condition: \\"IF no connected node exists\\", shall_not: \\"THE SYSTEM SHALL NOT trap focus\\", because: \\"maintains accessibility\\"},
      {condition: \\"IF modifier key held\\", shall_not: \\"THE SYSTEM SHALL NOT perform navigation action\\", because: \\"allows browser shortcuts\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Canvas has keyboard focus handler\\",
        \\"Selection state exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Tab navigates forward through flow\\",
        \\"Shift+Tab navigates backward\\",
        \\"Arrows move node position\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Focus visible on canvas\\",
      \\"No keyboard traps\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design keyboard navigation algorithm\\", done_when: \\"Tests pass\\"},
        {task: \\"Define grid snap for arrows\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-w3nz7rao/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/main.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/hooks/use_selection.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"N8n keyboard shortcuts\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 3, 'feature', '2026-02-22T14:33:40.692950186Z', 'lewis', '2026-02-22T14:33:40.692950186Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-dt0', 'ui: Add inline node config expansion on click', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-r5idx7nl.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-r5idx7nl.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-r5idx7nl"
  title: "ui: Add inline node config expansion on click"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL allow config editing directly on canvas\\",
      \\"THE SYSTEM SHALL preserve existing side panel functionality\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user double-clicks node\\", shall: \\"THE SYSTEM SHALL expand inline config editor\\"},
      {trigger: \\"WHEN user clicks outside inline editor\\", shall: \\"THE SYSTEM SHALL collapse and save changes\\"}
    ]
    unwanted: [
      {condition: \\"IF inline editor would overlap other nodes\\", shall_not: \\"THE SYSTEM SHALL NOT obscure other node content\\", because: \\"maintains canvas readability\\"},
      {condition: \\"IF node has many config fields\\", shall_not: \\"THE SYSTEM SHALL NOT show all fields inline\\", because: \\"prevents oversized inline panel\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"FlowNodeComponent exists\\",
        \\"NodeConfigEditor exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Inline panel shows essential config\\",
        \\"Changes sync to side panel\\",
        \\"Expand/collapse animated\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Only one inline editor open at a time\\",
      \\"Side panel stays in sync\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design inline panel component\\", done_when: \\"Tests pass\\"},
        {task: \\"Define essential fields per node type\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-r5idx7nl/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/node.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/config_panel/mod.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"N8n inline node editor\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 2, 'feature', '2026-02-22T14:33:40.672360053Z', 'lewis', '2026-02-22T14:33:40.672360053Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-i0r', 'ui: Add preview overlay for proposed nodes and edges', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-qd6ejznt.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-qd6ejznt.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-qd6ejznt"
  title: "ui: Add preview overlay for proposed nodes and edges"
  type: "feature"
  priority: 1
  effort_estimate: "4hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete ui: Add preview overlay for proposed nodes and edges with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"ui: Add preview overlay for proposed nodes and edges has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/ui/node.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/ui/edges.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/main.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-011.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify ui: Add preview overlay for proposed nodes and edges behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-qd6ejznt/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/node.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/edges.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/main.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:02.168586523Z', 'lewis', '2026-02-22T11:52:41.968661530Z', '2026-02-22T11:52:41.968637280Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-jw3', 'ui: Node configuration sidebar with dynamic expressions', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-uencc6ft.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-uencc6ft.cue


#EnhancedBead: {
  id: "new-app-20260219080116-uencc6ft"
  title: "ui: Node configuration sidebar with dynamic expressions"
  type: "feature"
  priority: 1
  effort_estimate: "4hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL show a settings panel when a node is double-clicked.\\",
      \\"THE SYSTEM SHALL support dynamic data mapping using expressions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a parameter is changed\\", shall: \\"THE SYSTEM SHALL update the node state and trigger a re-validation.\\"}
    ]
    unwanted: [
      {condition: \\"IF the expression is invalid\\", shall_not: \\"THE SYSTEM SHALL NOT crash the UI\\", because: \\"users need to see the error to fix it\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Node selected\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Node config matches input schema\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Panel is always visible on the right\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219080116-uencc6ft/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T14:01:17.087235869Z', 'lewis', '2026-02-19T19:17:25.852334945Z', '2026-02-19T19:17:25.852326495Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `assignee`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-k3k', 'compile: Fix 67 compilation errors blocking cargo test', 'The entire codebase has 67 compilation errors preventing cargo test from running. Multiple root causes:

1. Missing Rect import in src/ui/edges.rs line 8
2. Missing normalize_bend_delta function in src/ui/edges.rs
3. ParallelGroup struct field mismatch - code uses source_node, target_nodes, bounds but struct has bounding_box
4. Missing ReadableExt trait import in use_frozen_mode.rs and use_sidebar.rs
5. NodeId comparison errors in editor_interactions.rs - trying to use string methods on Uuid
6. Invalid f32 dereference in main.rs line 979

Files affected:
- src/ui/edges.rs
- src/ui/parallel_group_overlay.rs  
- src/hooks/use_frozen_mode.rs
- src/hooks/use_sidebar.rs
- src/ui/editor_interactions.rs
- src/main.rs

The code does not compile and cargo test cannot run.', 'closed', 0, 'bug', 'self', '2026-03-05T11:43:07.064155035Z', 'lewis', '2026-03-05T12:57:57.271439706Z', '2026-03-05T12:57:57.270922461Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-mul', 'ui: Add execution history timeline panel', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-aph61c5s.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-aph61c5s.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-aph61c5s"
  title: "ui: Add execution history timeline panel"
  type: "feature"
  priority: 2
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL record execution events with timestamps\\",
      \\"THE SYSTEM SHALL display history in chronological order\\"
    ]
    event_driven: [
      {trigger: \\"WHEN node execution completes\\", shall: \\"THE SYSTEM SHALL add event to history\\"},
      {trigger: \\"WHEN user clicks history item\\", shall: \\"THE SYSTEM SHALL scroll to and select that node\\"}
    ]
    unwanted: [
      {condition: \\"IF history exceeds memory limit\\", shall_not: \\"THE SYSTEM SHALL NOT crash from memory\\", because: \\"prevents browser crashes on long runs\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow has history field\\",
        \\"RunRecord exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Timeline panel with events\\",
        \\"Click to navigate to node\\",
        \\"Timestamps and durations visible\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Events ordered by time\\",
      \\"Most recent at bottom\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design timeline component\\", done_when: \\"Tests pass\\"},
        {task: \\"Define event data structure\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-aph61c5s/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/main.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"AWS Step Functions execution history\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 2, 'feature', '2026-02-22T14:33:40.615748716Z', 'lewis', '2026-02-22T14:33:40.615748716Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-om5', 'flow-extender: Add confidence scoring and rationale classifier', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-di3jigrs.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-di3jigrs.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-di3jigrs"
  title: "flow-extender: Add confidence scoring and rationale classifier"
  type: "feature"
  priority: 2
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Add confidence scoring and rationale classifier with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Add confidence scoring and rationale classifier has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/ui/config_panel/execution.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-002.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Add confidence scoring and rationale classifier behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-di3jigrs/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/config_panel/execution.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-21T21:26:01.892520319Z', 'lewis', '2026-02-22T05:20:31.381393002Z', '2026-02-22T05:20:31.381377602Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-pzw', 'ui: Connection Snapping', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-4qjbfuix.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219105843-4qjbfuix.cue


#EnhancedBead: {
  id: "new-app-20260219105843-4qjbfuix"
  title: "ui: Connection Snapping"
  type: "feature"
  priority: 3
  effort_estimate: "1hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL draw a phantom wire during dragging.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN dragging from a pin\\", shall: \\"THE SYSTEM SHALL show a phantom line to the cursor.\\"}
    ]
    unwanted: [
      {condition: \\"IF dragging to invalid pin\\", shall_not: \\"THE SYSTEM SHALL NOT snap\\", because: \\"clarity\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Pin is clicked\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Phantom wire is visible\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Coordinate transformation is accurate\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219105843-4qjbfuix/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 3, 'feature', '2026-02-19T16:58:43.902291708Z', 'lewis', '2026-02-19T17:10:01.587218358Z', '2026-02-19T17:10:01.587212338Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-u3a', 'engine: Step-by-step execution and wire-data inspection', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-sergwlss.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-sergwlss.cue


#EnhancedBead: {
  id: "new-app-20260219080116-sergwlss"
  title: "engine: Step-by-step execution and wire-data inspection"
  type: "feature"
  priority: 2
  effort_estimate: "4hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL display the data payload on top of wires after execution.\\",
      \\"THE SYSTEM SHALL support ''Test Step'' for individual node isolation.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN a node fails\\", shall: \\"THE SYSTEM SHALL highlight the node in red and stop the execution branch.\\"}
    ]
    unwanted: [
      {condition: \\"IF a large payload is on a wire\\", shall_not: \\"THE SYSTEM SHALL NOT clutter the canvas\\", because: \\"it should be collapsible\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow graph is valid\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Execution results stored per node\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Real-time status matches engine state\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement to make tests pass\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219080116-sergwlss/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 2, 'feature', '2026-02-19T14:01:17.177651536Z', 'lewis', '2026-02-19T19:17:25.852681392Z', '2026-02-19T19:17:25.852675842Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-w0m', 'ui: Add input/output payload preview panel', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-i1iefcgk.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260222083340-i1iefcgk.cue


#EnhancedBead: {
  id: "oya-frontend-20260222083340-i1iefcgk"
  title: "ui: Add input/output payload preview panel"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL display input payload for all nodes\\",
      \\"THE SYSTEM SHALL display output payload for executed nodes\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user selects executed node\\", shall: \\"THE SYSTEM SHALL show Input and Output tabs with data\\"},
      {trigger: \\"WHEN user clicks copy button\\", shall: \\"THE SYSTEM SHALL copy JSON to clipboard\\"}
    ]
    unwanted: [
      {condition: \\"IF payload contains sensitive data\\", shall_not: \\"THE SYSTEM SHALL NOT display secrets in plain text\\", because: \\"prevents credential exposure\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"SelectedNodePanel exists\\",
        \\"Node has last_output field\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Input tab with JSON viewer\\",
        \\"Output tab with JSON viewer\\",
        \\"Copy to clipboard works\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"JSON is valid and formatted\\",
      \\"Empty state shown for no data\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Write failing tests\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Design payload viewer component\\", done_when: \\"Tests pass\\"},
        {task: \\"Implement JSON formatting\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260222083340-i1iefcgk/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/ui/selected_node_panel.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/ui/config_panel/execution.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"AWS Step Functions input/output view\\",
      \\"N8n data preview\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'open', 1, 'feature', '2026-02-22T14:33:40.595218313Z', 'lewis', '2026-02-22T14:33:40.595218313Z', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-wki', 'flow-extender: Introduce extension conflict detection', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-pegfzo8k.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260221152601-pegfzo8k.cue


#EnhancedBead: {
  id: "oya-frontend-20260221152601-pegfzo8k"
  title: "flow-extender: Introduce extension conflict detection"
  type: "feature"
  priority: 1
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL preserve DAG safety and avoid invalid connections when generating extensions.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN user requests flow extension from current workflow state\\", shall: \\"THE SYSTEM SHALL complete flow-extender: Introduce extension conflict detection with deterministic outputs and actionable diagnostics.\\"}
    ]
    unwanted: [
      {condition: \\"IF extension planning detects uncertainty or conflict\\", shall_not: \\"THE SYSTEM SHALL NOT silently mutate workflow topology\\", because: \\"Silent mutation breaks trust and makes debugging impossible.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow JSON is parseable and internally consistent.\\",
        \\"Node identifiers remain unique before mutation.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"All generated changes are represented as explicit node/edge operations.\\",
        \\"flow-extender: Introduce extension conflict detection has deterministic behavior for same input workflow and options.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"No self-connections are introduced.\\",
      \\"Existing user-authored nodes and edges are never deleted implicitly.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/flow_extender/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/errors.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Where should extension metadata live for persistence and undo safety?\\", answered: false},
      {question: \\"How should this task surface diagnostics to users and tests?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read and annotate target files for task-005.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"},
        {task: \\"Map existing abstractions to new contracts before writing code.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add failing tests that specify flow-extender: Introduce extension conflict detection behavior.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"},
        {task: \\"Add failure-path tests for conflicts and malformed inputs.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement minimal production code to satisfy tests.\\", done_when: \\"Tests pass\\"},
        {task: \\"Wire feature into existing hooks/UI/state plumbing as required.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260221152601-pegfzo8k/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/flow_extender/mod.rs\\", relevance: \\"Related implementation\\"},
      {path: \\"src/errors.rs\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"Use given/when/then tests already in repository as style reference.\\",
      \\"Follow existing workflow mutation and undo stack conventions.\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-21T21:26:01.984024276Z', 'lewis', '2026-02-22T05:16:07.016951491Z', '2026-02-22T05:16:07.016935551Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-wmk', 'runtime-engine: fail run when cyclic graph leaves nodes unexecuted', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260220074859-uemcmy1x.cue implementation.cue
# Schema location: /home/lewis/src/oya-frontend/.beads/schemas/oya-frontend-20260220074859-uemcmy1x.cue


#EnhancedBead: {
  id: "oya-frontend-20260220074859-uemcmy1x"
  title: "runtime-engine: fail run when cyclic graph leaves nodes unexecuted"
  type: "bug"
  priority: 0
  effort_estimate: "2hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL report run success only when all non-skipped nodes execute to completion.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN prepare_run cannot schedule all nodes due to cycle or unresolved dependencies\\", shall: \\"THE SYSTEM SHALL mark run as failed with actionable error metadata.\\"}
    ]
    unwanted: [
      {condition: \\"IF execution_queue excludes nodes that remain pending\\", shall_not: \\"THE SYSTEM SHALL NOT record run.success as true\\", because: \\"it creates false-positive telemetry and hides runtime defects.\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Workflow may contain cycles because add_connection does not reject cyclic edges.\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"run history success=false when pending, unexecuted nodes remain.\\",
        \\"At least one node or run-level error explains unschedulable graph state.\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Every node is either completed, failed, or explicitly skipped before success=true.\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      {path: \\"src/graph/mod.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"src/graph/layout.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"},
      {path: \\"tests/graph_regressions.rs\\", what_to_extract: \\"Existing patterns\\", document_in: \\"research_notes.md\\"}
    ]
    research_questions: [
      {question: \\"Should cycle detection live in add_connection, prepare_run, or both for defense in depth?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Trace lifecycle of node status from prepare_run through run completion.\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Add regression tests for cyclic workflows and partial scheduling.\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Implement explicit unschedulable-node detection and error propagation.\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/oya-frontend-20260220074859-uemcmy1x/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      {path: \\"src/graph/mod.rs:452\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/mod.rs:673\\", relevance: \\"Related implementation\\"},
      {path: \\"src/graph/layout.rs:56\\", relevance: \\"Related implementation\\"}
    ]
    similar_implementations: [
      \\"tests/graph_regressions.rs:258\\"
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 0, 'bug', '2026-02-20T13:48:59.973016520Z', 'lewis', '2026-02-22T05:20:27.189092830Z', '2026-02-22T05:20:27.189079640Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `closed_at`, `close_reason`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-wqb', 'canvas: Advanced interactions (Pan, Zoom, Minimap, Lasso)', '# CUE Validation Schema
# Validate implementation: cue vet /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-eoh5kfki.cue implementation.cue
# Schema location: /home/lewis/src/new-app/.beads/schemas/new-app-20260219080116-eoh5kfki.cue


#EnhancedBead: {
  id: "new-app-20260219080116-eoh5kfki"
  title: "canvas: Advanced interactions (Pan, Zoom, Minimap, Lasso)"
  type: "feature"
  priority: 1
  effort_estimate: "4hr"
  labels: ["planner-generated"]

  clarifications: {
    clarification_status: "RESOLVED"
  }

  ears_requirements: {
    ubiquitous: [
      \\"THE SYSTEM SHALL support panning the canvas via middle-click or space+drag.\\",
      \\"THE SYSTEM SHALL provide a minimap showing the entire graph overview.\\"
    ]
    event_driven: [
      {trigger: \\"WHEN the zoom level changes\\", shall: \\"THE SYSTEM SHALL scale the SVG viewport around the mouse cursor.\\"}
    ]
    unwanted: [
      {condition: \\"IF nodes are outside the current view\\", shall_not: \\"THE SYSTEM SHALL NOT remove them from the DOM\\", because: \\"it breaks state and accessibility\\"}
    ]
  }

  contracts: {
    preconditions: {
      auth_required: false
      required_inputs: []
      system_state: [
        \\"Canvas element exists\\"
      ]
    }
    postconditions: {
      state_changes: [
        \\"Transform matrix is correctly updated\\"
      ]
      return_guarantees: []
    }
    invariants: [
      \\"Minimum zoom 0.1x, Maximum zoom 5x\\"
    ]
  }

  research_requirements: {
    files_to_read: [
      
    ]
    research_questions: [
      {question: \\"What existing patterns should be followed?\\", answered: false}
    ]
    research_complete_when: [
      "All files have been read and patterns documented"
    ]
  }

  inversions: {
    usability_failures: [
      {failure: "User encounters unclear error", prevention: "Provide specific error messages", test_for_it: "test_error_messages_are_clear"}
    ]
  }

  acceptance_tests: {
    happy_paths: [
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"},
      {name: \\"test_happy_path\\", given: \\"Valid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is 0\\", \\"Output is correct\\"], real_input: \\"command input\\", expected_output: \\"expected output\\"}
    ]
    error_paths: [
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"},
      {name: \\"test_error_path\\", given: \\"Invalid inputs\\", when: \\"User executes command\\", then: [\\"Exit code is non-zero\\", \\"Error message is clear\\"], real_input: \\"invalid input\\", expected_output: null, expected_error: \\"error message\\"}
    ]
  }

  e2e_tests: {
    pipeline_test: {
      name: "test_full_pipeline"
      description: "End-to-end test of full workflow"
      setup: {}
      execute: {
        command: "intent command"
      }
      verify: {
        exit_code: 0
      }
    }
  }

  verification_checkpoints: {
    gate_0_research: {
      name: "Research Gate"
      must_pass_before: "Writing code"
      checks: ["All research questions answered"]
      evidence_required: ["Research notes documented"]
    }
    gate_1_tests: {
      name: "Test Gate"
      must_pass_before: "Implementation"
      checks: ["All tests written and failing"]
      evidence_required: ["Test files exist"]
    }
    gate_2_implementation: {
      name: "Implementation Gate"
      must_pass_before: "Completion"
      checks: ["All tests pass"]
      evidence_required: ["CI green"]
    }
    gate_3_integration: {
      name: "Integration Gate"
      must_pass_before: "Closing bead"
      checks: ["E2E tests pass"]
      evidence_required: ["Manual verification complete"]
    }
  }

  implementation_tasks: {
    phase_0_research: {
      parallelizable: true
      tasks: [
        {task: \\"Read relevant files and understand existing patterns\\", done_when: \\"Documented\\", parallel_group: \\"research\\"}
      ]
    }
    phase_1_tests_first: {
      parallelizable: true
      gate_required: "gate_0_research"
      tasks: [
        {task: \\"Implement Viewport matrix state\\", done_when: \\"Test exists and fails\\", parallel_group: \\"tests\\"}
      ]
    }
    phase_2_implementation: {
      parallelizable: false
      gate_required: "gate_1_tests"
      tasks: [
        {task: \\"Add MouseWheel zoom and Drag pan handlers\\", done_when: \\"Tests pass\\"}
      ]
    }
    phase_4_verification: {
      parallelizable: true
      gate_required: "gate_2_implementation"
      tasks: [
        {task: "Run moon run :ci", done_when: "CI passes", parallel_group: "verification"}
      ]
    }
  }

  failure_modes: {
    failure_modes: [
      {symptom: "Feature does not work", likely_cause: "Implementation incomplete", where_to_look: [{file: "src/main.rs", what_to_check: "Implementation logic"}], fix_pattern: "Complete implementation"}
    ]
  }

  anti_hallucination: {
    read_before_write: [
      {file: "src/main.rs", must_read_first: true, key_sections_to_understand: ["Main entry point"]}
    ]
    apis_that_exist: []
    no_placeholder_values: ["Use real data from codebase"]
    git_verification: {
      before_claiming_done: "git status && git diff && moon run :test"
    }
  }

  context_survival: {
    progress_file: {
      path: ".bead-progress/new-app-20260219080116-eoh5kfki/progress.txt"
      format: "Markdown checklist"
    }
    recovery_instructions: "Read progress.txt and continue from current task"
  }

  completion_checklist: {
    tests: [
      "[ ] All acceptance tests written and passing",
      "[ ] All error path tests written and passing",
      "[ ] E2E pipeline test passing with real data",
      "[ ] No mocks or fake data in any test"
    ]
    code: [
      "[ ] Implementation uses Result<T, Error> throughout",
      "[ ] Zero unwrap or expect calls"
    ]
    ci: [
      "[ ] moon run :ci passes"
    ]
  }

  context: {
    related_files: [
      
    ]
    similar_implementations: [
      
    ]
  }

  ai_hints: {
    do: [
      "Use functional patterns: map, and_then, ?",
      "Return Result<T, Error> from all fallible functions",
      "READ files before modifying them"
    ]
    do_not: [
      "Do NOT use unwrap or expect",
      "Do NOT use panic!, todo!, or unimplemented!",
      "Do NOT modify clippy configuration"
    ]
    constitution: [
      "Zero unwrap law: NEVER use .unwrap or .expect",
      "Test first: Tests MUST exist before implementation"
    ]
  }
}
', 'closed', 1, 'feature', '2026-02-19T14:01:16.998146176Z', 'lewis', '2026-02-19T19:17:25.852107027Z', '2026-02-19T19:17:25.852096797Z', 'done', '.', 0, 0, '', '', '');
REPLACE INTO issues (`id`, `title`, `description`, `status`, `priority`, `issue_type`, `created_at`, `created_by`, `updated_at`, `source_repo`, `compaction_level`, `original_size`, `design`, `acceptance_criteria`, `notes`) VALUES ('bd-zv6', 'graph: Fix handler_name alias mapping to target instead of handler', 'In src/graph/mod.rs:125, the key alias mapping incorrectly maps handler_name to target:

Current code:
"targetService" | "service_name" | "handler_name" => Some("target"),

The problem: handler_name should map to "handler" (not "target") for ObjectCall nodes which have a handler field (see ObjectCallConfig in workflow_node.rs line 89).

This causes data loss when users set handler_name in the UI - it gets stored as target instead of handler.

Fix: Remove handler_name from this match arm and add a separate mapping:
"handler_name" => Some("handler"),', 'open', 1, 'bug', '2026-03-05T11:47:24.473757155Z', 'lewis', '2026-03-05T11:47:24.473757155Z', '.', 0, 0, '', '', '');
COMMIT;
