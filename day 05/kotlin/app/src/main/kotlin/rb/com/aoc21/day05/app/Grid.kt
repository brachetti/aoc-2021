package rb.com.aoc21.day05.app

import mu.KotlinLogging
import java.lang.Integer.min
import java.util.*
import kotlin.math.max
import kotlin.math.sqrt


class Point(internal val x: Int, internal val y: Int) : Comparable<Point> {
    companion object {
        infix fun from(input: String): Point {
            val (x, y) = input.split(",")
            return from(x.toInt(), y.toInt())
        }

        infix fun from(coords: List<Int>): Point {
            val (x, y) = coords
            return from(x to y)
        }

        infix fun from(pair: Pair<Int, Int>): Point {
            return from(pair.first, pair.second)
        }

        fun from(x: Int, y: Int): Point {
            return Point(x, y)
        }
    }

    override fun toString(): String {
        return "$x,$y"
    }

    override fun equals(other: Any?): Boolean {
        if (other is Point) {
            return x == other.x && y == other.y
        }
        return false
    }

    override fun hashCode(): Int {
        var result = x
        result = 31 * result + y
        return result
    }

    private fun len(): Double {
        return sqrt(((x * x) + (y * y)).toDouble())
    }

    override fun compareTo(other: Point): Int {
        return len().compareTo(other.len())
    }


}

class Line(private val from: Point, private val to: Point) {
    companion object {
        infix fun from(input: String): Line {
            val (a, b) = input.split(" -> ")
            return from(
                Point from a,
                Point from b
            )
        }

        infix fun from(coords: Array<Int>): Line {
            val (x1, y1, x2, y2) = coords
            return from(Point.from(x1, y1), Point.from(x2, y2))
        }

        infix fun from(pair: Pair<Point, Point>): Line {
            return from(pair.first, pair.second)
        }

        fun from(a: Point, b: Point): Line {
            return Line(a, b)
        }
    }

    override fun toString(): String {
        return "$from -> $to"
    }

    fun isSimple(): Boolean {
        return isHorizontal() || isVertical()
    }

    fun points(): List<Point> {
        val points = sortedSetOf<Point>()
        if (isSimple()) {
            if (isHorizontal()) {
                val low = min(from.x, to.x)
                val high = max(from.x, to.x)
                val other = from.y
                (low..high).forEach { x ->
                    points.add(Point.from(x, other))
                }
            } else if (isVertical()) {
                val low = min(from.y, to.y)
                val high = max(from.y, to.y)
                val other = from.x
                (low..high).forEach { y ->
                    points.add(Point.from(other, y))
                }
            }
        } else {
            (from.x..to.x).zip(from.y..to.y).map(Point::from).forEach(points::add)
        }

//        points.sor
        logger.debug { "Points on line $this: $points" }
        return points.toList()
    }

    fun isHorizontal(): Boolean {
        return from.y == to.y
    }

    fun isVertical(): Boolean {
        return from.x == to.x
    }
}

class Grid {
    companion object {
        val logger = KotlinLogging.logger {}
    }

    var contents: SortedMap<Point, Int> = TreeMap()

    fun countOverlapPoints(): Int {
        val filterValues = contents.filterValues { it > 1 }
        logger.debug { "Overlap Points are: $filterValues" }
        return filterValues.size
    }

    infix fun mark(point: Point) {
        logger.debug { "Marking Point $point" }
        if (contents.containsKey(point)) {
            val counter = contents.getValue(point)
            contents.set(point, counter + 1)
        } else {
            contents.put(point, 1)
        }

    }

    infix fun mark(line: Line) {
        logger.debug { "Marking Line $line" }
        line.points().forEach(::mark)
        logger.debug { "Marking Line Done" }
        logger.debug { "Resulting Grid is $this" }
    }

    fun markedPoints(): Int {
        return contents.size
    }

    private infix fun pointToString(pair: Pair<Int, Int>): String {
        val key = Point from (pair.first to pair.second)
        var value = contents.get(key) ?: 0
        value = min(value, 9)

        return when (value) {
            in 1..9 -> value.toString()
            else -> "."
        }
    }

    override fun toString(): String {
        val sb = StringBuilder("\n")
        val (maxX, maxY) = getDimensions()
        (0..maxX).forEach { x ->
            (0..maxY).forEach { y ->
                sb.append(pointToString(x to y))
            }
            sb.append("\n")
        }
        return sb.toString()
    }

    private fun getDimensions(): Pair<Int, Int> {
        val maxX = contents.keys.sortedBy(Point::x).last().x
        val maxY = contents.keys.sortedBy(Point::y).last().y
        return maxX to maxY
    }
}
