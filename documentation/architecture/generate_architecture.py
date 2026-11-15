from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch

OUT = Path(__file__).parent

fig, ax = plt.subplots(figsize=(16, 11))
ax.set_xlim(0, 16)
ax.set_ylim(0, 12)
ax.axis("off")

def box(x, y, w, h, title, detail, size=10):
    patch = FancyBboxPatch(
        (x, y), w, h,
        boxstyle="round,pad=0.08",
        linewidth=1.5,
        edgecolor="black",
        facecolor="white"
    )
    ax.add_patch(patch)
    ax.text(x + w/2, y + h*0.68, title,
            ha="center", va="center",
            fontsize=size, fontweight="bold")
    ax.text(x + w/2, y + h*0.32, detail,
            ha="center", va="center", fontsize=8)

def arrow(x1, y1, x2, y2):
    ax.add_patch(FancyArrowPatch(
        (x1, y1), (x2, y2),
        arrowstyle="-|>", mutation_scale=15,
        linewidth=1.3, color="black"
    ))

ax.text(8, 11.65, "SRIKESH OS — SYSTEM ARCHITECTURE",
        ha="center", fontsize=18, fontweight="bold")
ax.text(8, 11.25, "Rust x86-64 | no_std | BIOS and UEFI boot support",
        ha="center", fontsize=11)

# Boot path
box(0.6, 9.65, 3.0, 1.0, "HOST BUILD SYSTEM",
    "Cargo • build.rs • QEMU")
box(6.5, 9.65, 3.0, 1.0, "BOOT IMAGES",
    "BIOS image • UEFI image")
box(12.4, 9.65, 3.0, 1.0, "BOOTLOADER",
    "Loads kernel and BootInfo")

arrow(3.6, 10.15, 6.5, 10.15)
arrow(9.5, 10.15, 12.4, 10.15)

# Kernel entry
box(5.5, 8.0, 5.0, 1.0, "RUST KERNEL",
    "no_std • kernel_main • initialization")
arrow(13.9, 9.65, 10.5, 8.5)

# Hardware and interrupt services
box(0.4, 6.15, 3.5, 1.15, "CPU & MEMORY",
    "CPUID • memory map • allocator")
box(4.4, 6.15, 3.5, 1.15, "INTERRUPT SUBSYSTEM",
    "IDT • PIC • PIT • timer handler")
box(8.1, 6.15, 3.5, 1.15, "TASK MANAGEMENT",
    "TCB • task states • task entry")
box(12.0, 6.15, 3.5, 1.15, "DIAGNOSTICS",
    "Serial • logger • runtime statistics")

for x in [2.15, 6.15, 9.85, 13.75]:
    arrow(8, 8.0, x, 7.3)

# Scheduler
box(2.0, 4.25, 4.5, 1.1, "READY QUEUE",
    "Circular FIFO • capacity 8")
box(9.0, 4.25, 4.5, 1.1, "ROUND ROBIN SCHEDULER",
    "Preemptive • 5-tick quantum")

arrow(9.85, 6.15, 11.0, 5.35)
arrow(6.5, 4.8, 9.0, 4.8)
arrow(6.15, 6.15, 4.25, 5.35)

# Context switching
box(4.5, 2.35, 7.0, 1.05, "CONTEXT SWITCHING",
    "Save/restore registers • Rust scheduling • assembly • iretq")
arrow(11.0, 4.25, 9.5, 3.4)

# Tasks
box(1.2, 0.55, 5.0, 1.0, "TASK 1",
    "Preemptive demonstration task")
box(9.8, 0.55, 5.0, 1.0, "TASK 2",
    "Preemptive demonstration task")

arrow(6.0, 2.35, 3.7, 1.55)
arrow(10.0, 2.35, 12.3, 1.55)

# Supporting components
ax.text(8, 1.85,
        "SUPPORTING SERVICES: spin::Mutex synchronization • scheduler self-tests",
        ha="center", fontsize=9)

ax.text(8, 0.12,
        "Architecture reflects implemented demonstration functionality.",
        ha="center", fontsize=8, style="italic")

fig.tight_layout()
fig.savefig(OUT / "micro_os_architecture.png", dpi=220, bbox_inches="tight")
fig.savefig(OUT / "micro_os_architecture.svg", bbox_inches="tight")
plt.close(fig)

print("Architecture diagram generated successfully.")
print(OUT / "micro_os_architecture.png")
print(OUT / "micro_os_architecture.svg")
