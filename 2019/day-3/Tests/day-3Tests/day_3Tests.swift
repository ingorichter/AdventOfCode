@testable import day_3
import XCTest

final class day_3Tests: XCTestCase {
    var testData = [Int]()
    var wire1 = [String]()
    var wire2 = [String]()

    override func setUp() {
        super.setUp()

        let path = URL(fileURLWithPath: "./sample-input.txt")
        let content = try! String(contentsOf: path)

        let testData = content.split(separator: "\n")
        wire1 = testData[0].components(separatedBy: ",")
        wire2 = testData[1].components(separatedBy: ",")
    }

    func testPart1WithDescriptionData() {
        let wire1 = ["R8","U5","L5","D3"]
        let wire2 = ["U7","R6","D4","L4"]
        XCTAssertEqual(Part1(wire1, wire2), 6)
    }

    func testMove() {
        let (result, _) = move(.right, 3, Point(x: 1, y: 1))
        let expected: Array<Point> = [Point(x: 2, y: 1), Point(x: 3, y: 1), Point(x: 4, y: 1)]
        XCTAssertEqual(result, expected)
    }

    func testPart1WithTestData() {
        XCTAssertEqual(Part1(wire1, wire2), 1225)
    }

    func testPart2WithExampleData() {
        let wire1 = ["R75","D30","R83","U83","L12","D49","R71","U7","L72","U62","R66","U55","R34","D71","R55","D58","R83"]
        let wire2 = ["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51","U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"]
        XCTAssertEqual(Part2(wire1, wire2), 2)
    }

    func testPart2WithDescriptionData() {
        let wire1 = ["R8","U5","L5","D3"]
        let wire2 = ["U7","R6","D4","L4"]
        // find the distance of the intersection with the least amount of steps
        XCTAssertEqual(Part2(wire1, wire2), 30)
    }

    func testPart2WithTestData() {
        XCTAssertEqual(Part2(wire1, wire2), 107036)
    }

    static var allTests = [
        ("testMove", testMove),
        ("testPart1WithDescriptionData", testPart1WithDescriptionData),
        ("testPart2WithDescriptionData", testPart2WithDescriptionData),
        ("testPart1WithTestData", testPart1WithTestData),
        ("testPart2WithTestData", testPart2WithTestData)
    ]
}