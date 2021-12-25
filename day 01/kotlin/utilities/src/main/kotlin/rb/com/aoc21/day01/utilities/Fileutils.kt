package rb.com.aoc21.day01.utilities

import java.io.File
import java.math.BigInteger
import java.security.MessageDigest

class Fileutils {
    companion object {
        /**
         * Reads lines from the given input txt file.
         */
        fun readInput(name: String) = File("src", "$name.txt").readLines()

        /**
         * Converts string to md5 hash.
         */
        fun String.md5(): String = BigInteger(1, MessageDigest.getInstance("MD5").digest(toByteArray())).toString(16)
    }
}