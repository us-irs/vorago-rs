target remote localhost:2331

# Reset is problematic on RevA, okay for RevB
monitor reset

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

load

continue
