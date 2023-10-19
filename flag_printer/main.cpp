#include <iostream>
#include <array>


long recursion(long i, long global1);

int main() {
    std::array<long, 24> key_global_0 = {10847947, 3866210, 9301910, 2126234, 10958801, 13046951, 9621629, 8807611, 9823006, 1139417, 518677, 6720288, 2050213, 2638900, 8225281, 599975, 11184147, 1130648, 286914, 7607814, 2710715, 4264935, 1251426, 3611062};

    for (uint i = 0; i < 24; i = i + 1) {
        long flag = recursion(key_global_0[i], 0x4B7D33);
        printf("%s",(char *)&flag);
    }

    return 0;
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


long recursion(long key, long break_cond) {
    while (break_cond != 1) {
        key = (key + break_cond - 1) % 0xD6B833;
        break_cond--;
    }
    return key;
}
