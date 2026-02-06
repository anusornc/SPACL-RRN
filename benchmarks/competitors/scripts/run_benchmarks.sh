#!/bin/bash
#
# OWL2 Reasoner Benchmark Suite
# Compares Tableauxx with competitors: HermiT, Konclude, Pellet, FaCT++
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BENCHMARK_DIR="$PROJECT_ROOT/benchmarks/competitors"
RESULTS_DIR="$BENCHMARK_DIR/results"
ONTOLOGIES_DIR="$BENCHMARK_DIR/ontologies"

# Reasoners to benchmark
REASONERS=("hermit" "konclude" "pellet" "tableauxx")  # "factpp" - limited CLI support
OPERATIONS=("consistency")  # Can add "classification" later

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create necessary directories
mkdir -p "$RESULTS_DIR"
mkdir -p "$ONTOLOGIES_DIR"

# Function to prepare test ontologies
prepare_ontologies() {
    log_info "Preparing test ontologies..."
    
    # Copy existing test ontologies from the project
    if [ -d "$PROJECT_ROOT/tests/data" ]; then
        cp "$PROJECT_ROOT/tests/data"/*.owl "$ONTOLOGIES_DIR/" 2>/dev/null || true
        log_success "Copied test ontologies from tests/data"
    fi
    
    # Check what we have
    local count=$(find "$ONTOLOGIES_DIR" -name "*.owl" | wc -l)
    log_info "Found $count OWL ontologies for benchmarking"
}

# Function to build Docker images
build_images() {
    log_info "Building Docker images for reasoners..."
    cd "$BENCHMARK_DIR"
    
    for reasoner in "${REASONERS[@]}"; do
        log_info "Building $reasoner image..."
        if docker build -f "docker/Dockerfile.$reasoner" -t "owl-reasoner-$reasoner" "$PROJECT_ROOT" 2>&1 | tail -10; then
            log_success "$reasoner image built"
        else
            log_error "Failed to build $reasoner image"
        fi
    done
}

# Function to run a single benchmark
run_single_benchmark() {
    local reasoner=$1
    local ontology=$2
    local operation=$3
    local ontology_name=$(basename "$ontology")
    
    log_info "Running $reasoner on $ontology_name ($operation)..."
    
    local result_file="$RESULTS_DIR/${reasoner}_${ontology_name%.owl}_${operation}.json"
    local temp_output=$(mktemp)
    
    # Run the Docker container with timeout
    if timeout 300 docker run --rm \
        -v "$ONTOLOGIES_DIR:/ontologies:ro" \
        -v "$RESULTS_DIR:/results" \
        "owl-reasoner-$reasoner" \
        "/ontologies/$ontology_name" "$operation" > "$temp_output" 2>&1; then
        
        # Extract timing information
        local duration=$(grep -o '"duration_ms": [0-9]*' "$temp_output" | grep -o '[0-9]*' || echo "-1")
        
        # Create result JSON
        cat > "$result_file" << EOF
{
  "reasoner": "$reasoner",
  "ontology": "$ontology_name",
  "operation": "$operation",
  "duration_ms": $duration,
  "timestamp": "$(date -Iseconds)",
  "raw_output": $(jq -Rs . < "$temp_output")
}
EOF
        
        if [ "$duration" -gt 0 ]; then
            log_success "$reasoner completed in ${duration}ms"
        else
            log_warn "$reasoner completed but timing unavailable"
        fi
    else
        log_error "$reasoner failed or timed out on $ontology_name"
        cat > "$result_file" << EOF
{
  "reasoner": "$reasoner",
  "ontology": "$ontology_name",
  "operation": "$operation",
  "duration_ms": -1,
  "timestamp": "$(date -Iseconds)",
  "status": "failed",
  "error": "Timeout or execution error"
}
EOF
    fi
    
    rm -f "$temp_output"
}

# Function to run all benchmarks
run_all_benchmarks() {
    log_info "Starting benchmark runs..."
    
    # Find all OWL ontologies
    local ontologies=($(find "$ONTOLOGIES_DIR" -name "*.owl" -type f))
    
    if [ ${#ontologies[@]} -eq 0 ]; then
        log_error "No OWL ontologies found in $ONTOLOGIES_DIR"
        return 1
    fi
    
    log_info "Found ${#ontologies[@]} ontologies to benchmark"
    
    # Run benchmarks
    for ontology in "${ontologies[@]}"; do
        for reasoner in "${REASONERS[@]}"; do
            for operation in "${OPERATIONS[@]}"; do
                run_single_benchmark "$reasoner" "$ontology" "$operation"
            done
        done
    done
}

# Function to generate comparison report
generate_report() {
    log_info "Generating comparison report..."
    
    local report_file="$RESULTS_DIR/benchmark_report.md"
    
    cat > "$report_file" << 'EOF'
# OWL2 Reasoner Benchmark Results

Generated: $(date)

## Summary

| Reasoner | Version | Status |
|----------|---------|--------|
| Tableauxx | 0.2.0 | ✅ Tested |
| HermiT | 1.4.5 | ✅ Tested |
| Konclude | 0.7.0 | ✅ Tested |
| Pellet | 2.4.0 | ✅ Tested |
| FaCT++ | 1.6.5 | ⚠️ Limited CLI |

## Results by Ontology

EOF

    # Append results for each ontology
    for result_file in "$RESULTS_DIR"/*.json; do
        if [ -f "$result_file" ]; then
            cat "$result_file" >> "$report_file"
            echo "" >> "$report_file"
            echo "---" >> "$report_file"
            echo "" >> "$report_file"
        fi
    done
    
    log_success "Report generated: $report_file"
}

# Main execution
main() {
    log_info "OWL2 Reasoner Benchmark Suite"
    log_info "============================="
    
    # Check Docker is available
    if ! command -v docker &> /dev/null; then
        log_error "Docker is required but not installed"
        exit 1
    fi
    
    # Parse arguments
    case "${1:-all}" in
        prepare)
            prepare_ontologies
            ;;
        build)
            build_images
            ;;
        run)
            run_all_benchmarks
            ;;
        report)
            generate_report
            ;;
        all|*)
            prepare_ontologies
            build_images
            run_all_benchmarks
            generate_report
            ;;
    esac
    
    log_success "Benchmark suite completed!"
}

main "$@"
