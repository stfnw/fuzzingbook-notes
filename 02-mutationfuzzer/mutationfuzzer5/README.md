Mutate an initial seed input, fuzz an external program with the input, and
gather coverage information from the program execution.
The coverage info is then fed back into the mutation phase, thus closing the
feedback loop and making the mutation fuzzer coverage guided.

Compared to the previous iteration, this one provides some scaffolding and a
refactored program structure for evolving/managing a population as well as
plotting the coverage achieved against the number of performed fuzz cases using
gnuplot.
