//
//  Day.swift
//  test
//
//  Created by Dave DeLong on 12/22/17.
//  Copyright © 2017 Dave DeLong. All rights reserved.
//

private let yearRegex = Regex(pattern: #"/AOC(\d+)/"#)
private let dayRegex = Regex(pattern: #".+?Day (\d+).+?\.txt$"#)
private let classNameRegex = Regex(pattern: #"AOC(\d+).Day(\d+)"#)

open class Day: NSObject {

    public static func day(for date: Date) -> Day {
        var calendar = Calendar(identifier: .gregorian)
        calendar.timeZone = TimeZone(identifier: "America/New_York")!
        let components = calendar.dateComponents([.year, .day], from: Date())
        return day(for: components.year!, day: components.day!)
    }

    public static func day(for year: Int, day: Int) -> Day {
        Year(year).day(day)
    }

    private static let inputFiles: [Pair<Int>: String] = {
        let root = URL(fileURLWithPath: "\(#file)").deletingLastPathComponent().deletingLastPathComponent()
        let enumerator = FileManager.default.enumerator(at: root, includingPropertiesForKeys: nil)

        var files = [Pair<Int>: String]()

        while let next = enumerator?.nextObject() as? URL {
            guard let year = yearRegex.match(next.path)?.int(1) else {
                continue
            }
            guard let day = dayRegex.match(next.path)?.int(1) else {
                continue
            }

            files[Pair(year, day)] = next.path
        }

        return files
    }()

    public let input: Input

    public init(rawInput: String) {
        input = Input(rawInput)
        super.init()
    }

    public override init() {
        let name = String(cString: class_getName(type(of: self)))
        let match = name.match(classNameRegex)
        let year = match.int(1)!
        let day = match.int(2)!

        if let onDiskInputFile = Day.inputFiles[Pair(year, day)] {
            input = Input(file: onDiskInputFile)
        } else {
            input = Input("")
        }
        super.init()
    }

    open func run() -> (String, String) {
        autoreleasepool {
            (part1(), part2())
        }
    }

    open func part1() -> String {
        fatalError("Implement \(#function)")
    }

    open func part2() -> String {
        fatalError("Implement \(#function)")
    }
}
