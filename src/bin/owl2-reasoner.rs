//! OWL2 Reasoner CLI - Demo Application
//!
//! A command-line tool for reasoning with OWL2 ontologies.
//!
//! Usage:
//!   cargo run --bin owl2-reasoner -- <command> [options] <ontology_file>
//!
//! Commands:
//!   check       Check ontology consistency
//!   check-auto  Check with automatic reasoner selection
//!   convert     Convert OWL to binary format
//!   stats       Show ontology statistics
//!   compare     Compare Sequential vs SPACL performance
//!
//! Examples:
//!   cargo run --bin owl2-reasoner -- check tests/data/univ-bench.owl
//!   cargo run --bin owl2-reasoner -- stats tests/data/univ-bench.owl
//!   cargo run --bin owl2-reasoner -- compare tests/data/univ-bench.owl

use std::env;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use owl2_reasoner::{
    detect_profile, select_consistency_reasoner, serializer::BinaryOntologyFormat,
    util::ontology_io::load_ontology_with_env, BranchPolicyMode, ConsistencyReasoner, Ontology,
    OntologyCharacteristics, SchedulingMode, SimpleReasoner, SpeculativeConfig,
    SpeculativeTableauxReasoner,
};

fn flush_stdout() {
    let _ = std::io::stdout().flush();
}

fn phase_log(label: &str, detail: &str) {
    eprintln!("[phase] {} {}", label, detail);
}

fn env_truthy(key: &str) -> bool {
    match env::var(key) {
        Ok(value) => {
            let value = value.trim().to_ascii_lowercase();
            !(value.is_empty() || value == "0" || value == "false" || value == "no")
        }
        Err(_) => false,
    }
}

fn scheduling_mode_from_env() -> Result<SchedulingMode, String> {
    match env::var("SPACL_SCHED_MODE") {
        Ok(value) => match value.trim().to_ascii_lowercase().as_str() {
            "" | "adaptive" => Ok(SchedulingMode::Adaptive),
            "sequential" => Ok(SchedulingMode::Sequential),
            "always_parallel" | "always-parallel" | "parallel" | "forced_parallel" => {
                Ok(SchedulingMode::AlwaysParallel)
            }
            other => Err(format!(
                "unsupported SPACL_SCHED_MODE='{}' (expected adaptive|sequential|always_parallel)",
                other
            )),
        },
        Err(_) => Ok(SchedulingMode::Adaptive),
    }
}

fn branch_policy_from_env() -> Result<BranchPolicyMode, String> {
    match env::var("SPACL_BRANCH_POLICY") {
        Ok(value) => match value.trim().to_ascii_lowercase().as_str() {
            "" | "baseline" | "default" => Ok(BranchPolicyMode::Baseline),
            "heuristic" | "ranked" => Ok(BranchPolicyMode::Heuristic),
            "hybrid_rrn" | "hybrid-rrn" | "rrn" => Ok(BranchPolicyMode::HybridRrn),
            other => Err(format!(
                "unsupported SPACL_BRANCH_POLICY='{}' (expected baseline|heuristic|hybrid_rrn)",
                other
            )),
        },
        Err(_) => Ok(BranchPolicyMode::Baseline),
    }
}

fn speculative_config_from_env() -> Result<SpeculativeConfig, String> {
    let mut config = SpeculativeConfig::default();
    config.scheduling_mode = scheduling_mode_from_env()?;
    config.branch_policy = branch_policy_from_env()?;
    if let Ok(path) = env::var("SPACL_RRN_MODEL_PATH") {
        let path = path.trim();
        if !path.is_empty() {
            config.rrn_model_path = Some(path.to_string());
        }
    }
    if let Ok(path) = env::var("SPACL_BRANCH_SNAPSHOT_FILE") {
        let path = path.trim();
        if !path.is_empty() {
            config.branch_snapshot_path = Some(path.to_string());
        }
    }
    if env::var("SPACL_NOGOOD").is_ok() {
        config.enable_learning = env_truthy("SPACL_NOGOOD");
    }
    Ok(config)
}

fn emit_spacl_stats(reasoner: &SpeculativeTableauxReasoner) {
    if !env_truthy("SPACL_EMIT_STATS") {
        return;
    }
    let stats = reasoner.get_stats();
    eprintln!(
        "[spacl] mode={} branch_policy={} used_parallel={} branches_created={} work_items_expanded={} branches_pruned={} nogood_hits={} local_cache_hits={} global_cache_hits={} steal_attempts={} steal_successes={} policy_reordered_splits={} policy_fallbacks={} hybrid_policy_calls={} hybrid_model_calls={} branch_snapshots_written={}",
        stats.scheduling_mode,
        stats.branch_policy,
        stats.used_parallel,
        stats.branches_created,
        stats.work_items_expanded,
        stats.branches_pruned,
        stats.nogood_hits,
        stats.local_cache_hits,
        stats.global_cache_hits,
        stats.steal_attempts,
        stats.steal_successes,
        stats.policy_reordered_splits,
        stats.policy_fallbacks,
        stats.hybrid_policy_calls,
        stats.hybrid_model_calls,
        stats.branch_snapshots_written
    );
}

fn print_usage() {
    println!("OWL2 Reasoner CLI - Demo Application");
    println!();
    println!("Usage: owl2-reasoner <command> [options] <ontology_file>");
    println!();
    println!("Commands:");
    println!("  check <file>       Check ontology consistency");
    println!("  check-auto <file>  Check with automatic reasoner selection");
    println!("  convert <in> <out> Convert OWL to binary format (.owlbin)");
    println!("  stats <file>       Show ontology statistics");
    println!("  compare <file>     Compare Sequential vs SPACL performance");
    println!("  help               Show this help message");
    println!();
    println!("Options:");
    println!("  -v, --verbose      Verbose output");
    println!();
    println!("Environment (large ontology / production):");
    println!("  OWL2_REASONER_LARGE_PARSE=1   Enable large-file parser mode");
    println!("  OWL2_REASONER_AUTO_CACHE=1    Parse text and write .owlbin cache");
    println!("  OWL2_REASONER_FORCE_TEXT=1    Ignore .owlbin and parse source text");
    println!("  OWL2_REASONER_BIN_ONLY=1      Require .owlbin input/cache");
    println!("  OWL2_REASONER_MAX_FILE_SIZE   Override parser max file size in bytes");
    println!("  OWL2_REASONER_LARGE_PROFILE_AUTO=0  Disable auto large-file text profile");
    println!(
        "  OWL2_REASONER_LARGE_PROFILE_THRESHOLD=<bytes>  Auto profile threshold (default 4MB)"
    );
    println!(
        "  OWL2_REASONER_STRUCTURAL_XML_AUTO=0  Disable auto structural RDF/XML for large files"
    );
    println!(
        "  OWL2_REASONER_STRUCTURAL_XML_AUTO_THRESHOLD=<bytes>  Auto mode threshold (default 4MB)"
    );
    println!("  OWL2_REASONER_EXPERIMENTAL_XML_PARSER=1  Enable experimental RDF/XML pipeline");
    println!("  OWL2_REASONER_EXPERIMENTAL_XML_STRICT=1  Fail if unsupported experimental terms are skipped");
    println!("  OWL2_REASONER_BIN_FORMAT=v1   Write legacy .owlbin format (default v2)");
    println!("  SPACL_SCHED_MODE=adaptive|sequential|always_parallel");
    println!("  SPACL_BRANCH_POLICY=baseline|heuristic|hybrid_rrn");
    println!("  SPACL_RRN_MODEL_PATH=<path>   JSON model for hybrid_rrn branch ranking");
    println!("  SPACL_BRANCH_SNAPSHOT_FILE=<path>   Export branch-level policy snapshots (.jsonl)");
    println!("  SPACL_NOGOOD=0|1             Disable/enable nogood learning for ablations");
    println!("  SPACL_EMIT_STATS=1           Emit one-line SPACL telemetry after reasoning");
    println!();
    println!("Examples:");
    println!("  OWL2_REASONER_LARGE_PARSE=1 owl2-reasoner check large.owl");
    println!("  OWL2_REASONER_AUTO_CACHE=1 owl2-reasoner check large.owl");
    println!("  owl2-reasoner check tests/data/univ-bench.owl");
    println!("  owl2-reasoner check-auto large.owl");
    println!("  owl2-reasoner convert large.owl large.owlbin");
    println!("  owl2-reasoner check large.owlbin");
    println!("  owl2-reasoner stats tests/data/univ-bench.owl");
    println!("  owl2-reasoner compare tests/data/univ-bench.owl");
}

fn load_ontology(path: &str) -> Result<Arc<Ontology>, String> {
    let path = Path::new(path);
    println!("Loading ontology: {}", path.display());
    flush_stdout();
    phase_log("parse_start", &format!("file={}", path.display()));
    let start = Instant::now();
    let ontology = load_ontology_with_env(path).map_err(|e| format!("{}", e))?;
    let load_time = start.elapsed();
    println!("✓ Loaded in {:?}", load_time);
    flush_stdout();
    phase_log("parse_done", &format!("ms={}", load_time.as_millis()));
    Ok(Arc::new(ontology))
}

fn cmd_check_auto(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner check-auto <ontology_file>");
        return;
    }

    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let profile = detect_profile(&ontology);
    let analysis = OntologyCharacteristics::analyze(&ontology);
    let decision = select_consistency_reasoner(&ontology, profile);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║           ONTOLOGY ANALYSIS & REASONER SELECTION             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Statistics:");
    println!("  Classes:          {}", analysis.class_count);
    println!("  Properties:       {}", analysis.property_count);
    println!("  Axioms:           {}", ontology.axioms().len());
    println!("  Disjunctions:     {}", analysis.disjunction_count);
    println!("  Complexity:       {}", analysis.complexity_description());
    println!();
    println!("Characteristics:");
    println!(
        "  Hierarchy fast path: {}",
        if owl2_reasoner::HierarchicalClassificationEngine::can_handle(&ontology) {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "  Detected profile: {}",
        profile.map_or("OWL2 DL".to_string(), |p| p.name().to_string())
    );
    println!();

    let start = Instant::now();
    phase_log(
        "reason_start",
        &format!("reasoner={}", decision.reasoner.as_str()),
    );
    let consistent = match decision.reasoner {
        ConsistencyReasoner::Simple => {
            println!(
                "Selected: {} ({})",
                decision.reasoner.as_str(),
                decision.rationale
            );
            let reasoner = SimpleReasoner::from_arc(Arc::clone(&ontology));
            reasoner.is_consistent()
        }
        ConsistencyReasoner::Speculative => {
            println!(
                "Selected: {} ({})",
                decision.reasoner.as_str(),
                decision.rationale
            );
            let config = match speculative_config_from_env() {
                Ok(config) => config,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return;
                }
            };
            let mut reasoner =
                SpeculativeTableauxReasoner::with_config_arc(Arc::clone(&ontology), config);
            let result = reasoner.is_consistent();
            emit_spacl_stats(&reasoner);
            result
        }
    };

    let check_time = start.elapsed();

    match consistent {
        Ok(result) => {
            println!("\n✓ Consistency check complete in {:?}", check_time);
            flush_stdout();
            phase_log(
                "reason_done",
                &format!(
                    "ms={} result={}",
                    check_time.as_millis(),
                    if result { "consistent" } else { "inconsistent" }
                ),
            );
            println!();
            if result {
                println!("Result: ✅ CONSISTENT");
            } else {
                println!("Result: ❌ INCONSISTENT");
            }
        }
        Err(e) => {
            eprintln!("Error during reasoning: {:?}", e);
        }
    }
}

fn cmd_convert(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Error: Need input and output files");
        println!("Usage: owl2-reasoner convert <input.owl> <output.owlbin>");
        return;
    }

    let input_path = &args[0];
    let output_path = &args[1];

    // Load ontology from OWL
    let ontology = match load_ontology(input_path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Serialize to binary
    println!("\nConverting to binary format...");
    let start = Instant::now();

    let mut file = match std::fs::File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            return;
        }
    };

    match BinaryOntologyFormat::serialize(&ontology, &mut file) {
        Ok(()) => {
            let convert_time = start.elapsed();
            println!("✓ Conversion complete in {:?}", convert_time);

            // Show file size comparison
            let input_meta = std::fs::metadata(input_path).ok();
            let output_meta = std::fs::metadata(output_path).ok();

            if let (Some(in_meta), Some(out_meta)) = (input_meta, output_meta) {
                let in_size = in_meta.len();
                let out_size = out_meta.len();
                let ratio = out_size as f64 / in_size as f64;

                println!();
                println!("File sizes:");
                println!(
                    "  Input:  {} bytes ({:.2} MB)",
                    in_size,
                    in_size as f64 / 1_048_576.0
                );
                println!(
                    "  Output: {} bytes ({:.2} MB)",
                    out_size,
                    out_size as f64 / 1_048_576.0
                );
                println!("  Ratio:  {:.1}%", ratio * 100.0);
            }

            println!();
            println!("Usage: owl2-reasoner check {}", output_path);
        }
        Err(e) => {
            eprintln!("Error during conversion: {}", e);
        }
    }
}

fn cmd_check(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner check <ontology_file>");
        return;
    }

    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("\nChecking consistency...");
    flush_stdout();
    let start = Instant::now();
    phase_log("reason_start", "reasoner=SpeculativeTableauxReasoner");

    let config = match speculative_config_from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    let mut reasoner = SpeculativeTableauxReasoner::with_config_arc(ontology, config);
    match reasoner.is_consistent() {
        Ok(consistent) => {
            emit_spacl_stats(&reasoner);
            let check_time = start.elapsed();
            println!("✓ Consistency check complete in {:?}", check_time);
            flush_stdout();
            phase_log(
                "reason_done",
                &format!(
                    "ms={} result={}",
                    check_time.as_millis(),
                    if consistent {
                        "consistent"
                    } else {
                        "inconsistent"
                    }
                ),
            );
            println!();
            if consistent {
                println!("Result: ✅ CONSISTENT");
            } else {
                println!("Result: ❌ INCONSISTENT");
            }
        }
        Err(e) => {
            emit_spacl_stats(&reasoner);
            eprintln!("Error during reasoning: {:?}", e);
        }
    }
}

fn cmd_stats(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner stats <ontology_file>");
        return;
    }

    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let analysis = OntologyCharacteristics::analyze(&ontology);
    let profile = detect_profile(&ontology);
    let consistency_plan = select_consistency_reasoner(&ontology, profile);

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                  ONTOLOGY STATISTICS                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Basic Counts:");
    println!("  Classes:             {}", ontology.classes().len());
    println!(
        "  Object Properties:   {}",
        ontology.object_properties().len()
    );
    println!(
        "  Data Properties:     {}",
        ontology.data_properties().len()
    );
    println!(
        "  Named Individuals:   {}",
        ontology.named_individuals().len()
    );
    println!("  Total Axioms:        {}", ontology.axioms().len());
    println!();

    println!("Analysis:");
    println!("  Disjunctions:        {}", analysis.disjunction_count);
    println!(
        "  Tree-like hierarchy: {}",
        if analysis.is_tree_like { "Yes" } else { "No" }
    );
    println!("  Complexity score:    {:.3}", analysis.complexity_score);
    println!(
        "  Detected profile:    {}",
        profile.map_or("OWL2 DL".to_string(), |p| p.name().to_string())
    );
    println!();

    println!(
        "Recommended consistency reasoner: {} ({})",
        consistency_plan.reasoner.as_str(),
        consistency_plan.rationale
    );
}

fn cmd_compare(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No ontology file specified");
        println!("Usage: owl2-reasoner compare <ontology_file>");
        return;
    }

    let path = &args[0];
    let ontology = match load_ontology(path) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              REASONER PERFORMANCE COMPARISON                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Shared ontology for both reasoners (no deep clone)
    let ontology = Arc::clone(&ontology);

    // SimpleReasoner
    println!("Running SimpleReasoner...");
    let start = Instant::now();
    let simple_reasoner = SimpleReasoner::from_arc(Arc::clone(&ontology));
    let simple_result = simple_reasoner.is_consistent();
    let simple_time = start.elapsed();
    println!("  Result: {:?} in {:?}", simple_result, simple_time);
    println!();

    // SpeculativeTableauxReasoner
    println!("Running SpeculativeTableauxReasoner (SPACL)...");
    let start = Instant::now();
    let config = match speculative_config_from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    let mut speculative_reasoner =
        SpeculativeTableauxReasoner::with_config_arc(Arc::clone(&ontology), config);
    let speculative_result = speculative_reasoner.is_consistent();
    emit_spacl_stats(&speculative_reasoner);
    let speculative_time = start.elapsed();
    println!(
        "  Result: {:?} in {:?}",
        speculative_result, speculative_time
    );
    println!();

    // Summary
    println!("Summary:");
    println!("  SimpleReasoner:    {:?}", simple_time);
    println!("  SPACL:             {:?}", speculative_time);

    let speedup = simple_time.as_secs_f64() / speculative_time.as_secs_f64();
    if speedup > 1.0 {
        println!("  Speedup:           {:.2}x faster (SPACL)", speedup);
    } else {
        println!("  Speedup:           {:.2}x faster (Simple)", 1.0 / speedup);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    let command_args = &args[2..];

    match command.as_str() {
        "check" => cmd_check(command_args),
        "check-auto" => cmd_check_auto(command_args),
        "convert" => cmd_convert(command_args),
        "stats" => cmd_stats(command_args),
        "compare" => cmd_compare(command_args),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
        }
    }
}
