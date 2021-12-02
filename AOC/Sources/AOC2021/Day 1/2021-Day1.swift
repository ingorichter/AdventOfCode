//
//  Day1.swift
//  test
//
//  Created by Ingo Richter on 11/28/21.
//  Copyright © 2021 Ingo Richter. All rights reserved.
//

class Day1: Day {
    override func part1() -> String {
        let howManyTimesIncreased = input.lines.integers.adjacentPairs().count { $1 > $0 }

        return "\(howManyTimesIncreased)"
    }

    override func part2() -> String {
        let increasedWindowSums = input.lines.integers.windows(ofCount: 3).map(\.sum).adjacentPairs().count { $1 > $0 }

        return "\(increasedWindowSums)"
    }
}
