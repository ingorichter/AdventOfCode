import Foundation

func matchRepeatingNumbersPart1(_ numString: String) -> Bool {
    let regexOptions: NSRegularExpression.Options = []
    let regex = try! NSRegularExpression(pattern: "00+|11+|22+|33+|44+|55+|66+|77+|88+|99+", options: regexOptions)

    let matches = regex.matches(in: numString, range: NSRange(location: 0, length: numString.utf8.count))
    // print("Matches: \(matches.count)")
    return matches.count > 0
}

func matchRepeatingNumbersPart2(_ numString: String) -> Bool {
    let regexOptions: NSRegularExpression.Options = []
    let regex = try! NSRegularExpression(pattern: "00+|11+|22+|33+|44+|55+|66+|77+|88+|99+", options: regexOptions)

    let matches = regex.matches(in: numString, range: NSRange(location: 0, length: numString.utf8.count))
    // print("Matches: \(matches.count)")
    var valid = false
    matches.forEach { match in 
        // print("Range length: \(match.range.length)")
        if match.range.length == 2 {
            valid = true
        }
    }

    return valid
}

func matchCriteria(_ num: Int) -> Bool {
    var valid = true
    let digits = String(num)

    for i in 0..<digits.count - 1 {
        let index = digits.index(digits.startIndex, offsetBy: i)
        let value1 = digits[digits.index(index, offsetBy: 1)]
        let value2 = digits[index]

        if value1 < value2 {
            valid = false
            break
        }
    }

    return valid
}

func checkPart1(_ number: Int) -> Bool {
    if matchCriteria(number) {
        return matchRepeatingNumbersPart1(String(number))
    }

    return false
}

func checkPart2(_ number: Int) -> Bool {
    if matchCriteria(number) {
        return matchRepeatingNumbersPart2(String(number))
    }

    return false
}

func Part1(_ lowerBounds: Int, _ upperBounds: Int) -> Int {
    var validCombinations = 0

    (lowerBounds...upperBounds).forEach {
        if checkPart1($0) {
            validCombinations += 1
        }
    }

    return validCombinations
}

func Part2(_ lowerBounds: Int, _ upperBounds: Int) -> Int {
    var validCombinations = 0

    (lowerBounds...upperBounds).forEach {
        if checkPart2($0) {
            validCombinations += 1
        }
    }

    return validCombinations
}
