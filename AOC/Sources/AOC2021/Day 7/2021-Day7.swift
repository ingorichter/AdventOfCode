//
//  Day7.swift
//  test
//
//  Created by Ingo Richter on 11/28/21.
//  Copyright © 2021 Ingo Richter. All rights reserved.
//

class Day7: Day {
    var horizontalPositions: [Int] {
        return input.integers
    }

    func rangeFromSet(_ set: Set<Int>) -> ClosedRange<Int> {
        return set.min()!...set.max()!
    }

    func sumOfNaturalNumbers(upto topNaturalNumber: Int) -> Int {
        // I don't know if this has a better name in the Math world
        return (topNaturalNumber * (topNaturalNumber + 1)) / 2
    }

    override func part1() -> String {
        let uniquePositions = Set(horizontalPositions)
        var lowestFuelCost = Int.max

        for position in rangeFromSet(uniquePositions) {
            let fuelNeeded = horizontalPositions.reduce(0) { $0 + abs($1 - position) }
            if fuelNeeded < lowestFuelCost {
                lowestFuelCost = fuelNeeded
            }
        }

        return "\(lowestFuelCost)"
    }

    override func part2() -> String {
        let uniquePositions = Set(horizontalPositions)
        var lowestFuelCost = Int.max

        for position in rangeFromSet(uniquePositions) {
            let fuelCost = horizontalPositions.reduce(0) {
                let diff = abs($1 - position)
                let cost = sumOfNaturalNumbers(upto: diff)
                return $0 + cost
            }

            if fuelCost < lowestFuelCost {
                lowestFuelCost = fuelCost
            }
        }

        return "\(lowestFuelCost)"
    }
}
