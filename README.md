# Benches

A little issue with rkyv right now:

bincode outperform rkyv serialization by 60% with the nightly compiler

This repository is to provide a working minimal use case

```
Periods/Serialization/rkyv                                                                            
                        time:   [42.262 ms 42.322 ms 42.388 ms]
                        change: [-0.3779% -0.1701% +0.0664%] (p = 0.11 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

Periods/Serialization/Bincode                                                                            
                        time:   [15.241 ms 15.282 ms 15.328 ms]
                        change: [-3.2595% -2.6876% -2.1209%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```
