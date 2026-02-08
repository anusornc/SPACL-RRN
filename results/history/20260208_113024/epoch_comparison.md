# Benchmark Epoch Comparison

**Benchmark:** quick_benchmark  
**Date:** 2026-02-08 11:30:24  
**Total Epochs:** 3  
**Hardware:** Intel(R) Xeon(R) Silver 4214 CPU @ 2.20GHz

## Hardware Validation

| Property | Expected (Paper) | Actual (This Run) | Status |
|----------|------------------|-------------------|--------|
| CPU | AMD Ryzen 9 5900X | Intel(R) Xeon(R) Silver 4214 CPU @ 2.20GHz | ⚠️ Mismatch |
| Cores | 12/24 | 48 | - |
| Memory | 64GB | 62GB | - |

**⚠️ WARNING:** Hardware mismatch detected! Results may not match paper claims.

## Epoch Results

| Epoch | Status | Timestamp |
|-------|--------|-----------|
| 1 | COMPLETED | 2026-02-08 11:30:25 |
| 2 | COMPLETED | 2026-02-08 11:30:25 |
| 3 | COMPLETED | 2026-02-08 11:30:25 |

## Performance Consistency

### Variance Analysis

To analyze variance across epochs, check individual epoch metrics:
- Epoch 1: `epoch_1/metrics.txt`
- Epoch 2: `epoch_2/metrics.txt`
- Epoch 3: `epoch_3/metrics.txt`

### Coefficient of Variation (CV)
Calculate CV to check consistency:
\`\`\`bash
# Extract times and calculate statistics
for f in epoch_*/metrics.txt; do echo "$f:"; grep 'time:' $f | head -5; done
\`\`\`

## Recommendations



⚠️ **Hardware mismatch** detected. To ensure reproducibility:
1. Run benchmarks on claimed hardware (AMD Ryzen 9 5900X)
2. OR update paper hardware specifications
3. OR add erratum noting hardware change

## Files

- `system_info.txt` - Complete system configuration
- `epochs_summary.txt` - Summary of all epochs
- `epoch_N/benchmark_output.log` - Full output from epoch N
- `epoch_N/metrics.txt` - Extracted metrics from epoch N

## Next Steps

1. Check consistency across epochs
2. Calculate average and standard deviation
3. Compare with paper claims
4. Document any discrepancies
