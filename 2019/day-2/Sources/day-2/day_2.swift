func fuelRequiredForModule(_ mass: Int) -> Int {
    return Int(Double(mass / 3).rounded(.down) - 2)
}

func fuelRequiredForModuleWithFuelAddition(_ mass: Int) -> [Int] {
    var total = [mass]

    var temp = fuelRequiredForModule(mass)
    while temp > 0 {
        total.append(temp)

        temp = fuelRequiredForModule(temp)
    }

    return total
}

func requiredFuelPerModuleWithFuelAddition(_ program: [Int]) -> [Int] {
    let requiredFuelPerModuleWithFuelAddition = program.map {
        (mass: Int) -> Int in

        fuelRequiredForModuleWithFuelAddition(fuelRequiredForModule(mass)).reduce(0, +)
    }

    return requiredFuelPerModuleWithFuelAddition
}

func runProgram(_ program: [Int]) -> [Int] {
    var ip = 0
    var running = true
    var programCopy = program

    while running {
        let opcode = program[ip]

        switch opcode {
        case 1: // addition
            programCopy[program[ip + 3]] = programCopy[program[ip + 1]] + programCopy[program[ip + 2]]
            ip += 4
        case 2: // multiplication
            programCopy[program[ip + 3]] = programCopy[program[ip + 1]] * programCopy[program[ip + 2]]
            ip += 4
        case 99: // halt
            running = false
        default:
            running = false
        }
    }

    return programCopy
}

func Part1(_ program: [Int]) -> Int {
    var programCopy = program
    programCopy[1] = 12
    programCopy[2] = 2
    let result = runProgram(programCopy)

    return result[0]
}

func Part2(_ program: [Int]) -> Int {
    var programCopy = program
    var noun = 0
    var verb = 0
    var found = false
    var result = 0

    while noun <= 99 && !found {
        while verb <= 99 {
            programCopy[1] = noun
            programCopy[2] = verb
            let resultProgram = runProgram(programCopy)

            if resultProgram[0] == 19690720 {
                found = true
                result = 100 * noun + verb
            }

            programCopy = program
            verb += 1
        }
        verb = 0
        noun += 1
    }

    return result
}