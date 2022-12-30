//
//  2022-Day11.swift
//  Solutions for Day 11
//  https://adventofcode.com/2022/day/11
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

typealias MonkeyItemType = Int

class Monkey {
    var id: Int
    var items: [MonkeyItemType]
    var operation: String
    var value: String
    var divisor: MonkeyItemType
    var testPositiveNextMonkeyId: Int
    var testNegativeNextMonkeyId: Int
    var inspectedItems: Int = 0

    init(_ id: Int = 0, items: [MonkeyItemType] = [], operation: String = "", value: String = "", divisor: MonkeyItemType = 1, testPositiveMonkey: Int = 0, testNegativeMonkey: Int = 0) {
        self.id = id
        self.items = items
        self.operation = operation
        self.value = value
        self.divisor = divisor
        self.testPositiveNextMonkeyId = testPositiveMonkey
        self.testNegativeNextMonkeyId = testNegativeMonkey
    }

    func removeAllItems() {
        inspectedItems += self.items.count
        self.items = []
    }

    func throwItemToWhichMonkey(_ worryLevel: MonkeyItemType) -> Int {
        worryLevel.isMultiple(of: self.divisor) ? testPositiveNextMonkeyId : testNegativeNextMonkeyId
    }

    func newWorryLevel(_ oldWorryLevel: MonkeyItemType) -> MonkeyItemType {
        var newWorryLevel: MonkeyItemType = 0
        var val: MonkeyItemType = 0

        if value == "old" {
            val = oldWorryLevel
        } else {
            val = MonkeyItemType(self.value)!
        }

        switch operation {
        case "+": newWorryLevel = oldWorryLevel + val
        case "-": newWorryLevel = oldWorryLevel - val
        case "/": newWorryLevel = oldWorryLevel / val
        case "*": newWorryLevel = oldWorryLevel * val
        default:
            print("Unsupported operation")
        }

        return newWorryLevel
    }
}

extension Monkey: CustomDebugStringConvertible {
    var debugDescription: String {
        return "Monkey #\(id) with items \(items) inspected \(inspectedItems)"
    }
}

class Day11: Day {
    private let monkeyEntryRegex = Regex(#"Monkey (\d+):"#)
    private let operationRegex = Regex(#"\s+Operation: new = old ([\+\-\/\*]) (.*)$"#)
    private let testRegex = Regex(#"\s+Test: divisible by (\d+)$"#)
    private let testResultRegex = Regex(#"\s?If \b(true|false): throw to monkey (\d+)$"#)

//    static var rawInput: String? = """
//    Monkey 0:
//      Starting items: 79, 98
//      Operation: new = old * 19
//      Test: divisible by 23
//        If true: throw to monkey 2
//        If false: throw to monkey 3
//
//    Monkey 1:
//      Starting items: 54, 65, 75, 74
//      Operation: new = old + 6
//      Test: divisible by 19
//        If true: throw to monkey 2
//        If false: throw to monkey 0
//
//    Monkey 2:
//      Starting items: 79, 60, 97
//      Operation: new = old * old
//      Test: divisible by 13
//        If true: throw to monkey 1
//        If false: throw to monkey 3
//
//    Monkey 3:
//      Starting items: 74
//      Operation: new = old + 3
//      Test: divisible by 17
//        If true: throw to monkey 0
//        If false: throw to monkey 1
//    """

    func makeMonkey(_ monkeyLines:ArraySlice<Line>) -> Monkey {
        guard monkeyLines.count == 6 else { return Monkey() }

        let startIndex = monkeyLines.startIndex
        let match = monkeyEntryRegex.matches(in: monkeyLines[startIndex].raw)
        let id = match[0].int(1)!

        let parts = monkeyLines[startIndex + 1].raw.split(on: ":")
        let nums = parts[1].split(on: ", ")
        let items = nums.compactMap { MonkeyItemType($0.trimming(while: \.isWhitespace)) }

        let operationParts = operationRegex.matches(in: monkeyLines[startIndex + 2].raw)
        let op = operationParts[0][1]
        let value = operationParts[0][2]

        let testParts = testRegex.matches(in: monkeyLines[startIndex + 3].raw)
        let divisor = MonkeyItemType(testParts[0][1]!)

        let testResultTrueParts = testResultRegex.matches(in: monkeyLines[startIndex + 4].raw)
        let nextMonkeyTrue = Int(testResultTrueParts[0][2]!)

        let testResultFalseParts = testResultRegex.matches(in: monkeyLines[startIndex + 5].raw)
        let nextMonkeyFalse = Int(testResultFalseParts[0][2]!)

        return Monkey(id, items: items, operation: op!, value: value!, divisor: divisor!, testPositiveMonkey: nextMonkeyTrue!, testNegativeMonkey: nextMonkeyFalse!)
    }

    func prepareMonkeys() -> [Monkey] {
        let rawMonkeys = input().lines.split(on: { $0.raw.isEmpty })

        var monkeys: [Monkey] = []
        rawMonkeys.forEach { monkeyLines in
            monkeys.append(makeMonkey(monkeyLines))
        }

        return monkeys
    }

    func part1() async throws -> String {
        let rounds = 20

        let monkeys = prepareMonkeys()

        (0..<rounds).forEach { index in
            monkeys.forEach { monkey in
                monkey.items.forEach { item in
                    let worryLevel = monkey.newWorryLevel(item)
                    let boredomLevel = worryLevel / 3
                    let monkeyToReceiveItem = monkey.throwItemToWhichMonkey(boredomLevel)
                    monkeys[monkeyToReceiveItem].items.append(boredomLevel)
                }

                monkey.removeAllItems()
            }
        }

        let levelOfMonkeyBusiness = monkeys.max(count: 2) { monkey1, monkey2 in
            monkey1.inspectedItems < monkey2.inspectedItems
        }.reduce(1) { $0 * $1.inspectedItems }

        return String(levelOfMonkeyBusiness)
    }

    func part2() async throws -> String {
        let rounds = 10_000

        let monkeys = prepareMonkeys()
        let divisor = monkeys.product { monkey in
            monkey.divisor
        }

        (0..<rounds).forEach { index in
            monkeys.forEach { monkey in
                monkey.items.forEach { item in
                    let worryLevel = monkey.newWorryLevel(item) % divisor
                    let monkeyToReceiveItem = monkey.throwItemToWhichMonkey(worryLevel)
                    monkeys[monkeyToReceiveItem].items.append(worryLevel)
                }

                monkey.removeAllItems()
            }
        }

        let levelOfMonkeyBusiness = monkeys.max(count: 2) { monkey1, monkey2 in
            monkey1.inspectedItems < monkey2.inspectedItems
        }.reduce(1) { $0 * $1.inspectedItems }

        return String(levelOfMonkeyBusiness)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
