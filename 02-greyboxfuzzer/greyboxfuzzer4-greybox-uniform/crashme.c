// Example program that performs incremental comparisons (i.e. leading to new
// code coverage each time a new comparison succeeds) and then finally crashes
// on the input 'bad!' on the commandline.

#include <assert.h>
#include <string.h>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        return 0;
    }

    char *input = argv[1];
    int len = strlen(input);

    if (!(len > 0 && input[0] == 'b')) {
        return 0;
    }
    if (!(len > 1 && input[1] == 'a')) {
        return 0;
    }
    if (!(len > 2 && input[2] == 'd')) {
        return 0;
    }
    if (!(len > 3 && input[3] == '!')) {
        return 0;
    }

    return 1; // assert(0); // Crash.
}
