//
//  Day2.swift
//  test
//
//  Created by Ingo Richter on 11/28/21.
//  Copyright © 2021 Ingo Richter. All rights reserved.
//

enum Command
{
    case forward(Int)
    case up(Int)
    case down(Int)
}

class Day2: Day {
    func commands() -> [Command]
    {
        let commandRegex: Regex = #"(up|down|forward) (\d+)"#

        let commands = input.lines.raw.map { line -> Command in
            let m = commandRegex.firstMatch(in: line)
            let moveDir = m![1]
            let amount = m![int: 2]

            switch moveDir {
            case "up": return .up(amount ?? 0)
            case "down": return .down(amount ?? 0)
            case "forward": return .forward(amount ?? 0)
            default: fatalError()
            }
        }

        return commands
    }

    override func part1() -> String {
        var horizontalPosition = 0
        var depth = 0

        commands().forEach { command in
            switch command {
            case .up(let value): depth -= value
            case .down(let value): depth += value
            case .forward(let value): horizontalPosition += value
            }
        }

        return "\(horizontalPosition * depth)"
    }

    override func part2() -> String {
        var horizontalPosition = 0
        var depth = 0
        var aim = 0

        commands().forEach { command in
            switch command {
            case .up(let value): aim -= value
            case .down(let value): aim += value
            case .forward(let value):
                horizontalPosition += value
                depth += aim * value
            }
        }

        return "\(horizontalPosition * depth)"
    }
}
