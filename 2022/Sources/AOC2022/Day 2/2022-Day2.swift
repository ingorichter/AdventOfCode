//
//  2022-Day2.swift
//  Solutions for Day 2
//  https://adventofcode.com/2022/day/2
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

class Day2: Day {
    // https://en.wikipedia.org/wiki/Rock_paper_scissors
    // A=Rock = 1pt
    // B=Paper = 2pt
    // C=Scissors = 3pt
    // X=Rock
    // Y=Paper
    // Z=Scissors

    // Win = 6pt
    // Loss = 0pt
    // Draw = 3pt

    // "Opponent-Me"
    let possibleCombinationsPart1: [String:Int] = [
        "A X": 1 + 3, // 1pt for Rock and 3pt for Draw
        "A Y": 2 + 6, // 2pt for Paper and 6pt for Win
        "A Z": 3 + 0, // 3pt for Scissors and 0pt for Loss
        "B X": 1 + 0, // 1pt for Rock and 0pt for Loss
        "B Y": 2 + 3, // 2pt for Paper and 3pt for Draw
        "B Z": 3 + 6, // 3pt for Scissors and 6pt for Win
        "C X": 1 + 6, // 1pt for Rock and 6pt for Win
        "C Y": 2 + 0, // 2pt for Paper and 0pt for Loss
        "C Z": 3 + 3  // 3pt for Scissors and 3pt for Draw
    ]

    // X = I need to lose
    // Y = I need a draw
    // Z = I need to win
    let possibleCombinationsPart2: [String:Int] = [
        "A X": 3 + 0, // 3pt for Scissors and 0pt for Loss
        "A Y": 1 + 3, // 1pt for Rock and 3pt for Draw
        "A Z": 2 + 6, // 2pt for Paper and 6pt for Win
        "B X": 1 + 0, // 1pt for Rock and 0pt for Loss
        "B Y": 2 + 3, // 2pt for Paper and 3pt for Draw
        "B Z": 3 + 6, // 3pt for Scissors and 6pt for Win
        "C X": 2 + 0, // 2pt for Paper and 0pt for Loss
        "C Y": 3 + 3, // 3pt for Scissors and 3pt for Draw
        "C Z": 1 + 6  // 1pt for Rock and 6pt for Win
    ]

    func part1() async throws -> String {
        let total = input().lines.reduce(0) {
            $0 + possibleCombinationsPart1[$1.raw]!
        }
        return String(total)
    }

    func part2() async throws -> String {
        let total = input().lines.reduce(0) {
            $0 + possibleCombinationsPart2[$1.raw]!
        }
        return String(total)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
