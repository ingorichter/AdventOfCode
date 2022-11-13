#!/usr/bin/xcrun --toolchain default swift

import Foundation

let today = Date()
let calendar = Calendar(identifier: .gregorian)

let day = calendar.component(.day, from: today)
let month = calendar.component(.month, from: today)
let year = calendar.component(.year, from: today)
let todayString = "\(month)/\(day)/\(year % 100)"

let currentDir = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
let sources = currentDir.appendingPathComponent("Sources")
let yearFolder = sources.appendingPathComponent("AOC\(year)")
let resourcesFolder = yearFolder.appendingPathComponent("Resources")

print("Creating \(yearFolder.path)")

mkdir(yearFolder)
mkdir(resourcesFolder)

let mainContents = """
//
//  AOC\(year).swift
//
//
//  Created by Ingo Richter on \(todayString).
//

import Foundation
import ArgumentParser

@_exported import AOCCore

@main
struct AOC\(year): ParsableCommand {
    @Argument(help: \"Day to run\")
    var day = 1

    mutating func run() throws {
        print(\"Run Day \\(day)\")
    }
}

"""
mainContents >> yearFolder.appendingPathComponent("AOC\(year).swift")

for day in 1 ... 25 {
    let dayFolder = yearFolder.appendingPathComponent("Day \(day)")
    mkdir(dayFolder)

    let dayContents = """
    //
    //  \(year)-Day\(day).swift
    //  Solutions for Day \(day)
    //
    //  Created by Ingo Richter on \(todayString).
    //  Copyright © \(year) Ingo Richter. All rights reserved.
    //

    class Day\(day): Day {
        static var rawInput: String? { nil }

        func part1() async throws -> String {
            return #function
        }

        func part2() async throws -> String {
            return #function
        }

        func run() async throws -> (String, String) {
            let p1 = try await part1()
            let p2 = try await part2()
            return (p1, p2)
        }
    }

    """
    dayContents >> dayFolder.appendingPathComponent("\(year)-Day\(day).swift")

    "" >> resourcesFolder.appendingPathComponent("Day\(day).txt")
}

let testFolder = currentDir.appendingPathComponent("Tests").appendingPathComponent("AOCTests")
let testFile = testFolder.appendingPathComponent("Test\(year).swift")

mkdir(testFolder)

var contents = """
//
//  Test\(year).swift
//  AOCTests
//
//  Created by Ingo Richter on \(todayString).
//  Copyright © \(year) Ingo Richter. All rights reserved.
//

import XCTest
@testable import AOC\(year)

final class Test\(year): XCTestCase {
"""

for day in 1 ... 25 {
    let testContents = """

        func testDay\(day)() async throws {
            let day = Day\(day)()
            let (part1, part2) = try await day.run()

            XCTAssertEqual(part1, "")
            XCTAssertEqual(part2, "")
        }

    """
    contents += testContents
}

contents += """

}

"""
contents >> testFile

func mkdir(_ path: URL) {
    try? FileManager.default.createDirectory(at: path, withIntermediateDirectories: true, attributes: nil)
}

infix operator >>
func >> (lhs: String, rhs: URL) {
    try? Data(lhs.utf8).write(to: rhs, options: [])
}
