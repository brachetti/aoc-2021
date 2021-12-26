package rb.com.aoc21.day05.app

import mu.KotlinLogging

class HydrothermalGrid(private val inputs: List<String>) {
    private var grid = Grid()
    private var lines: List<Line> = arrayListOf()

    companion object {
        private val logger = KotlinLogging.logger { }
    }

    infix fun solve(problem: Int) {
        logger.info { "Solving problem $problem" }
        logger.debug { "input: ${inputs.subList(0, 4)}..." }
        destructureInput()

        when (problem) {
            1 -> problem01()
            2 -> problem02()
        }

        logger.info { "Final grid is $grid" }
        val result = countOverlapPoints()
        logResult(result)
    }

    private fun problem02() {
        logger.info { "At how many points do at least two lines overlap?" }
        applyLines(includeDiagonals = true)
    }

    private fun destructureInput() {
        lines = inputs.map(Line::from)
        logger.debug { "Lines: ${lines.subList(0, 4)}..." }
    }

    private fun problem01() {
        logger.info { "At how many points do at least two lines overlap?" }
        applyLines()
    }

    private fun applyLines(includeDiagonals: Boolean = false) {
        logger.debug { "Applying lines ..." }

        val relevantLines = when (includeDiagonals) {
            true -> lines
            false -> lines.filter(Line::isSimple).toList()
        }

        logger.debug { "Applying lines ... count of ${relevantLines.size}" }
        relevantLines.forEach { grid mark it }
        logger.debug { "Applying lines ... Done" }
    }

    private fun countOverlapPoints(): Int {
        return grid.countOverlapPoints()
    }

    private fun logResult(result: Int) {
        logger.info { "I can count $result overlaps" }
    }
}
