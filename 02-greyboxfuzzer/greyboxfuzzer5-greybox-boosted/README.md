https://www.fuzzingbook.org/html/GreyboxFuzzer.html

Greybox mutation-based fuzzer (with coverage guidance and preferredly choosing inputs that lead to new coverage).
Somehow in my experiments this doesn't really work better than the previous iteration.

```
[+] Running with random seed 15755614142247373161
Fuzz case 0
Fuzz case 200
Fuzz case 400
Fuzz case 600
Fuzz case 800
Fuzz case 1000
Fuzz case 1200
Fuzz case 1400
Fuzz case 1600
Fuzz case 1800
Fuzz case 2000
Fuzz case 2200
Fuzz case 2400
Fuzz case 2600
Fuzz case 2800
Fuzz case 3000
Fuzz case 3200
Fuzz case 3400
Fuzz case 3600
Fuzz case 3800

[+] Boosted greybox mutation-based fuzzer:
    - Runtime:                        14.5623s
    - Inputs leading to new coverage: {b'ood: CoverageH(14584952713502014028), ba'okd: CoverageH(861858939690744263), badI: CoverageH(15157251526510212828), good: CoverageH(8465429693363110185)}
    - All coverage:                   12 {8, 9, 13, 14, 16, 17, 19, 20, 22, 23, 25, 26}
    - Coverage frequencies: {
    CoverageH(
        861858939690744263,
    ): 368,
    CoverageH(
        8465429693363110185,
    ): 2588,
    CoverageH(
        14584952713502014028,
    ): 957,
    CoverageH(
        15157251526510212828,
    ): 87,
}
{
    b'ood: CoverageH(
        14584952713502014028,
    ),
    ba'okd: CoverageH(
        861858939690744263,
    ),
    badI: CoverageH(
        15157251526510212828,
    ),
    good: CoverageH(
        8465429693363110185,
    ),
}
```
