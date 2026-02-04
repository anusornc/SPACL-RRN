#!/bin/bash
# Overnight benchmark script for large ontologies
# Run with: nohup ./scripts/overnight_benchmark.sh > results/overnight_$(date +%Y%m%d).log 2>&1 &

set -e

echo "=========================================="
echo "Tableauxx Overnight Benchmark"
echo "Started: $(date)"
echo "=========================================="

# Create results directory
mkdir -p results/overnight

cd "$(dirname "$0")/.."

# Build release mode
echo "Building release binary..."
cargo build --release --example benchmark_large 2>&1 | tail -5

echo ""
echo "=========================================="
echo "Test 1: PATO (13K classes)"
echo "=========================================="
timeout 600 ./target/release/examples/benchmark_large benchmarks/ontologies/other/pato.owl 2>&1 | tee results/overnight/pato_$(date +%Y%m%d_%H%M).log || echo "Timeout or error"

echo ""
echo "=========================================="
echo "Test 2: DOID (15K classes)"
echo "=========================================="
timeout 600 ./target/release/examples/benchmark_large benchmarks/ontologies/other/doid.owl 2>&1 | tee results/overnight/doid_$(date +%Y%m%d_%H%M).log || echo "Timeout or error"

echo ""
echo "=========================================="
echo "Test 3: UBERON (15K classes)"
echo "=========================================="
timeout 900 ./target/release/examples/benchmark_large benchmarks/ontologies/other/uberon.owl 2>&1 | tee results/overnight/uberon_$(date +%Y%m%d_%H%M).log || echo "Timeout or error"

echo ""
echo "=========================================="
echo "Test 4: GO Basic (45K classes)"
echo "=========================================="
timeout 1800 ./target/release/examples/benchmark_large benchmarks/ontologies/other/go-basic.owl 2>&1 | tee results/overnight/go-basic_$(date +%Y%m%d_%H%M).log || echo "Timeout or error"

echo ""
echo "=========================================="
echo "Test 5: ChEBI (200K classes) - Long running"
echo "=========================================="
timeout 3600 ./target/release/examples/benchmark_large benchmarks/ontologies/other/chebi.owl 2>&1 | tee results/overnight/chebi_$(date +%Y%m%d_%H%M).log || echo "Timeout or error"

echo ""
echo "=========================================="
echo "Benchmark Complete: $(date)"
echo "=========================================="
echo "Results saved in results/overnight/"
