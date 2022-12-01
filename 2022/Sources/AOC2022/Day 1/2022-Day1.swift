//
//  2022-Day1.swift
//  Solutions for Day 1
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

class Day1: Day {
    func part1() async throws -> String {
        let numOfElves = input().lines.split(on: { $0.raw.isEmpty }).map(\.integers.sum)
        let elveWithMostCalories = numOfElves.max()!
        return String(elveWithMostCalories)
    }

    func part2() async throws -> String {
        let numOfElves = input().lines.split(on: { $0.raw.isEmpty }).map(\.integers.sum)
        let elveWithMostCalories = numOfElves.max(count: 3).sum
        return String(elveWithMostCalories)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
