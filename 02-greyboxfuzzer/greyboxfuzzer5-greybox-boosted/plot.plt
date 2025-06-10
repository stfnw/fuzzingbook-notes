#!/usr/bin/gnuplot

# Plot statistics of the fuzzer run.
# Adapted from https://github.com/gamozolabs/guifuzz/blob/471d744e0e46d21cad39e4287ddc6f13c9811b17/mesos/plot.plt

set title "Code Coverage during Fuzzing"

# set terminal wxt size 1000,800
set terminal pdf
set output "plot.pdf"

set xlabel "Fuzz cases"
set ylabel "Coverage"

# set yrange [0:*]
set logscale x

set grid
set key bottom

plot "plot.data" using 1:2 with lines linewidth 2 title "Greybox (boosted)"

# pause -1
