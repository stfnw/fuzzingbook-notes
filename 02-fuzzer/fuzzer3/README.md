Simple fuzzer creating random ASCII strings.
Introduce separate `runner` for doing something with the randomly generated values (here a runner executes an external program and feeds it the value).
Some example output:

```
$ cargo run
...
Pass SFWHHDNOVRKWWC
Pass WDOQNSOUZAFVAQAOYS
Pass FPOSYKAPUEZBEPCBGDC
Pass UNEQANZLCD
Pass UTAZFXQPATHBD
Pass FPQZQCFXPPCXQLDI
Pass SSQUGTTZQDUFGOIKIN
Pass JRHUCVCPAMFWBAXZMGW
Pass PVUCGOXAFCD
Pass EADZGOUDJNLLZDBRXKD
```
