package rb.com.aoc21.day05.app

import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class GridTest {

    private var grid: Grid = Grid()

    private var lines: ArrayList<Line> = arrayListOf()

    @BeforeEach
    internal fun setUp() {
        grid = Grid()
        lines = arrayListOf()
    }

    @Test
    internal fun canMarkPoints() {
        givenLine(arrayOf(0, 3, 0, 5))

        whenMarking()

        thenGridIsMarkedOnPoints(3)
    }

    @Test
    internal fun canCountOverlapses() {
        givenLines(
            listOf(
                arrayOf(0, 3, 0, 5),
                arrayOf(0, 3, 3, 3)
            )
        )

        whenMarking()

        thenGridHasOverlapsingPoints(1)
    }

    private fun thenGridHasOverlapsingPoints(expected: Int) {
        assertEquals(expected, grid.countOverlapPoints())
    }

    private fun thenGridIsMarkedOnPoints(expected: Int) {
        assertEquals(expected, grid.markedPoints())
    }

    private fun whenMarking() {
        lines.forEach { grid mark it }
    }

    private fun givenLine(coords: Array<Int>) {
        this.lines.add(Line from coords)
    }

    private fun givenLines(newLines: List<Array<Int>>) {
        newLines.forEach(::givenLine)
    }
}
