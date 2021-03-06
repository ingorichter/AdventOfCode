
import Foundation

// https://adventofcode.com/2020/day/2

struct Day02: Day {
    static func run(input: String) {
        let policyAndPasswords = input.components(separatedBy: .controlCharacters).filter { $0.count > 0 }

        var matchingPasswordCount = 0
        for policyAndPassword in policyAndPasswords {
            let parts = policyAndPassword.components(separatedBy: " ")
            let searchCharacter = parts[1]
            let characterToMatch = Character(searchCharacter.components(separatedBy: ":")[0])

            let quantityRange = parts[0].components(separatedBy: "-")
            let lowerBounds = Int(quantityRange[0])!
            let upperBounds = Int(quantityRange[1])!

            let occurences = parts[2].filter { characterToMatch == $0 }.count

            if (occurences >= lowerBounds && occurences <= upperBounds) {
                matchingPasswordCount += 1
            }
        }

        print("\(matchingPasswordCount) matched their policy")
    }
}
