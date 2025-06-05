Mutate an initial seed input, fuzz an external program with the input, and
gather coverage information from the program execution.
The coverage info is then fed back into the mutation phase, thus closing the
feedback loop and making the mutation fuzzer coverage guided.

Compared to the previous iteration, this one tries to improve the memory
footprint by not recording the coverage database as `BTreeMap<Coverage, Input>`,
thereby keeping a full coverage list of all inputs, but instead only recording
it as `BTreeMap<u64, Input>` (where the u64 is a hash of the coverage associated
with the input). This is possible since we care only about whether we have seen
a specific coverage already before, and not about what specific coverage it was.
(The key in that Map is not used for anything else besides contains-checks).
(Interesting inputs that lead to new coverage are saved to disk anyways).

Measuring the memory usage however shows that this refactoring does not really
improve memory usage and I don't know why (maybe the system under test is so
small that the full coverage map doesn't take up much space and it only becomes
relevant for larger programs... ?). Some example runs from tmpfs:

```
$ # new refactored version
$ rm /tmp/output
$ cargo build --release
$ seq 10 | while read i ; do /usr/bin/time -v timeout 10 target/release/mutationfuzzer8 2>&1 | tee -a /tmp/output ; done
$ cat /tmp/output | grep Max
        Maximum resident set size (kbytes): 25720
        Maximum resident set size (kbytes): 25584
        Maximum resident set size (kbytes): 25748
        Maximum resident set size (kbytes): 25816
        Maximum resident set size (kbytes): 25540
        Maximum resident set size (kbytes): 25724
        Maximum resident set size (kbytes): 25724
        Maximum resident set size (kbytes): 25748
        Maximum resident set size (kbytes): 25704
        Maximum resident set size (kbytes): 25720

$ # old version
$ rm /tmp/output
$ cargo build --release
$ seq 10 | while read i ; do /usr/bin/time -v timeout 10 target/release/mutationfuzzer7 2>&1 | tee -a /tmp/output ; done
$ cat /tmp/output| grep Max
        Maximum resident set size (kbytes): 25900
        Maximum resident set size (kbytes): 25808
        Maximum resident set size (kbytes): 25788
        Maximum resident set size (kbytes): 25736
        Maximum resident set size (kbytes): 25884
        Maximum resident set size (kbytes): 25924
        Maximum resident set size (kbytes): 25804
        Maximum resident set size (kbytes): 25784
        Maximum resident set size (kbytes): 25684
        Maximum resident set size (kbytes): 25588
```
