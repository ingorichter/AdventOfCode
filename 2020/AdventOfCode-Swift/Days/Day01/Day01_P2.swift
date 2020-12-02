
import Foundation

struct Day01_P2: Day {
    static func run(input: String) {
        let target = 2020
        var numbers: [Int] = input.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines).components(separatedBy: .controlCharacters).map { Int($0)! }

        numbers.sort()

        var firstIndex = 0
        var leftIndex = 0
        var rightIndex = 0
        var found = false

        for i in 0...numbers.count - 2 {
            leftIndex = i + 1
            rightIndex = numbers.count - 1

            while leftIndex < rightIndex {
                let sum = numbers[i] + numbers[leftIndex] + numbers[rightIndex]

                if sum == target {
                    firstIndex = i
                    found = true
                    break
                } else if sum < target {
                    leftIndex += 1
                } else {
                    rightIndex -= 1
                }
            }

            if found {
                break
            }
        }

        print("\(numbers[firstIndex]) + \(numbers[leftIndex]) + \(numbers[rightIndex]) = \(numbers[firstIndex] + numbers[leftIndex] + numbers[rightIndex])")
        print("Result = \(numbers[firstIndex] * numbers[leftIndex] * numbers[rightIndex])")
    }
}
