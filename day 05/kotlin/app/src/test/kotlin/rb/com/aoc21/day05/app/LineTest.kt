package rb.com.aoc21.day05.app

import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test

internal class LineTest {

    val line_horizontal = Line from arrayOf(0, 3, 5, 3)
    val line_vertical = Line from arrayOf(0, 3, 0, 5)
    val line_diagonal = Line from arrayOf(0, 0, 2, 2)


    @Test
    internal fun horizontalTest() {
        val line = line_horizontal
        val points = line.points()
        kotlin.test.assertEquals(
            listOf(
                Point from (0 to 3),
                Point from (1 to 3),
                Point from (2 to 3),
                Point from (3 to 3),
                Point from (4 to 3),
                Point from (5 to 3)
            ),
            points
        )
    }

    @Test
    internal fun shouldGoBackwardsHorizontally() {
        val line = Line from arrayOf(5, 3, 3, 3)

        val points = line.points()

        kotlin.test.assertEquals(
            sortedSetOf(
                Point from (5 to 3),
                Point from (4 to 3),
                Point from (3 to 3),
            ).toList(),
            points
        )
    }

    @Test
    internal fun verticalTest() {
        val line = line_vertical

        val points = line.points()

        kotlin.test.assertEquals(
            listOf(
                Point from (0 to 3),
                Point from (0 to 4),
                Point from (0 to 5),
            ),
            points
        )
    }

    @Test
    internal fun diagonalTest() {
        val line = line_diagonal

        val points = line.points()

        kotlin.test.assertEquals(
            listOf(
                Point from (0 to 0),
                Point from (1 to 1),
                Point from (2 to 2),
            ),
            points
        )
    }

    @Test
    internal fun isHorizontal() {
        val line = line_horizontal

        assertTrue(line.isHorizontal())
    }

    @Test
    internal fun isNotHorizontal() {
        val line = line_vertical

        assertFalse(line.isHorizontal())
    }

    @Test
    internal fun isVertical() {
        val line = line_vertical

        assertTrue(line.isVertical())
    }

    @Test
    internal fun isNotVertical() {
        val line = line_horizontal

        assertFalse(line.isVertical())
    }

    @Test
    internal fun isSimple() {
        assertTrue(line_vertical.isSimple(), "vertical")
        assertTrue(line_horizontal.isSimple(), "horizontal")
        assertFalse(line_diagonal.isSimple(), "diagonal")
    }
}
