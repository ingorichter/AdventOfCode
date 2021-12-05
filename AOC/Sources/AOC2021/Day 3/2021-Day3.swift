//
//  Day3.swift
//  test
//
//  Created by Ingo Richter on 11/28/21.
//  Copyright © 2021 Ingo Richter. All rights reserved.
//

import Darwin

class Day3: Day {
    func toInt(_ binaryCharSet: [Character]) -> Int {
        var res = 0

        let count = binaryCharSet.count
        for index in 0..<count {
            if binaryCharSet[index] == "1" {
                res = (res | (1 << (count - 1 - index)))
            }
        }

        return res
    }

    func invertBinaryValue(_ binaryCharSet: [Character]) -> [Character] {
        return binaryCharSet.map {
            $0 == "1" ? "0" : "1"
        }
    }

    func gammaValue(_ allLogEntries: Array<Array<Character>>, _ numberOfBits: Int) -> [Character] {
        var ones:Int = 0
        var zeroes:Int = 0
        var result = [Character]()

        (0..<numberOfBits).forEach { pos in
            allLogEntries.forEach { line in
                switch line[pos] {
                case "0": zeroes += 1
                case "1": ones += 1
                default:
                    fatalError()
                }
            }

            result.append(zeroes > ones ? "0" : "1")

            zeroes = 0
            ones = 0
        }

        return result
    }

    func oxygenGeneratorRating(_ allLogEntries: Array<Array<Character>>, _ numberOfBits: Int) -> [Character] {
        var result = [Character]()
        var allLogEntriesCopy = allLogEntries
        var ones = Array<Array<Character>>()
        var zeroes = Array<Array<Character>>()

        for pos in 0..<numberOfBits {
            allLogEntriesCopy.forEach { line in
                if line[pos] == "1" {
                    ones.append(line)
                } else {
                    zeroes.append(line)
                }
            }

            if ones.count >= zeroes.count {
                allLogEntriesCopy = ones
            } else {
                allLogEntriesCopy = zeroes
            }

            ones.removeAll()
            zeroes.removeAll()
        }

        if allLogEntriesCopy.count == 1 {
            result = allLogEntriesCopy.first!
        }

        return result
    }

    func co2ScrubberRating(_ allLogEntries: Array<Array<Character>>, _ numberOfBits: Int) -> [Character] {
        var result = [Character]()
        var allLogEntriesCopy = allLogEntries
        var ones = Array<Array<Character>>()
        var zeroes = Array<Array<Character>>()

        for pos in 0..<numberOfBits {
            allLogEntriesCopy.forEach { line in
                line[pos] == "0" ? zeroes.append(line) : ones.append(line)
            }

            allLogEntriesCopy = (zeroes.count > ones.count ? ones : zeroes)

            if zeroes.count == 1 || ones.count == 1 {
                allLogEntriesCopy = (zeroes.count == 1 ? zeroes : ones)
                break
            }

            ones.removeAll()
            zeroes.removeAll()
        }

        if allLogEntriesCopy.count == 1 {
            result = allLogEntriesCopy.first!
        }

        return result
    }

    override func part1() -> String {
        let allLogEntries = input.lines.characters
        let numberOfBits = allLogEntries[0].count

        let gammaValue = gammaValue(allLogEntries, numberOfBits)
        let epsilonValue = invertBinaryValue(gammaValue)

        return "\(toInt(gammaValue) * toInt(epsilonValue))"
    }

    override func part2() -> String {
        let allLogEntries = input.lines.characters
        let numberOfBits = allLogEntries[0].count

        let oxygenGeneratorRating = oxygenGeneratorRating(allLogEntries, numberOfBits)
        let co2ScrubberRating = co2ScrubberRating(allLogEntries, numberOfBits)

        return "\(toInt(oxygenGeneratorRating) * toInt( co2ScrubberRating))"
    }
}
