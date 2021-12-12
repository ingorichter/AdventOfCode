//
//  Day6.swift
//  test
//
//  Created by Ingo Richter on 11/28/21.
//  Copyright © 2021 Ingo Richter. All rights reserved.
//

import AOCCore

class Day6: Day {
    func populateAndCountFish(_ initialPopulationSet: CountedSet<Int>, _ days: Int) -> Int {
        var currentPopulationSet = initialPopulationSet

        for _ in 0..<days {
            var newPopulationSet = CountedSet<Int>()

            for (day, cnt) in currentPopulationSet {
                var daysLeft = day - 1
                if daysLeft < 0 {
                    daysLeft = 6
                    newPopulationSet[8, default: 0] += cnt
                }
                newPopulationSet[daysLeft, default: 0] += cnt
            }

            currentPopulationSet = newPopulationSet
        }

        return currentPopulationSet.values.sum
    }

    override func part1() -> String {
        let initialState = CountedSet(counting: input.integers)

        let result = populateAndCountFish(initialState, 80)
        return "\(result)"
    }

    override func part2() -> String {
        let initialState = CountedSet(counting: input.integers)

        let result = populateAndCountFish(initialState, 256)
        return "\(result)"
    }
}
