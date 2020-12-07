//
//  Passport.swift
//  AdventOfCode-Swift
//
//  Created by Ingo Richter on 12/6/20.
//  Copyright © 2020 Thomas Durand. All rights reserved.
//

import Foundation

struct Passport {
    var byr: String // byr (Birth Year)
    var iyr: String // iyr (Issue Year)
    var eyr: String // eyr (Expiration Year)
    var hgt: String // hgt (Height)
    var hcl: String // hcl (Hair Color)
    var ecl: String // ecl (Eye Color)
    var pid: String // pid (Passport ID)
    var cid: String // cid (Country ID)

    func nonEmpty() -> Bool {
        return byr != "" && iyr != "" && eyr != "" && hgt != "" && hcl != "" && ecl != "" && pid != ""
    }

    func valid() -> Bool {
        return byrValid() && iyrValid() && eyrValid() && hgtValid() && hclValid() && eclValid() && pidValid()
    }

    func matchRegex(regexPattern: String, str: String) -> (Bool, [NSTextCheckingResult]) {
        var match = false
        var matches: [NSTextCheckingResult]
        do {
            let regEx = try NSRegularExpression(pattern: regexPattern, options: .anchorsMatchLines)
            matches = regEx.matches(in: str, options: [], range: NSMakeRange(0, str.count))
            match = matches.count > 0
        } catch {
            return (match, [])
        }

        return (match, matches)
    }

    func byrValid() -> Bool {
        let pattern = "\\d{4}"
        let (matched, _) = matchRegex(regexPattern: pattern, str: byr)

        if matched == false {
            return false
        }

        let number = Int(byr)!

        if number >= 1920 && number <= 2002 {
            return true
        } else {
            return false
        }
    }

    func iyrValid() -> Bool {
        let pattern = "\\d{4}"
        let (matched, _) = matchRegex(regexPattern: pattern, str: iyr)

        if matched == false {
            return false
        }

        let number = Int(iyr)!

        if number >= 2010 && number <= 2020 {
            return true
        } else {
            return false
        }
    }

    func eyrValid() -> Bool {
        let pattern = "\\d{4}"
        let (matched, _) = matchRegex(regexPattern: pattern, str: eyr)

        if matched == false {
            return false
        }

        let number = Int(eyr)!

        if number >= 2020 && number <= 2030 {
            return true
        } else {
            return false
        }
    }

    func hgtValid() -> Bool {
        // ?<unit>
        // ?<value>
//        let pattern = #"(\d{3})(cm)|(\d{2})(in)"#
        let pattern = #"(?<value>\d{3}|\d{2})(cm|in)"#
        let (matched, matches) = matchRegex(regexPattern: pattern, str: hgt)

        if matched == false {
            return false
        }

        if let match = matches.first {
            let valueMatch = match.range(at: 1)
            if valueMatch.location != NSNotFound {
                guard let valueRange = Range(valueMatch, in: hgt) else { return false }
                let numericalValue = Int(hgt[valueRange])!

                let unitMatch = match.range(at: 2)
                if unitMatch.location != NSNotFound {
                    guard let unitRange = Range(unitMatch, in: hgt) else { return false }

                    let unit = String(hgt[unitRange])

                    if unit == "in" {
                        return numericalValue >= 59 && numericalValue <= 76
                    } else if unit == "cm" {
                        return numericalValue >= 150 && numericalValue <= 193
                    }
                }
            }
        }

        return false
    }

    func hclValid() -> Bool {
        let pattern = "^#[0-9a-f]+$"
        let (match, _) = matchRegex(regexPattern: pattern, str: hcl)

        return match
    }

    func eclValid() -> Bool {
        let pattern = "amb|blu|brn|gry|grn|hzl|oth"
        let (match, _) = matchRegex(regexPattern: pattern, str: ecl)

        return match
    }

    func pidValid() -> Bool {
        let pattern = #"^\d{9}?$"#
        let (match, _) = matchRegex(regexPattern: pattern, str: pid)

        return match
    }
}
