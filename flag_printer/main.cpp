#include <iostream>
#include <array>


long recursion(long i, long global1);

int main() {
    std::array<long, 24> key_global_0 = {
            0xA586CB, 0x3AFE62, 0x8DEF96, 0x20719A,
            0xA737D1, 0xC714A7, 0x92D07D, 0x8664BB,
            0x95E31E, 0x1162D9, 0x7EA15, 0x668B20,
            0x1F48A5, 0x284434, 0x7D8201, 0x927A7,
            0xAAA813, 0x114098, 0x460C2, 0x741606,
            0x295CBB, 0x4113E7, 0x131862, 0x3719B6,
    };


    for (uint i = 0; i < 24; i = i + 1) {
        long flag = recursion(key_global_0[i], 0x4B7D33);
        printf("%s", &flag);
    }

    return 0;
}

long recursion(long key, long break_cond) {
    while (break_cond != 1) {
        key = (key + break_cond - 1) % 0xD6B833;
        break_cond--;
    }
    return key;
}

//long recursion(long key, long break_cond) {
//    long i;
//
//    if (break_cond != 1) {
//        i = recursion(key, break_cond - 1);
//        key = (i + key) % 0xD6B833;
//    }
//    return key;
//}

