//
//  2022-Day6.swift
//  Solutions for Day 6
//  https://adventofcode.com/2022/day/6
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

class Day6: Day {
//    static var rawInput: String? = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"

    func findChunk(_ dataStream: String, _ chunkSize: Int) -> Int {
        var index = 0

        while index <= dataStream.count - chunkSize {
            let startIndex = dataStream.index(dataStream.startIndex, offsetBy: index)
            let endIndex = dataStream.index(startIndex, offsetBy: chunkSize)
            let chunk = dataStream[startIndex..<endIndex]

            let chars = Set(chunk)
            if chars.count == chunkSize {
                index += chunkSize
                break
            }
            else {
                index += 1
            }
        }

        return index
    }

    func part1() async throws -> String {
        return String(findChunk(input().raw, 4))
    }

    func part2() async throws -> String {
        return String(findChunk(input().raw, 14))
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
