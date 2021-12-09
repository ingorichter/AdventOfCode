//
//  Day5.swift
//  test
//
//  Created by Ingo Richter on 11/28/21.
//  Copyright © 2021 Ingo Richter. All rights reserved.
//

import AppKit

struct Point: Hashable {
    var x: Int
    var y: Int

    init(_ x: Int, _ y: Int) {
        self.x = x
        self.y = y
    }
}

extension Point {
    func vector (_ anotherPoint: Point) -> Point {
        let vx = anotherPoint.x - self.x
        let vy = anotherPoint.y - self.y

        return Point(vx, vy)
    }

    func unitVector(_ anotherPoint: Point) -> Point {
        var v = vector(anotherPoint)
        if v.x != 0 {
            v.x = v.x / abs(v.x)
        }

        if v.y != 0 {
            v.y = v.y / abs(v.y)
        }

        return Point(v.x, v.y)
    }
}

extension Point {
    static public func + (lhs: Point, rhs: Point) -> Point {
        Point(lhs.x + rhs.x, lhs.y + rhs.y)
    }

    static public func += (lhs: inout Point, rhs: Point) {
        lhs = lhs + rhs
    }
}

struct Vent {
    var start: Point
    var end: Point
}

extension Vent {
    var description: String {
        "(x=\(start),y=\(end)}"
    }
}

class Day5: Day {
    var vents: [Vent] {
        let ventRegex: Regex = #"(\d+),(\d+) -> (\d+),(\d+)"#

        let vents = input.lines.raw.map { line -> Vent in
            let m = ventRegex.firstMatch(in: line)
            let x1 = m![int: 1]
            let y1 = m![int: 2]
            let x2 = m![int: 3]
            let y2 = m![int: 4]

            return Vent(start: Point(x1!, y1!), end: Point(x2!, y2!))
        }

        return vents
    }

    func countOverlappingLines(_ vents: [Vent]) -> Int {
        var map = Dictionary<Point, Int>()

        for vent in vents {
            var pt = vent.start
            let direction = vent.start.unitVector(vent.end)

            while pt != vent.end {
                map[pt, default: 0] += 1
                pt += direction
            }
            map[pt, default: 0] += 1
        }

        return map.values.count(where: { $0 > 1 })
    }

    override func part1() -> String {
        let verticalAndHorizontalVentsOnly = vents.filter { $0.start.x == $0.end.x || $0.start.y == $0.end.y }
        let result = countOverlappingLines(verticalAndHorizontalVentsOnly)

        return "\(result)"
    }

    override func part2() -> String {
        let result = countOverlappingLines(vents)

        return "\(result)"
    }
}
