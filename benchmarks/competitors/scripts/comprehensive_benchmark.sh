#!/bin/bash
#
# Comprehensive OWL2 Reasoner Benchmark
# Tests: HermiT, Pellet/Openllet, Tableauxx
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCHMARK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$BENCHMARK_DIR/../.." && pwd)"
RESULTS_DIR="$BENCHMARK_DIR/results"

mkdir -p "$RESULTS_DIR"

# Test ontologies
ONTOLOGIES=(
    "disjunctive_test.owl"
    "disjunctive_simple.owl"
    "univ-bench.owl"
    "hierarchy_100.owl"
    "hierarchy_1000.owl"
    "hierarchy_10000.owl"
    "hierarchy_100000.owl"
)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERR]${NC} $1"; }

# Check which reasoners are available
check_reasoners() {
    log_info "Checking available reasoners..."
    
    if docker images | grep -q "owl-reasoner-hermit"; then
        log_success "HermiT: Available"
        HAVE_HERMIT=1
    else
        log_warn "HermiT: Not found"
        HAVE_HERMIT=0
    fi
    
    if docker images | grep -q "owl-reasoner-pellet"; then
        log_success "Pellet/Openllet: Available"
        HAVE_PELLET=1
    else
        log_warn "Pellet: Not found"
        HAVE_PELLET=0
    fi
    
    if [ -f "$PROJECT_ROOT/target/release/owl2-reasoner" ]; then
        log_success "Tableauxx: Available"
        HAVE_TABLEAUXX=1
    else
        log_warn "Tableauxx: Binary not found"
        HAVE_TABLEAUXX=0
    fi
}

# Run single benchmark
run_benchmark() {
    local reasoner=$1
    local ontology=$2
    local ontology_path="$BENCHMARK_DIR/ontologies/$ontology"
    
    if [ ! -f "$ontology_path" ]; then
        log_warn "Ontology not found: $ontology"
        return
    fi
    
    log_info "Testing $reasoner on $ontology..."
    
    local result_file="$RESULTS_DIR/${reasoner}_${ontology%.owl}.json"
    local start_time end_time duration_ms status
    
    start_time=$(date +%s%N)
    
    case $reasoner in
        hermit)
            if [ $HAVE_HERMIT -eq 1 ]; then
                docker run --rm -v "$BENCHMARK_DIR/ontologies:/ontologies:ro" \
                    owl-reasoner-hermit "/ontologies/$ontology" consistency 2>&1 | tail -10 > "$result_file.tmp"
                status=$?
            else
                echo "HermiT not available" > "$result_file.tmp"
                status=1
            fi
            ;;
        pellet)
            if [ $HAVE_PELLET -eq 1 ]; then
                docker run --rm -v "$BENCHMARK_DIR/ontologies:/ontologies:ro" \
                    owl-reasoner-pellet "/ontologies/$ontology" consistency 2>&1 | tail -10 > "$result_file.tmp"
                status=$?
            else
                echo "Pellet not available" > "$result_file.tmp"
                status=1
            fi
            ;;
        tableauxx)
            if [ $HAVE_TABLEAUXX -eq 1 ]; then
                "$PROJECT_ROOT/target/release/owl2-reasoner" check "$ontology_path" 2>&1 | tail -10 > "$result_file.tmp"
                status=$?
            else
                echo "Tableauxx not available" > "$result_file.tmp"
                status=1
            fi
            ;;
    esac
    
    end_time=$(date +%s%N)
    duration_ms=$(( (end_time - start_time) / 1000000 ))
    
    # Extract timing from output if present
    local reported_duration=$(grep -o '"duration_ms": [0-9]*' "$result_file.tmp" | grep -o '[0-9]*' || echo "$duration_ms")
    
    # Create JSON result
    cat > "$result_file" << EOF
{
  "reasoner": "$reasoner",
  "ontology": "$ontology",
  "wall_time_ms": $duration_ms,
  "reported_time_ms": $reported_duration,
  "status": "$([ $status -eq 0 ] && echo "success" || echo "error")",
  "output": $(jq -Rs . < "$result_file.tmp")
}
EOF
    
    rm -f "$result_file.tmp"
    
    if [ $status -eq 0 ]; then
        log_success "$reasoner on $ontology: ${duration_ms}ms (reported: ${reported_duration}ms)"
    else
        log_error "$reasoner on $ontology failed"
    fi
}

# Generate summary report
generate_report() {
    log_info "Generating report..."
    
    local report_file="$RESULTS_DIR/benchmark_report_$(date +%Y%m%d_%H%M%S).md"
    
    cat > "$report_file" << EOF
# OWL2 Reasoner Benchmark Report

**Date:** $(date -Iseconds)

## Reasoners Tested

| Reasoner | Status |
|----------|--------|
| HermiT | $([ $HAVE_HERMIT -eq 1 ] && echo "✅ Available" || echo "❌ Not found") |
| Pellet/Openllet | $([ $HAVE_PELLET -eq 1 ] && echo "✅ Available" || echo "❌ Not found") |
| Tableauxx | $([ $HAVE_TABLEAUXX -eq 1 ] && echo "✅ Available" || echo "❌ Not found") |

## Results Summary

| Reasoner | Ontology | Wall Time | Reported Time | Status |
|----------|----------|-----------|---------------|--------|
EOF

    # Add results to table
    for result_file in "$RESULTS_DIR"/*.json; do
        if [ -f "$result_file" ]; then
            local r o wt rt st
            r=$(jq -r '.reasoner' "$result_file")
            o=$(jq -r '.ontology' "$result_file")
            wt=$(jq -r '.wall_time_ms' "$result_file")
            rt=$(jq -r '.reported_time_ms' "$result_file")
            st=$(jq -r '.status' "$result_file")
            printf "| %s | %s | %s ms | %s ms | %s |\n" "$r" "$o" "$wt" "$rt" "$([ "$st" = "success" ] && echo "✅" || echo "❌")" >> "$report_file"
        fi
    done
    
    echo "" >> "$report_file"
    echo "## Raw Results" >> "$report_file"
    echo "" >> "$report_file"
    echo "\`\`\`json" >> "$report_file"
    cat "$RESULTS_DIR"/*.json 2>/dev/null | jq -s '.' >> "$report_file"
    echo "\`\`\`" >> "$report_file"
    
    log_success "Report saved to: $report_file"
}

# Main execution
main() {
    echo "=========================================="
    echo "  OWL2 Reasoner Comprehensive Benchmark"
    echo "=========================================="
    echo
    
    check_reasoners
    
    echo
    log_info "Starting benchmarks..."
    echo
    
    for ontology in "${ONTOLOGIES[@]}"; do
        log_info "Testing ontology: $ontology"
        
        [ $HAVE_HERMIT -eq 1 ] && run_benchmark "hermit" "$ontology"
        [ $HAVE_PELLET -eq 1 ] && run_benchmark "pellet" "$ontology"
        [ $HAVE_TABLEAUXX -eq 1 ] && run_benchmark "tableauxx" "$ontology"
        
        echo
    done
    
    generate_report
    
    echo
    echo "=========================================="
    echo "  Benchmark Complete"
    echo "=========================================="
}

main "$@"
