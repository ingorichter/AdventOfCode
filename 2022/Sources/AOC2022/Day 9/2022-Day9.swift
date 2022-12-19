//
//  2022-Day9.swift
//  Solutions for Day 9
//  https://adventofcode.com/2022/day/9
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

enum Direction {
    case Up
    case Down
    case Left
    case Right

    var x: Int {
        switch self {
        case .Left:
            return -1
        case .Right:
            return 1
        case .Up:
            return 0
        case .Down:
            return 0
        }
    }

    var y: Int {
        switch self {
        case .Left:
            return 0
        case .Right:
            return 0
        case .Up:
            return 1
        case .Down:
            return -1
        }
    }

    var steps: [Int] {
        [x, y]
    }
}

extension Direction: CustomDebugStringConvertible {
    init() {
        self = .Up
    }

    init?(_ value: String) {
        switch value.uppercased() {
        case "D": self = .Down
        case "U": self = .Up
        case "L": self = .Left
        case "R": self = .Right
        default: return nil
        }
    }

    var debugDescription: String {
        switch self {
        case .Up: return "Up"
        case .Down: return "Down"
        case .Left: return "Left"
        case .Right: return "Right"
        }
    }
}

struct Move: CustomDebugStringConvertible {
    var direction: Direction
    var steps: Int

    init(_ direction: Direction, _ steps: Int) {
        self.direction = direction
        self.steps = steps
    }

    var debugDescription: String {
        return "\(steps) Steps \(direction)"
    }
}

struct Point {
    var x: Int
    var y: Int
}

extension Point: CustomDebugStringConvertible, Hashable {
    static func == (lhs: Point, rhs: Point) -> Bool {
        lhs.x == rhs.x && lhs.y == rhs.y
    }

    mutating func adjustX(_ val: Int) {
        x += val
    }

    mutating func adjustY(_ val: Int) {
        y += val
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }

    var debugDescription: String {
        return "x: \(x) y: \(y)"
    }
}

class Day9: Day {
//    static var rawInput: String? = """
//    R 4
//    U 4
//    L 3
//    D 1
//    R 4
//    D 1
//    L 5
//    R 2
//    """

//    static var rawInput: String? = """
//    R 5
//    U 8
//    L 8
//    D 3
//    R 17
//    D 10
//    L 25
//    U 20
//    """

    func nextToEachOther(_ x1: Int, _ y1: Int, _ x2: Int, _ y2: Int) -> Bool {
        return abs(x1 - x2) <= 1 && abs(y1 - y2) <= 1
    }

    func normalize(_ v1: Int, _ v2: Int) -> Int {
        return (v1 - v2) / abs(v1 - v2)
    }

    func traceMovements(_ movements: [Move], _ numberOfKnots: Int) -> Set<Point> {
        var positions = Set<Point>()

        var headX: Int = 10
        var headY: Int = 10
        var tailX: Int = 10
        var tailY: Int = 10

        // track all knot positions
        var knotPositions: [Point] = Array(repeating: Point(x: 10, y: 10), count: numberOfKnots)

        // starting position
        positions.insert(Point(x: tailX, y: tailY))

        movements.forEach { move in
            for _ in 0..<move.steps {
                knotPositions[0].x += move.direction.x
                knotPositions[0].y += move.direction.y

                for i in 1..<knotPositions.count {
                    headX = knotPositions[i - 1].x
                    headY = knotPositions[i - 1].y
                    tailX = knotPositions[i].x
                    tailY = knotPositions[i].y

                    if !nextToEachOther(headX, headY, tailX, tailY) {
                        tailX += (headX == tailX ? 0 : (normalize(headX, tailX)))
                        tailY += (headY == tailY ? 0 : (normalize(headY, tailY)))
                    }

                    knotPositions[i].x = tailX
                    knotPositions[i].y = tailY
                }

                positions.insert(knotPositions.last!)
            }
        }

        return positions
    }

    func part1() async throws -> String {
        let movements = input().lines.map {
            let m = $0.raw.split(on: " ")
            return Move(Direction(String(m[0]))!, Int(m[1])!)
        }

        let positions = traceMovements(movements, 2)

        return String(positions.count)
    }

    func part2() async throws -> String {
        let movements = input().lines.map {
            let m = $0.raw.split(on: " ")
            return Move(Direction(String(m[0]))!, Int(m[1])!)
        }

        let positions = traceMovements(movements, 10)

        return String(positions.count)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
