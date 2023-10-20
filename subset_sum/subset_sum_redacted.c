#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FLAG_PREFIX  "caroloctf"
#define FLAG_CONTENT "REDACTED"

char flag[sizeof(FLAG_PREFIX) + sizeof(FLAG_CONTENT) + 17];
long values[100];
char max_index = sizeof(values) / sizeof(values[0]);

int main(int argc, char **argv) {
    // build flag
    char *current = flag;
    current = stpcpy(current, FLAG_PREFIX);
    *current = '{';
    current++;
    current = stpcpy(current, FLAG_CONTENT);
    *current = '}';

    // Direct memory access works
    /*
    char overflow = 127;
    for (char i = 121; i < 126; i += 1) {
        char test = overflow + i;
        long binky = values[test];
        for (int j = 0; j < 8; ++j) {
            char c = binky >> (j * 8);
            printf("%c", c);
        }
        //printf("%ld", binky);
    }
    return 0;
     */

    // Scrambled access through summation needs to be compensated for somehow
    char overflow = 127;
    for (char i = 121; i < 126; i += 1) {
        char end = overflow + i;
        long binky = 0;
        for (char x = overflow; x != end; x++) {
            binky += values[x];
            printf("%ld \n", values[x]);
        }
        for (int j = 0; j < 8; ++j) {
            char c = binky >> (j * 8);
            //printf("%c", c);
        }
        //printf("%ld", binky);
    }

    return 0;


    // run application
    while (1) {
        // print menu
        puts("---- Menu ----");
        puts("1: change value");
        puts("2: calculate sum");
        puts("3: exit");
        printf("choice: ");
        char choice;
        scanf("%hhd", &choice);
        switch (choice) {
            case 1:
                printf("index: ");
                char index;
                scanf("%hhd", &index);
                if (index < 0 || index >= max_index) {
                    printf("Index must be between 0 and %d\n", max_index);
                } else {
                    printf("value: ");
                    scanf("%ld", values + index);
                }
                break;
            case 2:
                char start, len;
                printf("start index: ");
                scanf("%hhd", &start);
                printf("sum length: ");
                scanf("%hhd", &len);
                char end = start + len;
                if (start < 0 || len < 0 || end > max_index) {
                    puts("The selected area is at least partially out of range.");
                } else {
                    long sum = 0;
                    for (char i = start; i != end; i++) {
                        sum += values[i];
                    }
                    printf("sum = %ld\n", sum);
                }
                break;
            default:
                exit(EXIT_SUCCESS);
        }
    }
}
