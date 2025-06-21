Like previous version, but with different selection mechanism.
Slightly based on https://deap.readthedocs.io/en/master/examples/ga_onemax.html.

This implementation seems to work well.
It works both better and faster then then naive implementation from genetic-algorithm-onemax2/3,
which selected the highest fitness individuals by sorting the population.
This implementation uses tournament selection (no global sorting required) and allows defining separate crossover and mutation rates.

The hyperparameter tuning required a bit of trial and error;
reducing the `mutation_rate_bitflip` to 0.005 seems to do the trick.

Example output (note that here both population_size and genome_size have been increased considerably compared to previous iterations; nonetheless this iteration finds the optimal solution faster):

```
$ cargo run
...
Generation  226: Best Fitness = 399
Generation  227: Best Fitness = 396
Generation  228: Best Fitness = 399
Generation  229: Best Fitness = 399
Generation  230: Best Fitness = 400
Best Individual: 1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111
```
