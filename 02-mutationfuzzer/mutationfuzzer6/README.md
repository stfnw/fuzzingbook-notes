Mutate an initial seed input, fuzz an external program with the input, and
gather coverage information from the program execution.
The coverage info is then fed back into the mutation phase, thus closing the
feedback loop and making the mutation fuzzer coverage guided.

Compared to the previous iteration, this one is multithreaded and can execute
multiple fuzz test cases in parallel. It also provides more scaffolding and
records more statistics, as well as saves inputs that led to new code coverage.

Example output of a particularly successful run with four threads that quickly increased coverage:

```
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/mutationfuzzer6`
[+] Running with random seed 15755513446171332859
[+] Running with random seed 15755513446171484584
[+] Running with random seed 15755513446171562501
[+] Running with random seed 15755513446171872373
        1.00 uptime |      38 fuzz cases |       42 coverage |     1 inputs
        2.00 uptime |      80 fuzz cases |       43 coverage |     2 inputs
        3.01 uptime |     121 fuzz cases |       43 coverage |     2 inputs
        4.01 uptime |     164 fuzz cases |       47 coverage |     3 inputs
        5.01 uptime |     204 fuzz cases |       47 coverage |     3 inputs
        6.01 uptime |     246 fuzz cases |       47 coverage |     3 inputs
        7.01 uptime |     286 fuzz cases |       47 coverage |     4 inputs
```

# A first attempt at profiling

Wondering whether the mutex on global fuzzer statistics data is held for too long and blocks for an unnecessary long time,
I tried to profile the code with linux perf.

```
RUSTFLAGS=-g cargo build --release
perf record --call-graph dwarf -- timeout 6 target/release/mutationfuzzer6
perf report --stdio > report
```

If I read the report correctly, from it we can see that external program execution by a very wide margin takes the most time.
Makes sense.
More importantly, the report clearly shows that *the compiler invocation* specifically is the most expensive part; here is an excerpt of the top entries by overhead; all of them concern compilation:

```
# Children      Self  Command          Shared Object              Symbol
    38.70%     0.00%  cc1              libc.so.6                  [.] __libc_start_call_main
    38.68%     0.00%  cc1              cc1                        [.] toplev::main(int, char**)
    38.67%     0.00%  cc1              cc1                        [.] main
    24.61%     0.00%  ld               libc.so.6                  [.] __libc_start_call_main
    16.97%     0.00%  cc1              cc1                        [.] 0x0000000001916893
    16.97%     0.00%  cc1              cc1                        [.] symbol_table::finalize_compilation_unit()
    16.45%     0.01%  cc1              cc1                        [.] symbol_table::compile()
    15.49%     0.00%  cc1              cc1                        [.] 0x000000000191681d
    15.49%     0.00%  cc1              cc1                        [.] c_common_parse_file()
    14.91%     0.01%  cc1              cc1                        [.] cgraph_node::expand()
    14.89%     0.00%  cc1              libc.so.6                  [.] __libc_start_main@@GLIBC_2.34
    14.73%     0.00%  cc1              cc1                        [.] _start
    14.31%     0.02%  cc1              cc1                        [.] c_parse_file()
    10.35%     3.88%  ld               libbfd-2.44-system.so      [.] bfd_elf_link_add_symbols
     9.83%     0.00%  cc1              cc1                        [.] 0x000000000142aa81
     9.83%     0.25%  cc1              cc1                        [.] c_lex_with_flags(tree_node**, unsigned int*, unsigned char*, int)
     9.21%     0.42%  cc1              cc1                        [.] _cpp_lex_token
     9.07%     0.00%  cc1              cc1                        [.] 0x00000000014143f8
     8.67%     6.04%  ld               libbfd-2.44-system.so      [.] bfd_link_hash_traverse
     8.53%     0.03%  cc1              cc1                        [.] execute_one_pass(opt_pass*)
     7.86%     0.01%  cc1              cc1                        [.] execute_pass_list(function*, opt_pass*)
     7.73%     0.00%  cc1              cc1                        [.] init_function_start(tree_node*)
     7.72%     0.00%  cc1              cc1                        [.] initialize_rtl()
     7.37%     0.00%  cc1              cc1                        [.] 0x0000000001435313
     6.56%     1.75%  cc1              cc1                        [.] _cpp_lex_direct
     5.69%     0.00%  cc1              cc1                        [.] 0x00000000014c150b
     5.26%     0.16%  cc1              cc1                        [.] _cpp_handle_directive
     5.22%     1.18%  cc1              cc1                        [.] ira_init()
     4.84%     2.12%  ld               libc.so.6                  [.] __memmove_avx_unaligned_erms
     4.77%     0.00%  ld               libbfd-2.44-system.so      [.] bfd_elf_size_dynamic_sections
     4.53%     0.29%  cc1              [kernel.kallsyms]          [k] do_syscall_64
     4.53%     0.00%  cc1              [kernel.kallsyms]          [k] entry_SYSCALL_64_after_hwframe
```

Since the program `cgi_decode.c` itself doesn't change between fuzzing invocations, the next iteration will do compilation once at the beginning of the program, and not for each fuzz case.
This should considerably increase the number of fuzz cases per time executed.

Some other interesting perf report excerpts:

Top entries for gcov / code coverage gathering:

```
     0.71%     0.00%  gcov             ld-linux-x86-64.so.2       [.] _dl_start
     0.66%     0.02%  gcov             [kernel.kallsyms]          [k] do_syscall_64
     0.66%     0.00%  gcov             [kernel.kallsyms]          [k] entry_SYSCALL_64_after_hwframe
     0.50%     0.00%  gcov             libc.so.6                  [.] __libc_start_main@@GLIBC_2.34
     0.46%     0.00%  gcov             x86_64-linux-gnu-gcov-14   [.] 0x0000000000428450
     0.46%     0.00%  gcov             ld-linux-x86-64.so.2       [.] _dl_sysdep_start
     0.45%     0.00%  gcov             libc.so.6                  [.] __libc_start_call_main
     0.45%     0.00%  gcov             ld-linux-x86-64.so.2       [.] _start
     0.34%     0.00%  gcov             ld-linux-x86-64.so.2       [.] dl_main
     0.32%     0.00%  gcov             [kernel.kallsyms]          [k] asm_exc_page_fault
     0.32%     0.07%  gcov             [kernel.kallsyms]          [k] do_user_addr_fault
     0.32%     0.00%  gcov             [kernel.kallsyms]          [k] exc_page_fault
```

Top entries for the fuzzer binary itself:

```
     0.80%     0.00%  mutationfuzzer6  [kernel.kallsyms]          [k] entry_SYSCALL_64_after_hwframe
     0.80%     0.02%  mutationfuzzer6  [kernel.kallsyms]          [k] do_syscall_64
     0.69%     0.00%  mutationfuzzer6  libc.so.6                  [.] __GI___clone3
     0.57%     0.00%  mutationfuzzer6  libc.so.6                  [.] start_thread
               mutationfuzzer6::fuzzer::run
     0.57%     0.00%  mutationfuzzer6  mutationfuzzer6            [.] std::sys::pal::unix::thread::Thread::new::thread_start
               mutationfuzzer6::fuzzer::run
     0.57%     0.00%  mutationfuzzer6  mutationfuzzer6            [.] core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h888ba1db1cfd780b
               mutationfuzzer6::fuzzer::run
     0.57%     0.00%  mutationfuzzer6  mutationfuzzer6            [.] std::sys::backtrace::__rust_begin_short_backtrace
               mutationfuzzer6::fuzzer::run
     0.57%     0.01%  mutationfuzzer6  mutationfuzzer6            [.] mutationfuzzer6::fuzzer::run
             --0.56%--mutationfuzzer6::fuzzer::run
     0.21%     0.00%  mutationfuzzer6  mutationfuzzer6            [.] std::process::Command::output
     0.17%     0.00%  mutationfuzzer6  [kernel.kallsyms]          [k] __x64_sys_execve
     0.17%     0.00%  mutationfuzzer6  [kernel.kallsyms]          [k] do_execveat_common.isra.0
     0.15%     0.00%  mutationfuzzer6  libc.so.6                  [.] __execvpex
     0.15%     0.00%  mutationfuzzer6  libc.so.6                  [.] __execvpe_common.isra.0
     0.15%     0.00%  mutationfuzzer6  libc.so.6                  [.] __GI___execve
     0.15%     0.00%  mutationfuzzer6  mutationfuzzer6            [.] std::sys::process::unix::unix::<impl std::sys::process::unix::common::Command>::spawn
     0.14%     0.00%  mutationfuzzer6  libc.so.6                  [.] __spawni_child
     0.12%     0.00%  mutationfuzzer6  [kernel.kallsyms]          [k] __do_sys_clone3
     0.12%     0.00%  mutationfuzzer6  [kernel.kallsyms]          [k] kernel_clone
     0.11%     0.00%  mutationfuzzer6  mutationfuzzer6            [.] std::sys::fs::unix::unlink
     0.11%     0.00%  mutationfuzzer6  [kernel.kallsyms]          [k] __x64_sys_unlink
```
