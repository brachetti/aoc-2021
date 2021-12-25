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
        }
    }

    private fun problem01() {
        logger.info { "Count the number of times a depth measurement increases from the previous measurement." }
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
        logger.info { "Increases: $increaseCount" }
    }
}