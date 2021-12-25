package rb.com.aoc21.day01.app

import mu.KotlinLogging

class Day01(private val inputs: List<Int>) {

    companion object {
        private val logger = KotlinLogging.logger {}
    }

    infix fun solve(problem: Int) {
        logger.info { "Solving problem $problem" }
        logger.debug { "input: ${inputs.subList(0, 10)}..." }

        when (problem) {
            1 -> problem01()
            2 -> problem02()
        }
    }

    private fun problem02() {
        logger.info { "Count the number of times the sum of measurements in this sliding window increases from the previous sum." }
        val windows = inputs.windowed(3)
        logger.debug { "Windows: ${windows.subList(0, 4)}..." }
        val windowSums = windows.map { it.sumOf { it } }
        logger.debug { "Window Sums: ${windowSums.subList(0, 4)}..." }
        val increaseCount = increaseCountFor(windowSums)
        logger.info { "Increases: $increaseCount" }
    }

    private fun problem01() {
        logger.info { "Count the number of times a depth measurement increases from the previous measurement." }
        val increaseCount = increaseCountFor(inputs)
        logger.info { "Increases: $increaseCount" }
    }

    private infix fun increaseCountFor(inputs: List<Int>): Int {
        var increaseCount = 0
        logger.debug { "${inputs.first()} -> init" }
        inputs.forEachIndexed { index, elem ->
            if (index == 0) {
                return@forEachIndexed
            }
            val elementBefore = inputs.get(index - 1)
            if (elem > elementBefore) {
                increaseCount += 1
                logger.debug { "$elem -> increase" }
            } else {
                logger.debug { "$elem" }
            }
        }
        return increaseCount
    }
}