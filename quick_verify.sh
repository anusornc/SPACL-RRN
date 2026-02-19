#!/bin/bash
# Quick verification of HermiT vs SPACL

set -e

ONTOLOGY_DIR="benchmarks/competitors/ontologies"
RESULTS_FILE="results/verification_$(date +%Y%m%d_%H%M%S).txt"

mkdir -p results
echo "HermiT vs SPACL Quick Verification" | tee "$RESULTS_FILE"
echo "=====================================" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Test with disjunctive_simple.owl (known working)
echo "Testing: disjunctive_simple.owl" | tee -a "$RESULTS_FILE"
echo "File size: $(du -h $ONTOLOGY_DIR/disjunctive_simple.owl | cut -f1)" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Run HermiT
echo -n "HermiT: " | tee -a "$RESULTS_FILE"
start=$(date +%s%N)
docker run --rm -v "$(pwd)/$ONTOLOGY_DIR:/ontologies:ro" owl-reasoner-hermit:latest "/ontologies/disjunctive_simple.owl" > /tmp/hermit_out.log 2>&1
hermit_time=$(( ($(date +%s%N) - start) / 1000000 ))
echo "${hermit_time}ms" | tee -a "$RESULTS_FILE"
grep "duration_ms" /tmp/hermit_out.log | tail -1 | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Run SPACL
echo -n "SPACL: " | tee -a "$RESULTS_FILE"
start=$(date +%s%N)
./target/release/owl2-reasoner check "$ONTOLOGY_DIR/disjunctive_simple.owl" > /tmp/spacl_out.log 2>&1
spacl_time=$(( ($(date +%s%N) - start) / 1000000 ))
echo "${spacl_time}ms" | tee -a "$RESULTS_FILE"
cat /tmp/spacl_out.log | grep -E "complete in|Result:" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Calculate speedup
if [ $spacl_time -gt 0 ]; then
    speedup=$(echo "scale=1; $hermit_time / $spacl_time" | bc 2>/dev/null || echo "N/A")
    echo "Speedup: ${speedup}x" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test with disjunctive_test.owl
echo "Testing: disjunctive_test.owl" | tee -a "$RESULTS_FILE"
echo "File size: $(du -h $ONTOLOGY_DIR/disjunctive_test.owl | cut -f1)" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Run HermiT
echo -n "HermiT: " | tee -a "$RESULTS_FILE"
start=$(date +%s%N)
docker run --rm -v "$(pwd)/$ONTOLOGY_DIR:/ontologies:ro" owl-reasoner-hermit:latest "/ontologies/disjunctive_test.owl" > /tmp/hermit_out2.log 2>&1
hermit_time=$(( ($(date +%s%N) - start) / 1000000 ))
echo "${hermit_time}ms" | tee -a "$RESULTS_FILE"
grep "duration_ms" /tmp/hermit_out2.log | tail -1 | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Run SPACL
echo -n "SPACL: " | tee -a "$RESULTS_FILE"
start=$(date +%s%N)
./target/release/owl2-reasoner check "$ONTOLOGY_DIR/disjunctive_test.owl" > /tmp/spacl_out2.log 2>&1
spacl_time=$(( ($(date +%s%N) - start) / 1000000 ))
echo "${spacl_time}ms" | tee -a "$RESULTS_FILE"
cat /tmp/spacl_out2.log | grep -E "complete in|Result:" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Calculate speedup
if [ $spacl_time -gt 0 ]; then
    speedup=$(echo "scale=1; $hermit_time / $spacl_time" | bc 2>/dev/null || echo "N/A")
    echo "Speedup: ${speedup}x" | tee -a "$RESULTS_FILE"
fi

echo "" | tee -a "$RESULTS_FILE"
echo "Verification complete. Results saved to: $RESULTS_FILE" | tee -a "$RESULTS_FILE"
