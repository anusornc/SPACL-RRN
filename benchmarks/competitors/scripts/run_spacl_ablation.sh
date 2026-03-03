#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCHMARK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$BENCHMARK_DIR/../.." && pwd)"
RESULTS_DIR="$BENCHMARK_DIR/results/history"

RUN_ID="${RUN_ID:-spacl_ablation_$(date +%Y%m%d_%H%M%S)}"
RUN_DIR="$RESULTS_DIR/$RUN_ID"
REPEATS="${REPEATS:-3}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-300}"
SKIP_BUILD="${SKIP_BUILD:-0}"

# Default workload mix:
# - branch-heavy synthetic disjunctive cases
# - branch-light hierarchy controls
DEFAULT_WORKLOADS=(
  "benchmarks/ontologies/disjunctive/disjunctive_10k.owl"
  "benchmarks/ontologies/disjunctive/disjunctive_30k.owl"
  "tests/data/hierarchy_10000.owl"
  "tests/data/hierarchy_100000.owl"
  "tests/data/univ-bench.owl"
)

# mode|nogood_enabled
DEFAULT_MODE_MATRIX=(
  "sequential|0"
  "adaptive|0"
  "adaptive|1"
  "always_parallel|0"
)

log() {
  printf '[spacl-ablation] %s\n' "$*"
}

truthy() {
  local value
  value="$(echo "${1:-}" | tr '[:upper:]' '[:lower:]' | xargs)"
  case "$value" in
    ""|"0"|"false"|"no") return 1 ;;
    *) return 0 ;;
  esac
}

ensure_run_dir() {
  mkdir -p "$RUN_DIR/logs"
}

resolve_workloads() {
  if [[ -n "${WORKLOADS:-}" ]]; then
    IFS=',' read -r -a WORKLOAD_LIST <<< "$WORKLOADS"
  else
    WORKLOAD_LIST=("${DEFAULT_WORKLOADS[@]}")
  fi

  local resolved=()
  local workload
  for workload in "${WORKLOAD_LIST[@]}"; do
    workload="${workload#"${workload%%[![:space:]]*}"}"
    workload="${workload%"${workload##*[![:space:]]}"}"
    [[ -z "$workload" ]] && continue
    if [[ -f "$PROJECT_ROOT/$workload" ]]; then
      resolved+=("$PROJECT_ROOT/$workload")
    elif [[ -f "$workload" ]]; then
      resolved+=("$workload")
    else
      log "missing workload: $workload"
      exit 1
    fi
  done
  WORKLOAD_LIST=("${resolved[@]}")
}

resolve_modes() {
  if [[ -n "${MODE_MATRIX:-}" ]]; then
    IFS=',' read -r -a MODE_LIST <<< "$MODE_MATRIX"
  else
    MODE_LIST=("${DEFAULT_MODE_MATRIX[@]}")
  fi
}

build_binary() {
  if [[ "$SKIP_BUILD" == "1" ]]; then
    return
  fi
  log "building owl2-reasoner"
  cargo build --bin owl2-reasoner
}

write_metadata() {
  cat > "$RUN_DIR/README.md" <<EOF
# SPACL Scheduler Ablation

- Run ID: \`$RUN_ID\`
- Repeats: \`$REPEATS\`
- Timeout seconds: \`$TIMEOUT_SECONDS\`
- Binary path: \`$PROJECT_ROOT/target/debug/owl2-reasoner\`
- Workloads: \`${#WORKLOAD_LIST[@]}\`
- Modes: \`${#MODE_LIST[@]}\`

This runner forces \`owl2-reasoner check\` to keep SPACL active. It does not use \`check-auto\`.
EOF

  {
    echo "repeat,mode,nogood_enabled,workload,status,wall_time_ms,parse_time_ms,reason_time_ms,used_parallel,branches_created,work_items_expanded,branches_pruned,nogood_hits,local_cache_hits,global_cache_hits,steal_attempts,steal_successes,log_file"
  } > "$RUN_DIR/results.csv"
}

extract_field() {
  local key="$1"
  local log_file="$2"
  local value
  value="$(grep -Eo "${key}=[^[:space:]]+" "$log_file" | tail -1 | cut -d= -f2 || true)"
  if [[ -z "$value" ]]; then
    value=""
  fi
  printf '%s' "$value"
}

run_one() {
  local repeat="$1"
  local mode="$2"
  local nogood="$3"
  local workload_path="$4"
  local workload_name log_file
  workload_name="$(basename "$workload_path")"
  log_file="$RUN_DIR/logs/${repeat}_${mode}_nogood${nogood}_${workload_name}.log"

  log "run repeat=$repeat mode=$mode nogood=$nogood workload=$workload_name"

  local start_ns end_ns wall_ms status parse_ms reason_ms
  start_ns="$(date +%s%N)"

  if timeout --kill-after=5s "${TIMEOUT_SECONDS}s" \
    env \
      OWL2_REASONER_LARGE_PARSE="${OWL2_REASONER_LARGE_PARSE:-1}" \
      SPACL_SCHED_MODE="$mode" \
      SPACL_NOGOOD="$nogood" \
      SPACL_EMIT_STATS=1 \
      "$PROJECT_ROOT/target/debug/owl2-reasoner" check "$workload_path" \
      >"$log_file" 2>&1; then
    status="success"
  else
    local exit_code=$?
    if [[ "$exit_code" -eq 124 || "$exit_code" -eq 137 ]]; then
      status="timeout"
    else
      status="failed"
    fi
  fi

  end_ns="$(date +%s%N)"
  wall_ms=$(( (end_ns - start_ns) / 1000000 ))
  parse_ms="$(grep -Eo '\[phase\][[:space:]]+parse_done[[:space:]]+ms=[0-9]+' "$log_file" | tail -1 | grep -Eo '[0-9]+' || true)"
  reason_ms="$(grep -Eo '\[phase\][[:space:]]+reason_done[[:space:]]+ms=[0-9]+' "$log_file" | tail -1 | grep -Eo '[0-9]+' || true)"
  [[ -z "$parse_ms" ]] && parse_ms=-1
  [[ -z "$reason_ms" ]] && reason_ms=-1

  local used_parallel branches_created work_items_expanded branches_pruned nogood_hits
  local local_cache_hits global_cache_hits steal_attempts steal_successes
  used_parallel="$(extract_field "used_parallel" "$log_file")"
  branches_created="$(extract_field "branches_created" "$log_file")"
  work_items_expanded="$(extract_field "work_items_expanded" "$log_file")"
  branches_pruned="$(extract_field "branches_pruned" "$log_file")"
  nogood_hits="$(extract_field "nogood_hits" "$log_file")"
  local_cache_hits="$(extract_field "local_cache_hits" "$log_file")"
  global_cache_hits="$(extract_field "global_cache_hits" "$log_file")"
  steal_attempts="$(extract_field "steal_attempts" "$log_file")"
  steal_successes="$(extract_field "steal_successes" "$log_file")"

  [[ -z "$used_parallel" ]] && used_parallel=""
  [[ -z "$branches_created" ]] && branches_created=-1
  [[ -z "$work_items_expanded" ]] && work_items_expanded=-1
  [[ -z "$branches_pruned" ]] && branches_pruned=-1
  [[ -z "$nogood_hits" ]] && nogood_hits=-1
  [[ -z "$local_cache_hits" ]] && local_cache_hits=-1
  [[ -z "$global_cache_hits" ]] && global_cache_hits=-1
  [[ -z "$steal_attempts" ]] && steal_attempts=-1
  [[ -z "$steal_successes" ]] && steal_successes=-1

  printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n' \
    "$repeat" \
    "$mode" \
    "$nogood" \
    "$workload_name" \
    "$status" \
    "$wall_ms" \
    "$parse_ms" \
    "$reason_ms" \
    "$used_parallel" \
    "$branches_created" \
    "$work_items_expanded" \
    "$branches_pruned" \
    "$nogood_hits" \
    "$local_cache_hits" \
    "$global_cache_hits" \
    "$steal_attempts" \
    "$steal_successes" \
    "logs/$(basename "$log_file")" \
    >> "$RUN_DIR/results.csv"
}

main() {
  ensure_run_dir
  resolve_workloads
  resolve_modes
  build_binary
  write_metadata

  local repeat mode_entry mode nogood workload
  for ((repeat = 1; repeat <= REPEATS; repeat++)); do
    for mode_entry in "${MODE_LIST[@]}"; do
      mode="${mode_entry%%|*}"
      nogood="${mode_entry##*|}"
      for workload in "${WORKLOAD_LIST[@]}"; do
        run_one "$repeat" "$mode" "$nogood" "$workload"
      done
    done
  done

  log "results written to $RUN_DIR/results.csv"
}

main "$@"
