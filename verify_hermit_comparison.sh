#!/bin/bash
# Verify HermiT vs SPACL comparison

set -e

RESULTS_DIR="results/hermit_verification_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo "========================================"
echo "HermiT vs SPACL Verification Benchmark"
echo "========================================"
echo "Results will be saved to: $RESULTS_DIR"
echo ""

ONTOLOGY_DIR="benchmarks/competitors/ontologies"

# Test ontologies
ONTOLOGIES=(
    "univ-bench.owl"
    "disjunctive_simple.owl"
    "disjunctive_test.owl"
    "hierarchy_100.owl"
    "hierarchy_1000.owl"
    "hierarchy_10000.owl"
)

# Check if Docker images exist
echo "Checking Docker images..."
if ! docker images | grep -q "owl-reasoner-hermit"; then
    echo "ERROR: HermiT Docker image not found. Building..."
    cd benchmarks/competitors/docker
    docker build -t owl-reasoner-hermit:latest -f Dockerfile.hermit .
    cd ../../..
fi

echo "Found HermiT Docker image ✓"
echo ""

# Function to run HermiT benchmark
run_hermit() {
    local ontology=$1
    local name=$(basename "$ontology")
    
    echo "Running HermiT on $name..."
    
    # Run with timeout
    start_time=$(date +%s%N)
    
    docker run --rm \
        -v "$(pwd)/$ONTOLOGY_DIR:/ontologies:ro" \
        owl-reasoner-hermit:latest \
        "/ontologies/$name" 2>&1 | tee "$RESULTS_DIR/hermit_${name%.owl}.log"
    
    end_time=$(date +%s%N)
    duration_ms=$(( (end_time - start_time) / 1000000 ))
    
    echo "HermiT completed in ${duration_ms}ms"
    echo ""
}

# Function to run SPACL benchmark  
run_spacl() {
    local ontology=$1
    local name=$(basename "$ontology")
    
    echo "Running SPACL on $name..."
    
    start_time=$(date +%s%N)
    
    ./target/release/owl2-reasoner check "$ONTOLOGY_DIR/$name" 2>&1 | tee "$RESULTS_DIR/spacl_${name%.owl}.log"
    
    end_time=$(date +%s%N)
    duration_ms=$(( (end_time - start_time) / 1000000 ))
    
    echo "SPACL completed in ${duration_ms}ms"
    echo ""
}

# Run benchmarks
for ontology in "${ONTOLOGIES[@]}"; do
    if [ -f "$ONTOLOGY_DIR/$ontology" ]; then
        echo "========================================"
        echo "Testing: $ontology"
        echo "========================================"
        
        # Get file size
        size=$(du -h "$ONTOLOGY_DIR/$ontology" | cut -f1)
        echo "File size: $size"
        
        # Run HermiT
        run_hermit "$ontology"
        
        # Run SPACL
        run_spacl "$ontology"
        
        echo ""
    else
        echo "WARNING: $ontology not found, skipping"
    fi
done

echo "========================================"
echo "Verification Complete"
echo "========================================"
echo "Results saved to: $RESULTS_DIR"
echo ""
echo "Summary:"
ls -la "$RESULTS_DIR/"
