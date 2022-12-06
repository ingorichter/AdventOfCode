//
//  2022-Day5.swift
//  Solutions for Day 5
//  https://adventofcode.com/2022/day/5
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

class Stack<T> {
    private var elements: [T] = []

    func push(elem: T) {
        elements.append(elem)
    }

    func pop() -> T {
        return elements.removeLast()
    }

    func top() -> T {
        return elements.last!
    }

    func debugDescription() -> String {
        return elements.debugDescription
    }
}

class Step {
    var from: Int
    var to: Int
    var quantity: Int

    private let moveInstructionRegex = Regex(#"move (\d+) from (\d+) to (\d+)"#)

    convenience init(line: String) {
        self.init(from: 0, to: 0, amount: 0)

        let match = moveInstructionRegex.matches(in: line)
        quantity = match[0].int(1)!
        from = match[0].int(2)!
        to = match[0].int(3)!
    }

    init(from: Int, to: Int, amount: Int) {
        self.from = from
        self.to = to
        self.quantity = amount
    }

    func debugDescription() -> String {
        return "move \(quantity) from \(from) to \(to)"
    }
}

class Day5: Day {
    //    static var rawInput: String? = """
    //        [D]
    //    [N] [C]
    //    [Z] [M] [P]
    //     1   2   3
    //
    //    move 1 from 2 to 1
    //    move 3 from 1 to 3
    //    move 2 from 2 to 1
    //    move 1 from 1 to 2
    //    """

    func rearrangeCratesOnStacksWithCrateMover9000(stacks: [Stack<Character>], moves: [Step]) -> [Stack<Character>] {
        moves.forEach { move in
            for _ in 0..<move.quantity {
                let crate = stacks[move.from - 1].pop()
                stacks[move.to - 1].push(elem: crate)
            }
        }
        return stacks
    }

    func rearrangeCratesOnStacksWithCrateMover9001(stacks: [Stack<Character>], moves: [Step]) -> [Stack<Character>] {
        moves.forEach { move in
            let tempStack = Stack<Character>()
            for _ in 0..<move.quantity {
                tempStack.push(elem: stacks[move.from - 1].pop())
            }

            for _ in 0..<move.quantity {
                stacks[move.to - 1].push(elem: tempStack.pop())
            }
        }
        return stacks
    }

    func makeCrates(_ line: Line, numberOfStacks: Int) -> [Character] {
        let offset = 4
        var crates: [Character] = Array.init(repeating: " ", count: numberOfStacks)

        let chars = Array(line.raw)

        stride(from: 0, to: chars.count, by: offset).forEach { index in
            if chars[index] == "[" {
                crates[index / offset] = chars[index + 1]
            }
        }

        return crates
    }

    func makeStacks(_ input: ArraySlice<Line>) -> [Stack<Character>] {
        var stacks: [Stack<Character>] = []

        let reversedInput = input.reversed()
        let numberOfStacks = reversedInput.first!.digits

        numberOfStacks.forEach { _ in
            stacks.append(Stack())
        }

        reversedInput.dropFirst().forEach { line in
            let crates = makeCrates(line, numberOfStacks: numberOfStacks.count)

            crates.enumerated().forEach { index, crate in
                if crate != " " {
                    stacks[index].push(elem: crate)
                }
            }
        }

        return stacks
    }

    func makeMoves(_ input: ArraySlice<Line>) -> [Step] {
        return input.map { line in
            return Step(line: line.raw)
        }
    }

    func part1() async throws -> String {
        let inputParts = input().lines.split(on: { $0.raw.isEmpty })

        let stacks: [Stack] = makeStacks(inputParts[0])
        let moves: [Step] = makeMoves(inputParts[1])

        let adjustedStacks = rearrangeCratesOnStacksWithCrateMover9000(stacks: stacks, moves: moves)

        return adjustedStacks.reduce("") {
            return $0 + String($1.top())
        }
    }

    func part2() async throws -> String {
        let inputParts = input().lines.split(on: { $0.raw.isEmpty })

        let stacks: [Stack] = makeStacks(inputParts[0])
        let moves: [Step] = makeMoves(inputParts[1])

        let adjustedStacks = rearrangeCratesOnStacksWithCrateMover9001(stacks: stacks, moves: moves)

        return adjustedStacks.reduce("") {
            return $0 + String($1.top())
        }
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
