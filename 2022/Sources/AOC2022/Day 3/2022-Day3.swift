//
//  2022-Day3.swift
//  Solutions for Day 3
//  https://adventofcode.com/2022/day/3
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

extension Line {
    func splitInHalf() -> [String] {
        let distance = self.raw.distance(from: self.raw.startIndex, to: self.raw.endIndex)

        let middleIndex = self.raw.index(self.raw.startIndex, offsetBy: distance / 2)
        let firstHalf = self.raw[self.raw.startIndex..<middleIndex]
        let secondHalf = self.raw[middleIndex..<self.raw.endIndex]

        return [String(firstHalf), String(secondHalf)]
    }
}

func toInt(unsigned: UInt) -> Int {

    let signed = (unsigned <= UInt(Int.max)) ?
        Int(unsigned) :
        Int(unsigned - UInt(Int.max) - 1) + Int.min

    return signed
}

func mapItem(_ element: String.Element) -> Int {
    var result = 0
    if element.isLowercase {
        result = (toInt(unsigned: UInt(element.asciiValue!)) - 97 + 1)
    } else {
        result = (toInt(unsigned: UInt(element.asciiValue!)) - 65 + 27)
    }

    return result
}

// ASCII A=65, Z=90
// a=97, z=122
class Day3: Day {
    func part1() async throws -> String {
        let total = input().lines.reduce(0) { acc, line in
            let halves = line.splitInHalf()
            let firstHalfSet = Set(halves[0])
            let secondHalfSet = Set(halves[1])
            let value = firstHalfSet.intersection(secondHalfSet).reduce(0) {
                return $0 + mapItem($1)
            }

            return acc + value
        }

        return String(total)
    }

    func groupBadgeItem(group: ArraySlice<Line>) -> Int {
        let firstElveItems = Set(group[group.index(group.startIndex, offsetBy: 0)].raw)
        let secondElveItems = Set(group[group.index(group.startIndex, offsetBy: 1)].raw)
        let thirdElveItems = Set(group[group.index(group.startIndex, offsetBy: 2)].raw)

        let groupItemSet = firstElveItems.intersection(secondElveItems).intersection(thirdElveItems)

        return groupItemSet.reduce(0) {
            return $0 + mapItem($1)
        }
    }

    func part2() async throws -> String {
        let total = input().lines.chunks(ofCount: 3).reduce(0) { acc, group in
            return acc + groupBadgeItem(group: group)
        }

        return String(total)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
