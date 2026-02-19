#!/bin/bash
# Verify hierarchy benchmarks

ONTOLOGY_DIR="benchmarks/competitors/ontologies"
RESULTS_FILE="results/hierarchy_verification_$(date +%Y%m%d_%H%M%S).txt"

mkdir -p results
echo "Hierarchy Benchmark Verification" | tee "$RESULTS_FILE"
echo "=================================" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

for size in 100 1000 10000; do
    ONTOLOGY="hierarchy_${size}.owl"
    
    if [ ! -f "$ONTOLOGY_DIR/$ONTOLOGY" ]; then
        echo "Skipping $ONTOLOGY (not found)"
        continue
    fi
    
    echo "Testing: $ONTOLOGY ($(du -h $ONTOLOGY_DIR/$ONTOLOGY | cut -f1))" | tee -a "$RESULTS_FILE"
    
    # Run HermiT
    echo -n "  HermiT: " | tee -a "$RESULTS_FILE"
    start=$(date +%s%N)
    if docker run --rm -v "$(pwd)/$ONTOLOGY_DIR:/ontologies:ro" owl-reasoner-hermit:latest "/ontologies/$ONTOLOGY" > /tmp/hermit_hier.log 2>&1; then
        hermit_time=$(( ($(date +%s%N) - start) / 1000000 ))
        hermit_reasoning=$(grep "duration_ms" /tmp/hermit_hier.log | tail -1 | grep -o '[0-9]*')
        echo "${hermit_time}ms (reasoning: ${hermit_reasoning}ms)" | tee -a "$RESULTS_FILE"
    else
        echo "FAILED" | tee -a "$RESULTS_FILE"
        hermit_time=0
    fi
    
    # Run SPACL
    echo -n "  SPACL:  " | tee -a "$RESULTS_FILE"
    start=$(date +%s%N)
    if ./target/release/owl2-reasoner check "$ONTOLOGY_DIR/$ONTOLOGY" > /tmp/spacl_hier.log 2>&1; then
        spacl_time=$(( ($(date +%s%N) - start) / 1000000 ))
        echo "${spacl_time}ms" | tee -a "$RESULTS_FILE"
    else
        echo "FAILED" | tee -a "$RESULTS_FILE"
        spacl_time=0
    fi
    
    # Calculate speedup
    if [ $spacl_time -gt 0 ] && [ $hermit_time -gt 0 ]; then
        speedup=$(echo "scale=2; $hermit_time / $spacl_time" | bc 2>/dev/null || echo "N/A")
        echo "  Speedup: ${speedup}x" | tee -a "$RESULTS_FILE"
    fi
    
    echo "" | tee -a "$RESULTS_FILE"
done

echo "Verification complete. Results saved to: $RESULTS_FILE"
