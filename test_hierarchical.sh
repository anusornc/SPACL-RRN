#!/bin/bash
# Quick test of hierarchical classification

cd "$(dirname "$0")"

echo "=========================================="
echo "  Testing Hierarchical Classification"
echo "=========================================="
echo ""

# Run the demo
echo "Running hierarchical_demo on LUBM..."
cargo run --example hierarchical_demo --release 2>&1 | tee /tmp/hierarchical_test.log

echo ""
echo "=========================================="
echo "  Test Complete"
echo "=========================================="
echo ""
echo "Results saved to: /tmp/hierarchical_test.log"
