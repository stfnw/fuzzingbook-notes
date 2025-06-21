https://www.fuzzingbook.org/html/GrammarFuzzer.html#Exercise-1:-Caching-Method-Results
https://www.fuzzingbook.org/html/GrammarFuzzer.html#Exercise-2:-Grammar-Pre-Compilation
https://www.fuzzingbook.org/html/GrammarFuzzer.html#Exercise-3:-Maintaining-Trees-to-be-Expanded

GrammarFuzzer / creating a random derivation tree / string from a grammar.
Adds some optimizations.

# Some profiling

```
# then copy to tmpfs
RUSTFLAGS=-g cargo build --release
perf record --call-graph dwarf ./target/release/grammarfuzzer5
```

## Before

```
$ perf report --stdio | grep grammarfuz | grep % | sort -k2 -nr | head           
    75.18%    55.76%  grammarfuzzer5  grammarfuzzer5     [.] _ZN14grammarfuzzer513grammarfuzzer4Tree23any_possible_expansions17haa623e9b969e9c7cE.llvm.17599862759720979252
    13.67%    12.23%  grammarfuzzer5  grammarfuzzer5     [.] grammarfuzzer5::grammarfuzzer::Tree::possible_expansions
     2.52%     2.52%  grammarfuzzer5  libc.so.6          [.] malloc
     2.52%     2.52%  grammarfuzzer5  [kernel.kallsyms]  [k] finish_task_switch.isra.0
     2.52%     2.16%  grammarfuzzer5  libc.so.6          [.] _int_malloc
    72.30%     1.80%  grammarfuzzer5  grammarfuzzer5     [.] grammarfuzzer5::grammarfuzzer::expand_tree_once
     1.80%     1.80%  grammarfuzzer5  libc.so.6          [.] __memmove_avx_unaligned
     1.80%     1.80%  grammarfuzzer5  libc.so.6          [.] cfree@GLIBC_2.2.5
     5.04%     1.44%  grammarfuzzer5  libc.so.6          [.] _int_realloc
     1.44%     1.44%  grammarfuzzer5  [vboxguest]        [k] vbg_req_perform
```

```
$ for _ in $(seq 10) ; do /usr/bin/time -v ./target/release/grammarfuzzer5 2>&1 | grep -e 'wall clock' -e 'Maximum resident set size' ; done
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.74
        Maximum resident set size (kbytes): 4316
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.69
        Maximum resident set size (kbytes): 4260
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.70
        Maximum resident set size (kbytes): 4308
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.71
        Maximum resident set size (kbytes): 4316
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.75
        Maximum resident set size (kbytes): 4316
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.74
        Maximum resident set size (kbytes): 3912
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.72
        Maximum resident set size (kbytes): 4312
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.71
        Maximum resident set size (kbytes): 4304
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.74
        Maximum resident set size (kbytes): 4300
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.75
        Maximum resident set size (kbytes): 4320
```

## After

```
$ perf report --stdio | grep grammarfuz | grep % | sort -k2 -nr | head
    46.15%    46.15%  grammarfuzzer5  [kernel.kallsyms]  [k] finish_task_switch.isra.0
    15.38%    15.38%  grammarfuzzer5  [kernel.kallsyms]  [k] _raw_spin_unlock_irqrestore
     7.69%     7.69%  grammarfuzzer5  libc.so.6          [.] unlink_chunk.isra.0
     7.69%     7.69%  grammarfuzzer5  [kernel.kallsyms]  [k] __tlb_remove_folio_pages
     7.69%     7.69%  grammarfuzzer5  [kernel.kallsyms]  [k] free_ldt_pgtables
    15.38%     7.69%  grammarfuzzer5  libc.so.6          [.] malloc_consolidate
    15.38%     7.69%  grammarfuzzer5  [kernel.kallsyms]  [k] queue_work_on
    84.62%     0.00%  grammarfuzzer5  libc.so.6          [.] __libc_start_main_impl (inlined)
    84.62%     0.00%  grammarfuzzer5  libc.so.6          [.] __libc_start_call_main
    84.62%     0.00%  grammarfuzzer5  [kernel.kallsyms]  [k] entry_SYSCALL_64_after_hwframe
```

```
$ for _ in $(seq 10) ; do /usr/bin/time -v ./target/release/grammarfuzzer5 2>&1 | grep -e 'wall clock' -e 'Maximum resident set size' ; done
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.02
        Maximum resident set size (kbytes): 3788
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.03
        Maximum resident set size (kbytes): 3800
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.04
        Maximum resident set size (kbytes): 3760
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.05
        Maximum resident set size (kbytes): 3788
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.04
        Maximum resident set size (kbytes): 3792
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.03
        Maximum resident set size (kbytes): 3808
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.02
        Maximum resident set size (kbytes): 3796
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.03
        Maximum resident set size (kbytes): 3764
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.03
        Maximum resident set size (kbytes): 3764
        Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.03
        Maximum resident set size (kbytes): 3708
```
