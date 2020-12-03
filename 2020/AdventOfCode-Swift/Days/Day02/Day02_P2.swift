
import Foundation

// https://adventofcode.com/2020/day/2

struct Day02_P2: Day {
    static func run(input: String) {
        let policyAndPasswords = input.components(separatedBy: .controlCharacters).filter { $0.count > 0 }

        var matchingPasswordCount = 0
        for policyAndPassword in policyAndPasswords {
            let parts = policyAndPassword.components(separatedBy: " ")
            let searchCharacter = parts[1]
            let characterToMatch = Character(searchCharacter.components(separatedBy: ":")[0])

            let positions = parts[0].components(separatedBy: "-")
            let position1 = Int(positions[0])!
            let position2 = Int(positions[1])!

            let occurences = parts[2].filter { characterToMatch == $0 }.count

            if (occurences > 0) {
                let firstPositionMatch = parts[2][parts[2].index(parts[2].startIndex, offsetBy: position1 - 1)] == characterToMatch
                let secondPositonMatch = parts[2][parts[2].index(parts[2].startIndex, offsetBy: position2 - 1)] == characterToMatch
                if (firstPositionMatch && !secondPositonMatch) || (!firstPositionMatch && secondPositonMatch) {
                    matchingPasswordCount += 1
                }
            }
        }

        print("\(matchingPasswordCount) matched their policy")
    }
}
