/*
 * This Groovy source file was generated by the Gradle 'init' task.
 */
package com.rb.aoc21.day10.utilities

import com.rb.aoc21.day10.list.LinkedList

class StringUtils {
    static String join(LinkedList source) {
        return JoinUtils.join(source)
    }

    static LinkedList split(String source) {
        return SplitUtils.split(source)
    }
}
