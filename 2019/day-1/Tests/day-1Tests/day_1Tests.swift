@testable import day_1
import XCTest

final class day_1Tests: XCTestCase {
    var testData = [Int]()

    override func setUp() {
        super.setUp()

        let path = URL(fileURLWithPath: "./sample-input.txt")
        let content = try! String(contentsOf: path)

        testData = content.split(separator: "\n").map { Int($0)! }
    }

    func testPart1WithDataFromDescription() {
        XCTAssertEqual(Part1([12, 14, 1969, 100_756]), 34241)
    }

    func testPart1WithTestData() {
        XCTAssertEqual(Part1(testData), 3_422_661)
    }

    func testPart2WithDataFromDescription() {
        XCTAssertEqual(Part2([12, 14, 1969, 100_756]), 51316)
    }

    func testPart2WithTestData() {
        XCTAssertEqual(Part2(testData), 5_131_103)
    }

    static var allTests = [
        ("testPart1WithDataFromDescription", testPart1WithDataFromDescription),
        ("testPart1WithTestData", testPart1WithTestData),
        ("testPart2WithDataFromDescription", testPart2WithDataFromDescription),
        ("testPart2WithTestData", testPart2WithTestData),
    ]
}