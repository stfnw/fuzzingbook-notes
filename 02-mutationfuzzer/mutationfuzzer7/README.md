Mutate an initial seed input, fuzz an external program with the input, and
gather coverage information from the program execution.
The coverage info is then fed back into the mutation phase, thus closing the
feedback loop and making the mutation fuzzer coverage guided.

Compared to the previous iteration, this one fixes a glaring performance
optimization possibility by doing compilation of the target binary only once at
the beginning and not anew for each individual fuzz case; thus achieving much
more fuzz cases per time unit spent.

Example run (in VM on tmpfs):

```
$ cargo run --release
   Compiling mutationfuzzer7 v0.1.0 (/tmp/mutationfuzzer7)
    Finished `release` profile [optimized] target(s) in 0.93s
     Running `target/release/mutationfuzzer7`
[+] Running with random seed 15755532146551530227
[+] Running with random seed 15755532146551666283
[+] Running with random seed 15755532146551766631
[+] Running with random seed 15755532146552410513
[+] Running with random seed 15755532146552485677
[+] Running with random seed 15755532146552689403
        1.09 uptime |    1871 fuzz cases |       43 coverage |     2 inputs
        2.09 uptime |    3750 fuzz cases |       47 coverage |     4 inputs
        3.09 uptime |    5626 fuzz cases |       47 coverage |     4 inputs
        4.09 uptime |    7508 fuzz cases |       47 coverage |     4 inputs
        5.09 uptime |    9374 fuzz cases |       47 coverage |     4 inputs
        6.09 uptime |   11276 fuzz cases |       47 coverage |     4 inputs
        7.09 uptime |   13161 fuzz cases |       47 coverage |     4 inputs
        8.09 uptime |   15155 fuzz cases |       47 coverage |     4 inputs
        9.09 uptime |   17067 fuzz cases |       47 coverage |     4 inputs
```

Example profiling (running in VM on disk):

```
$ RUSTFLAGS=-g cargo build --release
    Finished `release` profile [optimized] target(s) in 0.02s

$ perf record --call-graph dwarf -- timeout 6 target/release/mutationfuzzer7
[+] Running with random seed 15755525119011700523
[+] Running with random seed 15755525119012016681
[+] Running with random seed 15755525119012240423
[+] Running with random seed 15755525119012747975
[+] Running with random seed 15755525119013007509
[+] Running with random seed 15755525119012525907
        1.07 uptime |    1484 fuzz cases |       47 coverage |     4 inputs
        2.07 uptime |    2962 fuzz cases |       47 coverage |     4 inputs
        3.07 uptime |    4114 fuzz cases |       47 coverage |     4 inputs
        4.07 uptime |    5328 fuzz cases |       47 coverage |     4 inputs
        5.07 uptime |    6484 fuzz cases |       47 coverage |     4 inputs
[ perf record: Woken up 2889 times to write data ]
Warning:
Processed 582373 events and lost 60 chunks!

Check IO/CPU overload!

Warning:
9 out of order events recorded.
[ perf record: Captured and wrote 858.624 MB perf.data (99814 samples) ]

$ perf report --stdio > report
Warning:
Processed 582373 events and lost 60 chunks!

Check IO/CPU overload!

Warning:
9 out of order events recorded.
```

Top entries sorted by overhead (including children):

```
$ cat report | grep '^  *[0-9]' | sort -k1 -n -r | head
    18.75%     0.38%  mutationfuzzer7  [kernel.kallsyms]          [k] do_syscall_64
    18.75%     0.00%  mutationfuzzer7  [kernel.kallsyms]          [k] entry_SYSCALL_64_after_hwframe
    18.51%     0.78%  gcov             [kernel.kallsyms]          [k] do_syscall_64
    18.51%     0.00%  gcov             [kernel.kallsyms]          [k] entry_SYSCALL_64_after_hwframe
    17.19%     0.07%  mutationfuzzer7  libc.so.6                  [.] __GI___clone3
    14.78%     0.15%  mutationfuzzer7  mutationfuzzer7            [.] mutationfuzzer7::fuzzer::run
    14.78%     0.00%  mutationfuzzer7  mutationfuzzer7            [.] std::sys::pal::unix::thread::Thread::new::thread_start
    14.78%     0.00%  mutationfuzzer7  mutationfuzzer7            [.] std::sys::backtrace::__rust_begin_short_backtrace
    14.78%     0.00%  mutationfuzzer7  mutationfuzzer7            [.] core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::haae4dfd8690c1eb9
    14.78%     0.00%  mutationfuzzer7  libc.so.6                  [.] start_thread
```

Top entries sorted by overhead (self):

```
$ cat report | grep '^  *[0-9]' | sort -k2 -n -r | head
     4.52%     2.25%  cgi_decode       ld-linux-x86-64.so.2       [.] intel_check_word.constprop.0
     9.55%     2.03%  gcov             [kernel.kallsyms]          [k] do_user_addr_fault
     3.98%     1.98%  gcov             ld-linux-x86-64.so.2       [.] intel_check_word.constprop.0
     1.34%     1.32%  mutationfuzzer7  [kernel.kallsyms]          [k] _raw_spin_unlock_irqrestore
     2.16%     1.27%  gcov             [kernel.kallsyms]          [k] unmap_page_range
     5.65%     1.20%  cgi_decode       [kernel.kallsyms]          [k] do_user_addr_fault
     1.17%     1.16%  gcov             [kernel.kallsyms]          [k] _raw_spin_unlock_irqrestore
     1.06%     1.01%  mutationfuzzer7  [kernel.kallsyms]          [k] finish_task_switch.isra.0
     1.20%     0.97%  gcov             [kernel.kallsyms]          [k] next_uptodate_folio
     2.21%     0.87%  gcov             libc.so.6                  [.] llseek@GLIBC_2.2.5
```

Here we clearly see the cost of mutex lock / synchronization.
