
# Running Risky in Clion

To run Risky in clion, just use the provided run configuration. You can use "run" and "run release". However, you cannot use these runconfigurations to debug the kernel. To do this, read the instructions below

# Debugging Risky in Clion

To debug Risky in clion, you will have to run the "run debug" configuration. This replaces the default cargo runner, with a different qemu command which waits on `localhost:1234`. Now you can use the "debug" run configuration to debug the kernel. Breakpoints will work as normal. 
