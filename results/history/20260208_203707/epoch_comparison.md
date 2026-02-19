# Benchmark Epoch Comparison

**Benchmark:** hierarchical_benchmark  
**Date:** 2026-02-08 20:37:07  
**Total Epochs:** 1  
**Hardware:** Intel(R) Xeon(R) Silver 4214 CPU @ 2.20GHz

## Hardware Validation

| Property | Expected (Paper) | Actual (This Run) | Status |
|----------|------------------|-------------------|--------|
| CPU | Intel Xeon Silver 4214 | Intel(R) Xeon(R) Silver 4214 CPU @ 2.20GHz | ✅ Match |
| Cores | 12/24 | 48 | - |
| Memory | 64GB | 62GB | - |



## Epoch Results

| Epoch | Status | Timestamp |
|-------|--------|-----------|
| 1 | COMPLETED | 2026-02-08 20:37:07 |

## Performance Consistency

Only 1 epoch run. For variance analysis, run with multiple epochs:
`./scripts/run_benchmark_v2.sh hierarchical_benchmark 5`

## Recommendations

⚠️ **Low epoch count** (1). For statistical significance, recommend at least 3-5 epochs.



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
