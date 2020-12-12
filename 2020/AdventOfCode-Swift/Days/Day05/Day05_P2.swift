import Foundation

// https://adventofcode.com/2020/day/5

struct Day05_P2: Day {
    static func test(input: String) {
        let boardingPass = BinaryBoardingPass(value: "FBFBBFFRLR")
        assert(boardingPass.row == 44)
        assert(boardingPass.seat == 5)
    }

    static func printSeatMap(seatMap: [[Bool]]) {
        for array in seatMap {
            for value in array {
                print(value, terminator: " ")
            }
            print(" ")
        }
    }

    static func run(input: String) {
        var seatMap: [[Bool]] = Array(repeating: Array(repeating: false, count: 8), count: 128)
        var seatIDSet = Set<Int>()

        let boardingPasses = input.components(separatedBy: .controlCharacters).filter { $0.count > 0 }

        boardingPasses.map { digitalBoardingPass in
            return BinaryBoardingPass(value: digitalBoardingPass)
        }.forEach { boardingPass in
            seatMap[boardingPass.row][boardingPass.seat] = true
            seatIDSet.insert(boardingPass.seatID)
        }

        for (rowIndex, row) in seatMap.enumerated() {
            for (seatIndex, seat) in row.enumerated() {
                if seat == false {
                    let seatID = rowIndex * 8 + seatIndex
                    if seatIDSet.contains(seatID + 1) && seatIDSet.contains(seatID - 1) {
                        print("Your seatID \(seatID)")
                    }
                }
            }
        }
    }
}
