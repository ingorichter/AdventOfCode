@testable import day_2
import XCTest

final class day_2Tests: XCTestCase {
    var testData = [Int]()

    override func setUp() {
        super.setUp()

        let path = URL(fileURLWithPath: "./sample-input.txt")
        let content = try! String(contentsOf: path)

        testData = content.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines).split(separator: ",").map { Int($0)! }
    }

    func testPart1WithTestData() {
        XCTAssertEqual(Part1(testData), 3085697)
    }

    func testPart2WithTestData() {
        XCTAssertEqual(Part2(testData), 9425)
    }

    static var allTests = [
        ("testPart1WithTestData", testPart1WithTestData),
        ("testPart2WithTestData", testPart2WithTestData),
    ]
}