#!/usr/bin/env python3
"""Generate paper figures from benchmark data"""
import matplotlib.pyplot as plt
import numpy as np

# Benchmark data
classes = [100, 500, 1000, 5000, 10000]
sequential = [13.3, 75.9, 159.7, 805.9, 1865.3]
spacl = [20.9, 84.3, 158.4, 277.0, 382.3]
speedup = [s/p for s, p in zip(sequential, spacl)]

# Figure 1: Scalability comparison
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

# Left: Time comparison
ax1.plot(classes, sequential, 'o-', label='Sequential', linewidth=2, markersize=8)
ax1.plot(classes, spacl, 's-', label='SPACL', linewidth=2, markersize=8)
ax1.set_xlabel('Number of Classes', fontsize=12)
ax1.set_ylabel('Time (µs)', fontsize=12)
ax1.set_title('Scalability: Sequential vs SPACL', fontsize=14, fontweight='bold')
ax1.legend(fontsize=11)
ax1.grid(True, alpha=0.3)
ax1.set_xscale('log')
ax1.set_yscale('log')

# Right: Speedup
ax2.plot(classes, speedup, 'D-', color='green', linewidth=2, markersize=8)
ax2.axhline(y=1, color='red', linestyle='--', label='Parity (1x)')
ax2.set_xlabel('Number of Classes', fontsize=12)
ax2.set_ylabel('Speedup (Sequential/SPACL)', fontsize=12)
ax2.set_title('SPACL Speedup vs Sequential', fontsize=14, fontweight='bold')
ax2.legend(fontsize=11)
ax2.grid(True, alpha=0.3)
ax2.set_xscale('log')

plt.tight_layout()
plt.savefig('scalability.png', dpi=300, bbox_inches='tight')
plt.savefig('scalability.pdf', bbox_inches='tight')
print("Generated: scalability.png, scalability.pdf")

# Figure 2: Throughput comparison
fig, ax = plt.subplots(figsize=(10, 6))

seq_throughput = [c / (t/1_000_000) / 1_000_000 for c, t in zip(classes, sequential)]
spacl_throughput = [c / (t/1_000_000) / 1_000_000 for c, t in zip(classes, spacl)]

x = np.arange(len(classes))
width = 0.35

bars1 = ax.bar(x - width/2, seq_throughput, width, label='Sequential', color='steelblue')
bars2 = ax.bar(x + width/2, spacl_throughput, width, label='SPACL', color='darkorange')

ax.set_xlabel('Number of Classes', fontsize=12)
ax.set_ylabel('Throughput (M operations/sec)', fontsize=12)
ax.set_title('Throughput Comparison: Sequential vs SPACL', fontsize=14, fontweight='bold')
ax.set_xticks(x)
ax.set_xticklabels(classes)
ax.legend(fontsize=11)
ax.grid(True, alpha=0.3, axis='y')

# Add value labels on bars
for bar in bars1:
    height = bar.get_height()
    ax.annotate(f'{height:.1f}',
                xy=(bar.get_x() + bar.get_width() / 2, height),
                xytext=(0, 3),
                textcoords="offset points",
                ha='center', va='bottom', fontsize=9)

for bar in bars2:
    height = bar.get_height()
    ax.annotate(f'{height:.1f}',
                xy=(bar.get_x() + bar.get_width() / 2, height),
                xytext=(0, 3),
                textcoords="offset points",
                ha='center', va='bottom', fontsize=9)

plt.tight_layout()
plt.savefig('throughput.png', dpi=300, bbox_inches='tight')
plt.savefig('throughput.pdf', bbox_inches='tight')
print("Generated: throughput.png, throughput.pdf")

# Figure 3: Speedup with crossover annotation
fig, ax = plt.subplots(figsize=(10, 6))

ax.plot(classes, speedup, 'o-', linewidth=3, markersize=10, color='green')
ax.axhline(y=1, color='red', linestyle='--', linewidth=2, label='Parity (1x)')
ax.axvline(x=1000, color='purple', linestyle=':', linewidth=2, label='Crossover (~1000)')
ax.fill_between(classes, speedup, 1, where=[s > 1 for s in speedup], alpha=0.3, color='green', label='SPACL Advantage')

ax.set_xlabel('Number of Classes', fontsize=13)
ax.set_ylabel('Speedup (Sequential/SPACL)', fontsize=13)
ax.set_title('SPACL Speedup with Crossover Point', fontsize=15, fontweight='bold')
ax.legend(fontsize=11, loc='upper left')
ax.grid(True, alpha=0.3)
ax.set_xscale('log')

# Annotate key points
ax.annotate(f'{speedup[-1]:.1f}x speedup', xy=(classes[-1], speedup[-1]), 
            xytext=(classes[-1]/2, speedup[-1] + 0.5),
            arrowprops=dict(arrowstyle='->', color='black'),
            fontsize=11, fontweight='bold')

plt.tight_layout()
plt.savefig('speedup.png', dpi=300, bbox_inches='tight')
plt.savefig('speedup.pdf', bbox_inches='tight')
print("Generated: speedup.png, speedup.pdf")

print("\nAll figures generated successfully!")
