
import Foundation

// https://adventofcode.com/2020/day/1

struct Day01: Day {
    static func run(input: String) {
        var numbers: [Int] = input.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines).components(separatedBy: .controlCharacters).map { Int($0)! }

        numbers.sort()

        let target = 2020
        var leftIndex = 0
        var rightIndex = numbers.count - 1

        while leftIndex < rightIndex {
            let sum = numbers[leftIndex] + numbers[rightIndex]

            if sum == target {
                break
            } else if sum < target {
                leftIndex += 1
            } else {
                rightIndex -= 1
            }
        }

        print("\(numbers[leftIndex]) + \(numbers[rightIndex]) = \(numbers[leftIndex] + numbers[rightIndex])")
        print("Result = \(numbers[leftIndex] * numbers[rightIndex])")
    }
}
