target remote localhost:2331

# For some reason, this is problematic even if the JLinkScript disabled the remote
# write protection. Therefore, don't do it for now
# This is only problematic on board RevA
# monitor reset

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

load

continue
