//
//  Day4.swift
//  test
//
//  Created by Ingo Richter on 11/28/21.
//  Copyright © 2021 Ingo Richter. All rights reserved.
//


// inspired by https://docs.swift.org/swift-book/LanguageGuide/Subscripts.html
// there is a Matrix struct, but I only wanted a subset of it
// needs to be a class and not struct, because modifying instances in a
// collection won't be permanently updated
class BingoBoard {
    var grid: [[Int]] = []

    var rowCount: Int { grid.count }
    var colCount: Int { grid[0].count }

    init(grid: [[Int]]) {
        self.grid = grid
    }

    subscript(_ row: Int, _ col: Int) -> Int {
        get {
            return grid[row][col]
        }

        set {
            grid[row][col] = newValue
        }
    }

    func copy() -> BingoBoard {
        return BingoBoard(grid: self.grid)
    }

    func replaceAllNumbers(_ number: Int, _ withNumber: Int) {
        for row in 0..<rowCount {
            for col in 0..<colCount {
                if self[row, col] == number {
                    self[row, col] = withNumber
                }
            }
        }
    }

    func findIndexOfNumber(_ number: Int) -> (Int, Int) {
        for row in 0..<rowCount {
            for col in 0..<colCount {
                if self[row, col] == number {
                    return (row, col)
                }
            }
        }

        return (-1, -1)
    }

    func rowBingo(_ board: BingoBoard) -> Bool {
        for row in 0..<board.rowCount {
            guard board.grid[row].allSatisfy({ $0 == -1 }) == false else {
                return true
            }
        }

        return false
    }

    func bingoOnBoard() -> Bool {
        var result = rowBingo(self)

        if !result {
            result = rowBingo(transpose())
        }

        return result
    }

    func transpose() -> BingoBoard {
        let gridCopy = self

        for row in 0..<gridCopy.rowCount {
            for col in row..<gridCopy.colCount {
                let previousValue = gridCopy[row, col]
                gridCopy[row, col] = gridCopy[col, row]
                gridCopy[col, row] = previousValue
            }
        }

        return gridCopy
    }
}

class Day4: Day {
    var numbers: [Int] {
        input.lines[0].integers
    }

    func bingoBoards() -> [BingoBoard] {
        let boardInputData = input.lines.dropFirst().chunks(of: 6)

        let boards = boardInputData.map { lines in
            return BingoBoard(grid: lines.dropFirst().map(\.integers))
        }

        return boards
    }

    override func part1() -> String {
        var result = 0

        let allBingoBoards = bingoBoards()

        for number in numbers {
            for board in allBingoBoards {
                let (row, col) = board.findIndexOfNumber(number)
                if row != -1 && col != -1 {
                    board[row, col] = -1
                }

                if board.bingoOnBoard() {
                    board.replaceAllNumbers(-1, 0)
                    let sumOfAllRemainingNumbers = board.grid.map { $0.reduce(0, +) }.sum
                    result = number * sumOfAllRemainingNumbers
                    break
                }
            }

            if result != 0 {
                break
            }
        }

        return "\(result)"
    }

    override func part2() -> String {
        var result = 0
        var longestRun = 0

        let allBingoBoards = bingoBoards()

        for board in allBingoBoards {
            var numberOfTurns = 0

            for number in numbers {
                if !board.bingoOnBoard() {
                    let (row, col) = board.findIndexOfNumber(number)
                    if row != -1 && col != -1 {
                        board[row, col] = -1
                    }

                    numberOfTurns += 1

                    if board.bingoOnBoard() && numberOfTurns > longestRun {
                        board.replaceAllNumbers(-1, 0)
                        let sumOfAllRemainingNumbers = board.grid.map { $0.reduce(0, +) }.sum

                        longestRun = numberOfTurns
                        result = number * sumOfAllRemainingNumbers
                        break
                    }
                }
            }
        }

        return "\(result)"
    }
}
