package ctf;

import java.io.Console;

public class FlagChecker {
        public static char[] wanted = new char[]{'K', 'M', 'Z', 'W', 'X', 'W', 'K', '`', 'N', 'c', 'K', 'N', 'Z', 'G', '\u009d', '¡', 'G', ' ', 'G', 'O', '\u009c', '\u009c', 'P', 'G', 'P', '\u009b', 'K', '\u009c', 'Y', '\\', '\u009d', 'X', '\u009b', 'Z', 'G', 'N', '\u009c', 'Z', 'G', 'R', ' ', '^', ' ', 'i'};

        public FlagChecker() {
        }

        public static void main(String[] var0) {

            String result = "";
            for(int i=0; i<wanted.length; ++i) {
                result += (char)((wanted[i]-42) ^ 66);
            }

            System.out.println(result);
        }
    }

