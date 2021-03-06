/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package rb.com.aoc21.day01.app

import kotlinx.cli.ArgParser
import kotlinx.cli.ArgType
import kotlinx.cli.default
import kotlinx.cli.required
import mu.KotlinLogging

import org.apache.log4j.Level
import org.apache.log4j.LogManager
import rb.com.aoc21.day01.utilities.Fileutils
import rb.com.aoc21.day01.utilities.StringUtils

val logger = KotlinLogging.logger { }

fun readFrom(mode: String): List<String> {
    val filename = when(mode) {
        "example", "input" -> mode
        else -> error("Could not understand mode, valid examples are 'example' and 'input'.")
    }

    return Fileutils.readInput(filename)
}

fun main(args: Array<String>) {
    logger.debug { "Starting up ..." }
    val parser = ArgParser("day 01")

    val mode by parser
        .option(
            ArgType.Choice(listOf("example", "input"), { it }),
            shortName = "m",
            fullName = "mode",
            description = "Chose mode (example or input)",
        )
        .required()

    val problemArg by parser
        .option(
            ArgType.Choice(listOf("1", "2"), { it }),
            shortName = "p",
            fullName = "problem",
            description = "Which problem to solve?",
        ).default("1")

    val verbose by parser
        .option(
            ArgType.Boolean,
            fullName = "verbose",
            description = "Turn on verbosity"
        )
        .default(false)

    parser.parse(args)

    var problem = problemArg.toInt()

    if (verbose) {
        LogManager.getRootLogger().level = Level.DEBUG
    }

    val inputData = StringUtils.splitToInt(readFrom(mode))
    val day01 = Day01(inputData)

    day01 solve problem
}
