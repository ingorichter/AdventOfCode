import Foundation

// https://adventofcode.com/2020/day/5

struct Day05: Day {
    static func test(input: String) {
        let boardingPass = BinaryBoardingPass(value: "FBFBBFFRLR")
        assert(boardingPass.row == 44)
        assert(boardingPass.seat == 5)
    }

    static func run(input: String) {
        let boardingPasses = input.components(separatedBy: .controlCharacters).filter { $0.count > 0 }

        var highestSeatId = 0

        boardingPasses.map { digitalBoardingPass in
            return BinaryBoardingPass(value: digitalBoardingPass)
        }.forEach { boardingPass in
            if boardingPass.seatID > highestSeatId {
                highestSeatId = boardingPass.seatID
            }
        }

        print("Highest SeatID: \(highestSeatId)")
    }
}
