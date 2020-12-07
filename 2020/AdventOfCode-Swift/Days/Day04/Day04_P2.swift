
import Foundation


// https://adventofcode.com/2020/day/4

struct Day04_P2: Day {
    static func test(input: String) {
        let passportCorrect1 = Passport(byr: "2002", iyr: "", eyr: "", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect1.byrValid() == true)
        let passportCorrect2 = Passport(byr: "2002", iyr: "2010", eyr: "", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect2.iyrValid() == true)
        let passportCorrect3 = Passport(byr: "2002", iyr: "2020", eyr: "", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect3.iyrValid() == true)
        let passportWrong2 = Passport(byr: "2002", iyr: "2021", eyr: "", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportWrong2.iyrValid() == false)
        let passportCorrect4 = Passport(byr: "2002", iyr: "2020", eyr: "2020", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect4.eyrValid() == true)
        let passportCorrect5 = Passport(byr: "2002", iyr: "2020", eyr: "2030", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect5.eyrValid() == true)
        let passportCorrect6 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect6.eyrValid() == true)
        let passportWrong3 = Passport(byr: "2002", iyr: "2021", eyr: "2019", hgt: "", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportWrong3.eyrValid() == false)
        let passportCorrect7 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "150cm", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect7.hgtValid() == true)
        let passportCorrect8 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "193cm", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect8.hgtValid() == true)
        let passportCorrect9 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "59in", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect9.hgtValid() == true)
        let passportCorrect10 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportCorrect10.hgtValid() == true)
        let passportWrong4 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "78in", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportWrong4.hgtValid() == false)
        let passportWrong5 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "149cm", hcl: "", ecl: "", pid: "", cid: "")
        assert(passportWrong5.hgtValid() == false)
        let passportCorrect11 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "150cm", hcl: "#123abc", ecl: "", pid: "", cid: "")
        assert(passportCorrect11.hclValid() == true)
        let passportWrong6 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "193cm", hcl: "#123abz", ecl: "", pid: "", cid: "")
        assert(passportWrong6.hclValid() == false)
        let passportWrong7 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "59in", hcl: "123abc", ecl: "", pid: "", cid: "")
        assert(passportWrong7.hclValid() == false)
        let passportCorrect14 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "amb", pid: "", cid: "")
        assert(passportCorrect14.eclValid() == true)
        let passportCorrect15 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "blu", pid: "", cid: "")
        assert(passportCorrect15.eclValid() == true)
        let passportCorrect16 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "brn", pid: "", cid: "")
        assert(passportCorrect16.eclValid() == true)
        let passportCorrect17 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "gry", pid: "", cid: "")
        assert(passportCorrect17.eclValid() == true)
        let passportCorrect18 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "grn", pid: "", cid: "")
        assert(passportCorrect18.eclValid() == true)
        let passportCorrect19 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "hzl", pid: "", cid: "")
        assert(passportCorrect19.eclValid() == true)
        let passportCorrect20 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "oth", pid: "", cid: "")
        assert(passportCorrect20.eclValid() == true)
        let passportWrong8 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "59in", hcl: "123abc", ecl: "zatter", pid: "", cid: "")
        assert(passportWrong8.eclValid() == false)
        let passportCorrect21 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "76in", hcl: "", ecl: "oth", pid: "000000001", cid: "")
        assert(passportCorrect21.pidValid() == true)
        let passportWrong9 = Passport(byr: "2002", iyr: "2020", eyr: "2025", hgt: "59in", hcl: "123abc", ecl: "zatter", pid: "0123456789", cid: "")
        assert(passportWrong9.pidValid() == false)
    }

    static func run(input: String) {
        // read all passports
        let lines = input.components(separatedBy: .controlCharacters)

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

        print(passports.filter { $0.valid() }.count)
    }
}
