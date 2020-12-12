
import Foundation

class BinaryBoardingPass {
    let value: String
    var row: Int
    var seat: Int

    init(value: String) {
        self.value = value
        self.row = 0
        self.seat = 0
        self.decodeRow()
        self.decodeSeat()
    }

    func decodeRow() {
        var lowerBounds = 0
        var upperBounds = 127

        let index = self.value.index(self.value.startIndex, offsetBy: 7)
        self.value[..<index].forEach{ c in
            let delta = (upperBounds - lowerBounds) / 2
            if c == "F" {
                upperBounds = upperBounds - delta - 1
            } else if c == "B" {
                lowerBounds = lowerBounds + delta + 1
            }
        }

        self.row = lowerBounds // upperBounds
    }

    func decodeSeat() {
        var lowerBounds = 0
        var upperBounds = 7

        let index = self.value.index(self.value.startIndex, offsetBy: 7)
        self.value[index..<self.value.endIndex].forEach{ c in
            let delta = (upperBounds - lowerBounds) / 2
            if c == "L" {
                upperBounds = upperBounds - delta - 1
            } else if c == "R" {
                lowerBounds = lowerBounds + delta + 1
            }
        }

        self.seat = lowerBounds
    }

    var seatID: Int {
        self.row * 8 + self.seat
    }

    var debugDescription: String {
        return "\(self) Row:\(row) and Seat:\(seat)"
    }
}

