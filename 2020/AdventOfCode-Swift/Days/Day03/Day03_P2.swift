
import Foundation

// https://adventofcode.com/2020/day/3

struct Day03_P2: Day {
    static func run(input: String) {
        let squaresAndTrees = input.components(separatedBy: .controlCharacters).filter { $0.count > 0 }

        let lines = squaresAndTrees.count

        var trees = 0
        let lineLength = 31
        var allTrees = 1
        var xPos = 0
        let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        for step in steps {
            stride(from: step.1, to: lines, by: step.1).forEach { lineNumber in
                let line = squaresAndTrees[lineNumber]
                xPos += step.0
                let index = line.index(line.startIndex, offsetBy: xPos % lineLength)
                if line[index] == "#" {
                    trees += 1
                }
            }
            print("Hit \(trees)")
            allTrees *= trees

            trees = 0
            xPos = 0
        }

        print("Hit \(allTrees)")
    }
}
