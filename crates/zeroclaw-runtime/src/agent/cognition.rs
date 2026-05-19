use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;

/// A behavioral module loaded from markdown files with YAML frontmatter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorModule {
    pub name: String,
    pub priority: u32,
    pub contexts: Vec<String>,
    pub token_cost: u32,
    pub dependencies: Vec<String>,
    pub content: String,
}

/// Dynamic progression state supporting matrix modeling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechDimension {
    pub theory: u32,
    pub implementation: u32,
    pub deployment: u32,
    pub debugging: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGraph {
    pub technologies: HashMap<String, TechDimension>,
}

impl Default for CapabilityGraph {
    fn default() -> Self {
        let mut tech = HashMap::new();
        tech.insert("rust".to_string(), TechDimension { theory: 20, implementation: 15, deployment: 10, debugging: 10 });
        tech.insert("docker".to_string(), TechDimension { theory: 15, implementation: 10, deployment: 5, debugging: 5 });
        tech.insert("git".to_string(), TechDimension { theory: 30, implementation: 25, deployment: 15, debugging: 20 });
        Self { technologies: tech }
    }
}

/// Conversation memory tracking failure logs, modes, and rolling query history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPattern {
    pub recent_queries: Vec<String>,
    pub failure_patterns: Vec<String>,
    pub recent_modes: Vec<String>,
    pub topic_drift_detected: bool,
    pub consecutive_failures: u32,
    pub consecutive_academic: u32,
    pub execution_pressure_level: u32,
    pub debug_guidance_level: u32,
}

impl Default for ConversationPattern {
    fn default() -> Self {
        Self {
            recent_queries: Vec::new(),
            failure_patterns: Vec::new(),
            recent_modes: Vec::new(),
            topic_drift_detected: false,
            consecutive_failures: 0,
            consecutive_academic: 0,
            execution_pressure_level: 0,
            debug_guidance_level: 0,
        }
    }
}

/// Event bus events that alter cognitive state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CognitionEvent {
    CompilerFailure,
    ProjectStalled,
    OverResearchDetected,
    ExecutionImproved,
    FocusDriftDetected,
    NormalQuery(String),
}

// Parse YAML-like frontmatter out of behavior markdown files.
pub fn parse_behavior_module(content: &str) -> Option<BehaviorModule> {
    if !content.starts_with("---") {
        return None;
    }

    let mut parts = content.splitn(3, "---");
    let _empty_header = parts.next()?;
    let header_content = parts.next()?;
    let markdown_body = parts.next()?.trim().to_string();

    let mut name = String::new();
    let mut priority = 5;
    let mut contexts = Vec::new();
    let mut token_cost = 100;
    let mut dependencies = Vec::new();

    for line in header_content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim();
            let val = val.trim();
            match key {
                "name" => name = val.to_string(),
                "priority" => priority = val.parse().unwrap_or(5),
                "token_cost" => token_cost = val.parse().unwrap_or(100),
                "contexts" => {
                    let cleaned = val.replace('[', "").replace(']', "");
                    contexts = cleaned.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                }
                "dependencies" => {
                    let cleaned = val.replace('[', "").replace(']', "");
                    dependencies = cleaned.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                }
                _ => {}
            }
        }
    }

    if name.is_empty() {
        return None;
    }

    Some(BehaviorModule {
        name,
        priority,
        contexts,
        token_cost,
        dependencies,
        content: markdown_body,
    })
}

/// Write default directories and behavioral micro-modules.
pub fn scaffold_cognition_system(workspace_dir: &Path) {
    let behavior_dir = workspace_dir.join("behavior");
    let folders = ["mentor_engine", "build_engine", "debug_engine", "research_engine", "focus_engine"];
    
    for folder in &folders {
        let path = behavior_dir.join(folder);
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
    }

    // Default modules with metadata
    let default_files = [
        ("mentor_engine/questioning.md", 
         "---\nname: questioning\npriority: 9\ncontexts: [mentor]\ntoken_cost: 150\ndependencies: [tradeoffs]\n---\n\
          #### Socratic Questioning Rules\n\
          - Never provide full code answers directly.\n\
          - Ask targeted Socratic questions to guide the user's reasoning.\n\
          - Ask: \"What do you think is causing this?\", \"How would you structure the inputs?\"\n"),
          
        ("mentor_engine/systems_thinking.md",
         "---\nname: systems_thinking\npriority: 8\ncontexts: [mentor, research]\ntoken_cost: 200\ndependencies: []\n---\n\
          #### Systems Thinking Rules\n\
          - Push the user to map out data flows and component boundaries.\n\
          - Ask for design schemas or API boundaries before writing logic.\n"),
          
        ("mentor_engine/tradeoffs.md",
         "---\nname: tradeoffs\npriority: 7\ncontexts: [mentor, research]\ntoken_cost: 180\ndependencies: []\n---\n\
          #### Tradeoff Analysis Rules\n\
          - When the user proposes a solution, ask about time/space complexity or maintenance costs.\n\
          - Require comparison of alternative designs.\n"),
          
        ("mentor_engine/creativity.md",
         "---\nname: creativity\npriority: 6\ncontexts: [mentor]\ntoken_cost: 120\ndependencies: []\n---\n\
          #### Creativity Under Constraints\n\
          - Push for minimal code footprint.\n\
          - Encourage creative usage of existing tools or libraries before pulling in new dependencies.\n"),
          
        ("build_engine/mvp.md",
         "---\nname: mvp\npriority: 10\ncontexts: [build]\ntoken_cost: 150\ndependencies: [execution]\n---\n\
          #### MVP Enforcement\n\
          - Prioritize a working prototype over secondary feature polish.\n\
          - Define a single core feature and verify it works before adding complexity.\n"),
          
        ("build_engine/execution.md",
         "---\nname: execution\npriority: 9\ncontexts: [build]\ntoken_cost: 200\ndependencies: []\n---\n\
          #### Execution Rules\n\
          - Socratic mode is suspended. Provide clear, direct, and actionable code/commands.\n\
          - Output complete code blocks for modified files when implementing.\n"),
          
        ("build_engine/optimization.md",
         "---\nname: optimization\npriority: 5\ncontexts: [build]\ntoken_cost: 130\ndependencies: []\n---\n\
          #### Optimization Pacing\n\
          - Do not optimize prematurely. Wait for profile data or user requests.\n\
          - Keep components simple and readable first.\n"),
          
        ("debug_engine/diagnostics.md",
         "---\nname: diagnostics\npriority: 10\ncontexts: [debug]\ntoken_cost: 180\ndependencies: [hypothesis]\n---\n\
          #### Diagnostic Rules\n\
          - Prioritize compiler outputs, stack traces, and system error codes.\n\
          - Ask the user to run diagnostic commands (`cargo check`, `go test`, debug runs) first.\n"),
          
        ("debug_engine/hypothesis.md",
         "---\nname: hypothesis\npriority: 9\ncontexts: [debug]\ntoken_cost: 150\ndependencies: []\n---\n\
          #### Debugging Hypothesis\n\
          - Ask the user: \"What is your hypothesis for why this error occurs?\"\n\
          - Trace execution flow step-by-step to isolate the failure point.\n"),
          
        ("research_engine/signal_vs_hype.md",
         "---\nname: signal_vs_hype\npriority: 8\ncontexts: [research]\ntoken_cost: 150\ndependencies: []\n---\n\
          #### Signal vs Hype Filter\n\
          - Filter out short-term tech hype cycles.\n\
          - Recommend stable, standard library, or widely adopted industry solutions.\n"),
          
        ("research_engine/tradeoffs.md",
         "---\nname: tradeoffs_research\npriority: 7\ncontexts: [research]\ntoken_cost: 160\ndependencies: []\n---\n\
          #### Research Tradeoff Mapping\n\
          - Map latency, licensing, maintainability, and vendor lock-in trade-offs.\n"),
          
        ("research_engine/comparisons.md",
         "---\nname: comparisons\npriority: 6\ncontexts: [research]\ntoken_cost: 140\ndependencies: []\n---\n\
          #### Comparative Framework Rules\n\
          - Avoid generic 'A vs B' lists.\n\
          - Use structured feature matrices mapping security, complexity, and resource footprint.\n\
          - Focus on ecosystem support, integration effort, and operational complexity.\n"),

        ("focus_engine/domains.md",
         "---\nname: domains\npriority: 8\ncontexts: [focus]\ntoken_cost: 120\ndependencies: []\n---\n\
          #### Focus Domain Isolation\n\
          - Enforce focus on maximum 2 active domains/directories.\n\
          - Prevent tool/context hopping.\n"),
          
        ("focus_engine/pacing.md",
         "---\nname: pacing\npriority: 7\ncontexts: [focus]\ntoken_cost: 130\ndependencies: []\n---\n\
          #### Pacing & Recovery\n\
          - Monitor user fatigue: if they repeat errors, suggest a short break.\n")
    ];

    for &(subpath, content) in &default_files {
        let full_path = behavior_dir.join(subpath);
        if !full_path.exists() {
            let _ = fs::write(full_path, content);
        }
    }
}

/// Loads pattern state from `behavior/conversation_pattern.json`.
pub fn load_pattern_state(workspace_dir: &Path) -> ConversationPattern {
    let path = workspace_dir.join("behavior").join("conversation_pattern.json");
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(state) = serde_json::from_str::<ConversationPattern>(&content) {
                return state;
            }
        }
    }
    ConversationPattern::default()
}

/// Saves pattern state to `behavior/conversation_pattern.json`.
pub fn save_pattern_state(workspace_dir: &Path, state: &ConversationPattern) {
    let path = workspace_dir.join("behavior").join("conversation_pattern.json");
    let _ = fs::create_dir_all(path.parent().unwrap());
    if let Ok(payload) = serde_json::to_string_pretty(state) {
        let _ = fs::write(path, payload);
    }
}

/// Loads capability graph from `behavior/progress_state.json`.
pub fn load_capability_graph(workspace_dir: &Path) -> CapabilityGraph {
    let path = workspace_dir.join("behavior").join("progress_state.json");
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(graph) = serde_json::from_str::<CapabilityGraph>(&content) {
                return graph;
            }
        }
    }
    // Fallback: create default
    let default_graph = CapabilityGraph::default();
    let _ = fs::create_dir_all(path.parent().unwrap());
    if let Ok(payload) = serde_json::to_string_pretty(&default_graph) {
        let _ = fs::write(path, payload);
    }
    default_graph
}

/// Saves capability graph to `behavior/progress_state.json`.
pub fn save_capability_graph(workspace_dir: &Path, graph: &CapabilityGraph) {
    let path = workspace_dir.join("behavior").join("progress_state.json");
    if let Ok(payload) = serde_json::to_string_pretty(graph) {
        let _ = fs::write(path, payload);
    }
}

/// Intent Classification Engine using context history + keyword weight heuristic.
pub fn classify_intent(workspace_dir: &Path, query: &str) -> String {
    let query_lower = query.to_lowercase();
    let pattern = load_pattern_state(workspace_dir);

    // Heuristic classification weights
    let mut debug_score = 0;
    let mut build_score = 0;
    let mut research_score = 0;
    let mut mentor_score = 0;

    // Debug keywords
    let debug_kws = ["error", "panic", "compile", "failed", "failed to", "warning", "exception", "exit code", "stderr", "traceback", "crash", "bug", "segfault", "undefined", "not found", "invalid"];
    for kw in &debug_kws {
        if query_lower.contains(kw) {
            debug_score += 5;
        }
    }

    // Build/execution keywords
    let build_kws = ["ship", "build", "prototype", "write code", "implement", "deploy", "run dev", "create project", "cargo build", "setup", "make a", "generate code", "refactor", "optimize"];
    for kw in &build_kws {
        if query_lower.contains(kw) {
            build_score += 5;
        }
    }

    // Research/analysis keywords
    let research_kws = ["what is", "explain", "compare", "tradeoff", "why does", "pros and cons", "paper", "literature", "research", "difference between", "how works", "architecture of"];
    for kw in &research_kws {
        if query_lower.contains(kw) {
            research_score += 5;
        }
    }

    // Mentor keywords
    let mentor_kws = ["guidance", "thinking", "strategy", "teach me", "guide", "how should i", "socratic", "learn"];
    for kw in &mentor_kws {
        if query_lower.contains(kw) {
            mentor_score += 5;
        }
    }

    // Integrate conversation context to avoid false-positive transient routing
    if let Some(last_mode) = pattern.recent_modes.last() {
        match last_mode.as_str() {
            "debug_mode" => debug_score += 2,
            "build_mode" => build_score += 2,
            "research_mode" => research_score += 2,
            "mentor_mode" => mentor_score += 2,
            _ => {}
        }
    }

    // Determine highest score (fallback to mentor_mode)
    let mut highest = "mentor_mode";
    let mut max_score = 1; // Minimum score threshold

    if debug_score > max_score {
        highest = "debug_mode";
        max_score = debug_score;
    }
    if build_score > max_score {
        highest = "build_mode";
        max_score = build_score;
    }
    if research_score > max_score {
        highest = "research_mode";
        max_score = research_score;
    }
    if mentor_score > max_score {
        highest = "mentor_mode";
    }

    highest.to_string()
}

/// Unified Event Bus for zero-overhead situational adjustments and skill transfer updates.
pub fn emit_cognition_event(workspace_dir: &Path, event: CognitionEvent) {
    let mut pattern = load_pattern_state(workspace_dir);
    let mut graph = load_capability_graph(workspace_dir);

    match event {
        CognitionEvent::CompilerFailure => {
            pattern.consecutive_failures += 1;
            pattern.consecutive_academic = 0;
            if !pattern.failure_patterns.contains(&"repeated_debug_failure".to_string()) {
                pattern.failure_patterns.push("repeated_debug_failure".to_string());
            }
            if pattern.consecutive_failures >= 3 {
                pattern.debug_guidance_level = (pattern.debug_guidance_level + 1).min(5);
            }
        }
        CognitionEvent::ProjectStalled => {
            pattern.execution_pressure_level = (pattern.execution_pressure_level + 1).min(5);
        }
        CognitionEvent::OverResearchDetected => {
            pattern.consecutive_academic += 1;
            if pattern.consecutive_academic >= 3 {
                pattern.execution_pressure_level = (pattern.execution_pressure_level + 1).min(5);
            }
        }
        CognitionEvent::ExecutionImproved => {
            pattern.consecutive_failures = 0;
            pattern.consecutive_academic = 0;
            pattern.execution_pressure_level = pattern.execution_pressure_level.saturating_sub(1);
            pattern.debug_guidance_level = pattern.debug_guidance_level.saturating_sub(1);
            
            // Skill Transfer: Rust implementation increases debugging and design!
            if let Some(rust) = graph.technologies.get_mut("rust") {
                rust.implementation = (rust.implementation + 1).min(100);
                rust.debugging = (rust.debugging + 1).min(100);
                rust.theory = (rust.theory + 1).min(100);
            }
        }
        CognitionEvent::FocusDriftDetected => {
            pattern.topic_drift_detected = true;
        }
        CognitionEvent::NormalQuery(query) => {
            // General query tracking
            pattern.recent_queries.push(query);
            if pattern.recent_queries.len() > 10 {
                pattern.recent_queries.remove(0);
            }
        }
    }

    save_pattern_state(workspace_dir, &pattern);
    save_capability_graph(workspace_dir, &graph);
}

/// Load and prioritize behavioral modules from the behavior registry under a specified token budget.
pub fn load_prioritized_modules(
    workspace_dir: &Path,
    active_mode: &str,
    token_budget: usize,
) -> String {
    let behavior_dir = workspace_dir.join("behavior");
    let mut modules = Vec::new();

    // Mapping mode name to subfolder name
    let engine_folder = match active_mode {
        "mentor_mode" => "mentor_engine",
        "build_mode" => "build_engine",
        "debug_mode" => "debug_engine",
        "research_mode" => "research_engine",
        "deep_work_mode" => "focus_engine",
        _ => "mentor_engine",
    };

    let target_dir = behavior_dir.join(engine_folder);
    if let Ok(entries) = fs::read_dir(target_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(module) = parse_behavior_module(&content) {
                        modules.push(module);
                    }
                }
            }
        }
    }

    // Focus engine rules can also be loaded dynamically as dependencies or helper modules if under token budget
    let focus_dir = behavior_dir.join("focus_engine");
    if active_mode != "deep_work_mode" {
        if let Ok(entries) = fs::read_dir(focus_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("md") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Some(module) = parse_behavior_module(&content) {
                            modules.push(module);
                        }
                    }
                }
            }
        }
    }

    // Sort modules by priority (highest first)
    modules.sort_by(|a, b| b.priority.cmp(&a.priority));

    // Resolve dependencies (simple check: if dependecy exists, ensure it is added/moved up)
    // For simplicity, we keep items sorted by priority. If a high priority module requires a dependency,
    // we make sure that dependency is loaded as well.
    let mut loaded_names = HashSet::new();
    let mut final_modules = Vec::new();
    let mut current_cost = 0;

    for module in modules {
        if current_cost + module.token_cost as usize > token_budget {
            continue; // Budget exceeded
        }

        // Add dependencies first
        for dep in &module.dependencies {
            if !loaded_names.contains(dep) {
                // Find dependency module in system
                if let Some(dep_mod) = find_module_by_name(&behavior_dir, dep) {
                    if current_cost + dep_mod.token_cost as usize <= token_budget {
                        current_cost += dep_mod.token_cost as usize;
                        loaded_names.insert(dep.clone());
                        final_modules.push(dep_mod);
                    }
                }
            }
        }

        if !loaded_names.contains(&module.name) {
            current_cost += module.token_cost as usize;
            loaded_names.insert(module.name.clone());
            final_modules.push(module);
        }
    }

    let mut result = String::new();
    for m in final_modules {
        result.push_str(&format!("\n### Module: {}\n", m.name));
        result.push_str(&m.content);
        result.push('\n');
    }
    result
}

fn find_module_by_name(behavior_dir: &Path, name: &str) -> Option<BehaviorModule> {
    let folders = ["mentor_engine", "build_engine", "debug_engine", "research_engine", "focus_engine"];
    for folder in &folders {
        let dir = behavior_dir.join(folder);
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("md") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Some(module) = parse_behavior_module(&content) {
                            if module.name == name {
                                return Some(module);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Cognitive Energy Balancer mode picker
pub fn get_cognitive_energy_mode(active_mode: &str, pattern: &ConversationPattern) -> String {
    // If active mode is build/execution, or execution pressure is high, balance with lightweight mode.
    if active_mode == "build_mode" || pattern.execution_pressure_level >= 3 {
        "lightweight_mode (Fast practical answers, minimized Socratic mentoring)".to_string()
    } else {
        "deep_reasoning_mode (Full Socratic systems analysis & engineering strategy)".to_string()
    }
}

/// Saves the latest query and triggers dynamic cognitive event routing.
pub fn save_last_query(workspace_dir: &Path, query: &str) {
    let last_query_path = workspace_dir.join("behavior").join("last_query.txt");
    let _ = fs::create_dir_all(last_query_path.parent().unwrap());
    let _ = fs::write(&last_query_path, query);

    // Emit normal query event
    emit_cognition_event(workspace_dir, CognitionEvent::NormalQuery(query.to_string()));

    // Event bus logic: detect compiler failures or panics in query
    let query_lower = query.to_lowercase();
    if query_lower.contains("error:") || query_lower.contains("panic!") || query_lower.contains("failed to compile") || query_lower.contains("exit status") || query_lower.contains("exit code") {
        emit_cognition_event(workspace_dir, CognitionEvent::CompilerFailure);
    }

    // Event bus logic: detect academic overload (too much research/explain questions without code changes)
    if query_lower.contains("explain") || query_lower.contains("compare") || query_lower.contains("what is") {
        emit_cognition_event(workspace_dir, CognitionEvent::OverResearchDetected);
    }

    // Event bus logic: check if the query indicates successful implementation or build success
    if query_lower.contains("build success") || query_lower.contains("test pass") || query_lower.contains("compiled successfully") || query_lower.contains("ship it") || query_lower.contains("works now") {
        emit_cognition_event(workspace_dir, CognitionEvent::ExecutionImproved);
    }
}
