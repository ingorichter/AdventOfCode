//
//  2022-Day8.swift
//  Solutions for Day 8
//  https://adventofcode.com/2022/day/8
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

struct ForestGrid {
    public var numberOfLines: Int
    public var numberOfColumns: Int

    private let input: [Line]

    init(_ inputLines: [Line]) {
        input = inputLines
        numberOfLines = inputLines.count
        numberOfColumns = inputLines[0].raw.count
    }

    func treeVisibleFromTheLeft(_ col: Int, _ row: Int) -> Bool {
        let max = input[row].digits[0...col-1].max()
        return max! < input[row].digits[col]
    }

    func treeVisibleFromTheRight(_ col: Int, _ row: Int) -> Bool {
        let max = input[row].digits[col + 1...numberOfColumns - 1].max()
        return max! < input[row].digits[col]
    }

    func treeVisibleFromTheTop(_ col: Int, _ row: Int) -> Bool {
        let max = input[0...row - 1].map { line in
            line.digits[col]
        }.max()
        return max! < input[row].digits[col]
    }

    func treeVisibleFromTheBottom(_ col: Int, _ row: Int) -> Bool {
        let max = input[row + 1...numberOfLines - 1].map { line in
            line.digits[col]
        }.max()
        return max! < input[row].digits[col]
    }

    func isTreeVisibleFromAnyDirection(_ col: Int, _ row: Int) -> Bool {
        treeVisibleFromTheLeft(col, row) || treeVisibleFromTheRight(col, row) || treeVisibleFromTheTop(col, row) || treeVisibleFromTheBottom(col, row)
    }

    func scenicScore(_ col: Int, _ row: Int) -> Int {
        var topScore = 0
        var bottomScore = 0
        var leftScore = 0
        var rightScore = 0

        let currentHeight = input[row].digits[col]

        // left score
        for currentColumnIndex in stride(from: col - 1, through: 0, by: -1) {
            leftScore += 1

            if input[row].digits[currentColumnIndex] >= currentHeight {
                break
            }
        }

        // right score
        for currentColumnIndex in col + 1..<numberOfColumns {
            rightScore += 1

            if input[row].digits[currentColumnIndex] >= currentHeight {
                break
            }
        }

        // top score
        for currentRowIndex in stride(from: row - 1, through: 0, by: -1) {
            topScore += 1

            if input[currentRowIndex].digits[col] >= currentHeight {
                break
            }
        }

        // bottom score
        for currentRowIndex in row + 1..<numberOfLines {
            bottomScore += 1

            if input[currentRowIndex].digits[col] >= currentHeight {
                break
            }
        }

        return topScore * bottomScore * leftScore * rightScore
    }
}

class Day8: Day {
//    static var rawInput: String? = """
//    30373
//    25512
//    65332
//    33549
//    35390
//    """

    func findVisibleTrees(_ forestGrid: ForestGrid) -> Int {
        var visibleTrees = 0

        for row in 1..<forestGrid.numberOfLines - 1 {
            for col in 1..<forestGrid.numberOfColumns - 1 {
//                print("Examine tree in col: \(col) and row: \(row)")
                if forestGrid.isTreeVisibleFromAnyDirection(col, row) {
                    visibleTrees += 1
                }
            }
        }

        return visibleTrees
    }

    func calcScenicScore(_ forestGrid: ForestGrid) -> Int {
        var scenicScoreForEachTree:[Int] = []

        for row in 1..<forestGrid.numberOfLines - 1 {
            for col in 1..<forestGrid.numberOfColumns - 1 {
                print("Examine tree in col: \(col) and row: \(row)")
                scenicScoreForEachTree.append(forestGrid.scenicScore(col, row))
            }
        }

        return scenicScoreForEachTree.max()!
    }

    func part1() async throws -> String {
        let grid = ForestGrid(input().lines)

        var treesVisibleFromOutside = 4 * (grid.numberOfColumns - 1)
        treesVisibleFromOutside += findVisibleTrees(grid)
        print(treesVisibleFromOutside)

        return String(treesVisibleFromOutside)
    }

    func part2() async throws -> String {
        let grid = ForestGrid(input().lines)

        let scenicScore = calcScenicScore(grid)
        print(scenicScore)

        return String(scenicScore)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
