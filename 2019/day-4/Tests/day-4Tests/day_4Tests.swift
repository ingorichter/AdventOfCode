@testable import day_4
import XCTest

final class day_4Tests: XCTestCase {
    let lowerBounds = 136760
    let upperBounds = 595730

    override func setUp() {
        super.setUp()
    }

    func testPart1WithTestData() {
        XCTAssertEqual(Part1(lowerBounds, upperBounds), 1873)
    }

    func testPart2WithTestData() {
        XCTAssertEqual(Part2(lowerBounds, upperBounds), 1264)
    }

    func testMatchCriteriaPart1() {
        var result = checkPart1(122345)
        XCTAssertTrue(result)

        result = checkPart1(111123)
        XCTAssertTrue(result)

        result = checkPart1(135679)
        XCTAssertFalse(result)

        result = checkPart1(111111)
        XCTAssertTrue(result)

        result = checkPart1(223450)
        XCTAssertFalse(result)

        result = checkPart1(123789)
        XCTAssertFalse(result)

        result = checkPart1(112233)
        XCTAssertTrue(result)

        result = checkPart1(123444)
        XCTAssertTrue(result)

        result = checkPart1(111122)
        XCTAssertTrue(result)
    }

    func testMatchCriteriaPart2() {
        var result = checkPart2(122345)
        XCTAssertTrue(result)

        result = checkPart2(111123)
        XCTAssertFalse(result)

        result = checkPart2(135679)
        XCTAssertFalse(result)

        result = checkPart2(111111)
        XCTAssertFalse(result)

        result = checkPart2(223450)
        XCTAssertFalse(result)

        result = checkPart2(123789)
        XCTAssertFalse(result)

        result = checkPart2(112233)
        XCTAssertTrue(result)

        result = checkPart2(123444)
        XCTAssertFalse(result)

        result = checkPart2(111122)
        XCTAssertTrue(result)
    }

    static var allTests = [
        ("testMatchCriteriaPart1", testMatchCriteriaPart1),
        ("testMatchCriteriaPart2", testMatchCriteriaPart2),
        ("testPart1WithTestData", testPart1WithTestData),
        ("testPart2WithTestData", testPart2WithTestData)
    ]
}