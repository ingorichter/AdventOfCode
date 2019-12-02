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

func requiredFuelPerModuleWithFuelAddition(_ inputData: [Int]) -> [Int] {
    let requiredFuelPerModuleWithFuelAddition = inputData.map {
        (mass: Int) -> Int in

        fuelRequiredForModuleWithFuelAddition(fuelRequiredForModule(mass)).reduce(0, +)
    }

    return requiredFuelPerModuleWithFuelAddition
}

func Part1(_ inputData: [Int]) -> Int {
    return inputData.map { mass in
        fuelRequiredForModule(mass)
    }.reduce(0, +)
}

func Part2(_ inputData: [Int]) -> Int {
    return requiredFuelPerModuleWithFuelAddition(inputData).reduce(0, +)
}