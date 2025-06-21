Compile an external C program (cgi_decode) and gather statement coverage from it through processing the gcov coverage data file.
Example output:

```
$ cargo run
...

cov_standard = {("cgi_decode", 15), ("cgi_decode", 16), ("cgi_decode", 17), ("cgi_decode", 19), ("cgi_decode", 20), ("cgi_decode", 21), ("cgi_decode", 22), ("cgi_decode", 23), ("cgi_decode", 24), ("cgi_decode", 25), ("cgi_decode", 26), ("cgi_decode", 27), ("cgi_decode", 28), ("cgi_decode", 30), ("cgi_decode", 31), ("cgi_decode", 32), ("cgi_decode", 33), ("cgi_decode", 34), ("cgi_decode", 35), ("cgi_decode", 37), ("cgi_decode", 38), ("cgi_decode", 39), ("cgi_decode", 40), ("cgi_decode", 41), ("cgi_decode", 42), ("cgi_decode", 43), ("cgi_decode", 45), ("cgi_decode", 46), ("cgi_decode", 47), ("cgi_decode", 49), ("cgi_decode", 57), ("cgi_decode", 58), ("cgi_decode", 60), ("cgi_decode", 61), ("cgi_decode", 64), ("cgi_decode", 65), ("cgi_decode", 67), ("cgi_decode", 68), ("cgi_decode", 70), ("cgi_decode", 71), ("cgi_decode", 72), ("cgi_decode", 73)}

cov_plus     = {("cgi_decode", 15), ("cgi_decode", 16), ("cgi_decode", 17), ("cgi_decode", 19), ("cgi_decode", 20), ("cgi_decode", 21), ("cgi_decode", 22), ("cgi_decode", 23), ("cgi_decode", 24), ("cgi_decode", 25), ("cgi_decode", 26), ("cgi_decode", 27), ("cgi_decode", 28), ("cgi_decode", 30), ("cgi_decode", 31), ("cgi_decode", 32), ("cgi_decode", 33), ("cgi_decode", 34), ("cgi_decode", 35), ("cgi_decode", 37), ("cgi_decode", 38), ("cgi_decode", 39), ("cgi_decode", 40), ("cgi_decode", 41), ("cgi_decode", 42), ("cgi_decode", 43), ("cgi_decode", 45), ("cgi_decode", 46), ("cgi_decode", 47), ("cgi_decode", 48), ("cgi_decode", 49), ("cgi_decode", 57), ("cgi_decode", 58), ("cgi_decode", 60), ("cgi_decode", 61), ("cgi_decode", 64), ("cgi_decode", 65), ("cgi_decode", 67), ("cgi_decode", 68), ("cgi_decode", 70), ("cgi_decode", 71), ("cgi_decode", 72), ("cgi_decode", 73)}

difference   = [("cgi_decode", 48)]
```
