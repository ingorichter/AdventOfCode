//
//  2022-Day10.swift
//  Solutions for Day 10
//  https://adventofcode.com/2022/day/10
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

class Day10: Day {
//    static var rawInput: String? = """
//    noop
//    addx 3
//    addx -5
//    """

//    static var rawInput: String? = """
//    addx 15
//    addx -11
//    addx 6
//    addx -3
//    addx 5
//    addx -1
//    addx -8
//    addx 13
//    addx 4
//    noop
//    addx -1
//    addx 5
//    addx -1
//    addx 5
//    addx -1
//    addx 5
//    addx -1
//    addx 5
//    addx -1
//    addx -35
//    addx 1
//    addx 24
//    addx -19
//    addx 1
//    addx 16
//    addx -11
//    noop
//    noop
//    addx 21
//    addx -15
//    noop
//    noop
//    addx -3
//    addx 9
//    addx 1
//    addx -3
//    addx 8
//    addx 1
//    addx 5
//    noop
//    noop
//    noop
//    noop
//    noop
//    addx -36
//    noop
//    addx 1
//    addx 7
//    noop
//    noop
//    noop
//    addx 2
//    addx 6
//    noop
//    noop
//    noop
//    noop
//    noop
//    addx 1
//    noop
//    noop
//    addx 7
//    addx 1
//    noop
//    addx -13
//    addx 13
//    addx 7
//    noop
//    addx 1
//    addx -33
//    noop
//    noop
//    noop
//    addx 2
//    noop
//    noop
//    noop
//    addx 8
//    noop
//    addx -1
//    addx 2
//    addx 1
//    noop
//    addx 17
//    addx -9
//    addx 1
//    addx 1
//    addx -3
//    addx 11
//    noop
//    noop
//    addx 1
//    noop
//    addx 1
//    noop
//    noop
//    addx -13
//    addx -19
//    addx 1
//    addx 3
//    addx 26
//    addx -30
//    addx 12
//    addx -1
//    addx 3
//    addx 1
//    noop
//    noop
//    noop
//    addx -9
//    addx 18
//    addx 1
//    addx 2
//    noop
//    noop
//    addx 9
//    noop
//    noop
//    noop
//    addx -1
//    addx 2
//    addx -37
//    addx 1
//    addx 3
//    noop
//    addx 15
//    addx -21
//    addx 22
//    addx -6
//    addx 1
//    noop
//    addx 2
//    addx 1
//    noop
//    addx -10
//    noop
//    noop
//    addx 20
//    addx 1
//    addx 2
//    addx 2
//    addx -6
//    addx -11
//    noop
//    noop
//    noop
//    """

    var signalStrengths: [Int: Int] = [:]
    var cyclesOfInterest = [20, 60, 100, 140, 180, 220]
    var crt: [[String]] = Array(repeating: Array(repeating: "#", count: 40), count: 6)

    func updateSignalStrength(_ cycle: Int, _ registerValue: Int) {
        if cyclesOfInterest.contains(cycle) {
            signalStrengths[cycle] = cycle * registerValue
        }
    }

    func updateCrt(_ cycle: Int, _ registerValue: Int) {
        let (crtRow, crtCol) = cycle.quotientAndRemainder(dividingBy: 40)

        guard crtRow < 6 else { return }

        if crtCol == registerValue - 1 || crtCol == registerValue || crtCol == registerValue + 1 {
            crt[crtRow][crtCol] = "#"
        } else {
            crt[crtRow][crtCol] = "."
        }

//        print("Update Row \(crtRow) and Col \(crtCol) with \(crt[crtRow][crtCol])")
    }

    func printCrt() {
        crt.forEach { line in
            print(line.joined())
        }
    }

    func part1() async throws -> String {
        var signalStrengthSum = 0
        var xRegister: Int = 1
        var cycles = 0

        let instructions = input().lines

        instructions.forEach { line in
            let instruction = line.raw.split(on: " ")

            cycles += 1
            updateSignalStrength(cycles, xRegister)
//            print("The x register has a value of \(xRegister) after \(cycles) cycles")

            if instruction.count == 2 {
                cycles += 1
                updateSignalStrength(cycles, xRegister)
                xRegister += Int(instruction[1])!
//                print("The x register has a value of \(xRegister) after \(cycles) cycles")
            }
        }

//        print("The x register has a value of \(xRegister) after \(cycles) cycles")
        signalStrengthSum = signalStrengths.values.sum
//        print("Signal Strength: \(signalStrengthSum)")

        return String(signalStrengthSum)
    }

    func part2() async throws -> String {
        var xRegister: Int = 1
        var cycles = 0

        let instructions = input().lines

        instructions.forEach { line in
            let instruction = line.raw.split(on: " ")

            updateCrt(cycles, xRegister)
//            print("The x register has a value of \(xRegister) after \(cycles) cycles")

            if instruction.count == 2 {
                cycles += 1
                updateCrt(cycles, xRegister)
                cycles += 1
                xRegister += Int(instruction[1])!
//                print("The x register has a value of \(xRegister) after \(cycles) cycles")
            } else {
                cycles += 1
            }
        }

        printCrt()

        return "BGLRBZAU"
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
