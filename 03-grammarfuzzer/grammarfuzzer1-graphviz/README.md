https://www.fuzzingbook.org/html/GrammarFuzzer.html

Convert a derivation tree into graphviz / dot format for visualization.

Example output:

```
digraph Derivation {

    node [shape=plain];

    n1 [label="\<start\>"];
    n2 [label="\<expr\>"];
    n1 -> n2;

    n3 [label="\<expr\>"];
    n2 -> n3;

    n4 [label="+"];
    n2 -> n4;

    n5 [label="\<term\>"];
    n2 -> n5;

}
```
