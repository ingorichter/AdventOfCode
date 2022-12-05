//
//  2022-Day4.swift
//  Solutions for Day 4
//  https://adventofcode.com/2022/day/4
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

extension ClosedRange {
    func containsRange(_ rangeB: ClosedRange<Int>) -> Bool {
        return rangeB.lowerBound >= self.lowerBound as! Int && rangeB.upperBound <= self.upperBound as! Int
    }
}

class Day4: Day {
    func makeRange(start: Int, end: Int) -> ClosedRange<Int> {
        return start...end
    }

    func part1() async throws -> String {
        let result = input().lines.map {
            let sectionRanges = $0.csvWords
            let sectionRange1 = sectionRanges[0].raw.split(separator: "-")
            let sectionRange2 = sectionRanges[1].raw.split(separator: "-")

            let range1 = Int(sectionRange1[0])!...Int(sectionRange1[1])!
            let range2 = Int(sectionRange2[0])!...Int(sectionRange2[1])!

            return range1.containsRange(range2) || range2.containsRange(range1)
        }.count(where: { $0 == true })

        return String(result)
    }

    func part2() async throws -> String {
        let result = input().lines.map {
            let sectionRanges = $0.csvWords
            let sectionRange1 = sectionRanges[0].raw.split(separator: "-")
            let sectionRange2 = sectionRanges[1].raw.split(separator: "-")

            let range1 = Int(sectionRange1[0])!...Int(sectionRange1[1])!
            let range2 = Int(sectionRange2[0])!...Int(sectionRange2[1])!

            return range1.overlaps(range2)
        }.count(where: { $0 == true })

        return String(result)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
