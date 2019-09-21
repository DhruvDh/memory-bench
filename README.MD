# Assignment: Benchmarking the memory subsystem

## Part !: Analysis

- **How much DRAM memory? Memory speed? Memory Technology?**

16 GB LPDDR3 memory @ 1866Mhz

- **How much bandwidth can your processor draw from the bus? You will need to discover width of the memory bus and clock speed of the memory bus. Full duplex/half duplex?** 

29.856‬ GB/s. (8 bytes per cycle per channel)

- **What is the highest level of data cache? How big is it? How is it shared across core Same question for all levels of data cache**

Highest level is L3, 8 MB, shared across all cores.

Next is L2, 256 KB, shared across two threads in a core, not across cores.

Next is L1 instruction and data cache, 4 KB each, shared acorss two threads in a core, not across cores.