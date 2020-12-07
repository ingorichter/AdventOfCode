
import Foundation

// https://adventofcode.com/2020/day/4

struct Day04: Day {
    static func run(input: String) {
        // read all passports
        let lines = input.components(separatedBy: .controlCharacters) //.filter { $0.count > 0 }

        var passports: [Passport] = []
        var currentPassport = Passport(byr: "", iyr: "", eyr: "", hgt: "", hcl: "", ecl: "", pid: "", cid: "")

        for line in lines {
            if line == "" {
                passports.append(currentPassport)
                currentPassport = Passport(byr: "", iyr: "", eyr: "", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
            } else {
                line.components(separatedBy: " ").forEach { comp in
                    let parts = comp.split(separator: ":")
                    if parts[0] == "byr" {
                        currentPassport.byr = String(parts[1])
                    }
                    if parts[0] == "iyr" {
                        currentPassport.iyr = String(parts[1])
                    }
                    if parts[0] == "eyr" {
                        currentPassport.eyr = String(parts[1])
                    }
                    if parts[0] == "hgt" {
                        currentPassport.hgt = String(parts[1])
                    }
                    if parts[0] == "hcl" {
                        currentPassport.hcl = String(parts[1])
                    }
                    if parts[0] == "ecl" {
                        currentPassport.ecl = String(parts[1])
                    }
                    if parts[0] == "pid" {
                        currentPassport.pid = String(parts[1])
                    }
                    if parts[0] == "cid" {
                        currentPassport.cid = String(parts[1])
                    }
                }
            }
        }

        print(passports.filter { $0.nonEmpty() }.count)
    }
}
