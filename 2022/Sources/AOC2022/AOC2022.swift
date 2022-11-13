//
//  AOC2022.swift
//
//
//  Created by Ingo Richter on 11/13/22.
//

import Foundation
import ArgumentParser

@_exported import AOCCore

@main
struct AOC2022: AsyncParsableCommand {
    @Argument(help: "Run Day x")
    var day = 1

    mutating func run() async throws {
        guard day >= 1 && day <= 25 else {
            print("Day \(day) doesn't exist. Exit")
            return
        }

        let dayToRun = AOC2022.days[day - 1]
        print("Run Day \(day)")

        do {
            let startDate = Date()
            let (p1, p2) = try await dayToRun.run()
            let endDate = Date()

            print("Executing \(type(of: dayToRun))")
            print("Part 1: \(p1)")
            print("Part 2: \(p2)")
            print("Time: \(endDate.timeIntervalSince(startDate))")
        }
        catch {
            print("Error executing test \(day)")
        }
    }
}

extension AOC2022 {
    public static let days: Array<any Day> = [
        Day1(),
        Day2(),
        Day3(),
        Day4(),
        Day5(),
        Day6(),
        Day7(),
        Day8(),
        Day9(),
        Day10(),
        Day11(),
        Day12(),
        Day13(),
        Day14(),
        Day15(),
        Day16(),
        Day17(),
        Day18(),
        Day19(),
        Day20(),
        Day21(),
        Day22(),
        Day23(),
        Day24(),
        Day25()
    ]
}
