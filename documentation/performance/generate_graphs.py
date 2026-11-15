import csv
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

BASE = Path(__file__).parent
CSV_FILE = BASE / "scheduler_results.csv"

with CSV_FILE.open(newline="") as file:
    rows = list(csv.DictReader(file))

if not rows:
    raise SystemExit("ERROR: No performance records found.")

modes = [row["Boot_Mode"] for row in rows]
context_switches = [int(row["Context_Switches"]) for row in rows]
task1 = [int(row["Task1_Dispatches"]) for row in rows]
task2 = [int(row["Task2_Dispatches"]) for row in rows]

# Graph 1: Scheduler activity comparison
x = np.arange(len(modes))
width = 0.24

fig, ax = plt.subplots(figsize=(9, 5))
ax.bar(x - width, context_switches, width, label="Context switches")
ax.bar(x, task1, width, label="Task 1 dispatches")
ax.bar(x + width, task2, width, label="Task 2 dispatches")

ax.set_title("Scheduler Activity: BIOS vs UEFI")
ax.set_xlabel("Boot mode")
ax.set_ylabel("Observed count")
ax.set_xticks(x)
ax.set_xticklabels(modes)
ax.legend()
ax.grid(axis="y", linestyle="--", alpha=0.4)

fig.tight_layout()
fig.savefig(BASE / "scheduler_activity_comparison.png", dpi=200)
plt.close(fig)

# Graph 2: Configured quantum and calculated time slice
quantum = int(rows[0]["Quantum_Ticks"])
pit_frequency = int(rows[0]["PIT_Frequency_Hz"])
time_slice = float(rows[0]["Time_Slice_ms"])

fig, ax = plt.subplots(figsize=(8, 5))
bars = ax.bar(
    ["Quantum (ticks)", "Time slice (ms)"],
    [quantum, time_slice]
)

ax.set_title("Round Robin Scheduler Time-Slice Configuration")
ax.set_ylabel("Value")
ax.grid(axis="y", linestyle="--", alpha=0.4)

for bar, value in zip(bars, [quantum, time_slice]):
    ax.text(
        bar.get_x() + bar.get_width() / 2,
        bar.get_height(),
        str(value),
        ha="center",
        va="bottom"
    )

fig.tight_layout()
fig.savefig(BASE / "scheduler_quantum_analysis.png", dpi=200)
plt.close(fig)

print("Generated:")
print(BASE / "scheduler_activity_comparison.png")
print(BASE / "scheduler_quantum_analysis.png")
print(f"PIT frequency: {pit_frequency} Hz")
print(f"Quantum: {quantum} ticks")
print(f"Calculated time slice: {time_slice} ms")
