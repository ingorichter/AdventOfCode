#!/usr/bin/xcrun --toolchain default swift

import Foundation

let today = Date()
let calendar = Calendar(identifier: .gregorian)

let day = calendar.component(.day, from: today)
let month = calendar.component(.month, from: today)
let year = calendar.component(.year, from: today)
let todayString = "\(month)/\(day)/\(year % 100)"

let u = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
let sources = u.appendingPathComponent("Sources")
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

@_exported import AOCCore
"""
mainContents >> yearFolder.appendingPathComponent("AOC\(year).swift")

for day in 1 ... 25 {
    let dayFolder = yearFolder.appendingPathComponent("Day \(day)")
    mkdir(dayFolder)

    let dayContents = """
    //
    //  Day\(day).swift
    //  test
    //
    //  Created by Ingo Richter on \(todayString).
    //  Copyright © \(year) Ingo Richter. All rights reserved.
    //

    class Day\(day): Day {
        override func run() -> (String, String) {
            super.run()
        }

        override func part1() -> String {
            #function
        }

        override func part2() -> String {
            #function
        }
    }

    """
    dayContents >> dayFolder.appendingPathComponent("\(year)-Day\(day).swift")

    "" >> resourcesFolder.appendingPathComponent("Day\(day).txt")
}

let testFile = u.appendingPathComponent("Tests").appendingPathComponent("AOCTests").appendingPathComponent("Test\(year).swift")

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

class Test\(year): XCTestCase {
"""

for day in 1 ... 25 {
    let testContents = """

        func testDay\(day)() {
            let day = Day\(day)()
            let (part1, part2) = day.run()

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
    try! Data(lhs.utf8).write(to: rhs, options: [])
}
