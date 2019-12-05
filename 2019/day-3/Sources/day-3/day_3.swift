enum Direction {
    case right
    case left
    case up
    case down
}

struct Point {
    var x = 0
    var y = 0
}

extension Point: Hashable {
    static func == (lhs: Point, rhs: Point) -> Bool {
        return lhs.x == rhs.x && lhs.y == rhs.y
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(x)
        hasher.combine(y)
    }
}

extension Point: CustomDebugStringConvertible {
    var debugDescription: String {
        return "Point(x: \(x), y: \(y))"
    }
}

extension Point {
    func manhattanDistance(_ anotherPoint: Point) -> Int {
        return abs(self.x - anotherPoint.x) + abs(self.y - anotherPoint.y) 
    }
}

func move(_ direction: Direction, _ steps: Int, _ startPos: Point) -> (Array<Point>, Point) {
    var wirePoints = Array<Point>()
    var point = startPos

    switch direction {
    case .left:
        for _ in (0..<steps) {
            point = Point(x: point.x - 1, y: point.y)
            wirePoints.append(point)
        }
    case .right:
        for _ in (0..<steps) {
            point = Point(x: point.x + 1, y: point.y)
            wirePoints.append(point)
        }
    case .up:
        for _ in (0..<steps) {
            point = Point(x: point.x, y: point.y + 1)
            wirePoints.append(point)
        }
    case .down:
        for _ in (0..<steps) {
            point = Point(x: point.x, y: point.y - 1)
            wirePoints.append(point)
        }
    }

    return (wirePoints, point)
}

func drawWire(_ wire: [String], _ startPoint: Point) -> Array<Point> {
    var wirePoints = Array<Point>()
    var startPos = startPoint

    for instruction in wire {
        let direction = instruction[instruction.startIndex]
        let steps = Int(instruction[instruction.index(after: instruction.startIndex)...])!

        var newWirePoints = Array<Point>()

        switch direction {
        case "R": // move right
            (newWirePoints, startPos) = move(.right, steps, startPos)
        case "L": // move left
            (newWirePoints, startPos) = move(.left, steps, startPos)
        case "U": // move up
            (newWirePoints, startPos) = move(.up, steps, startPos)
        case "D": // move down
            (newWirePoints, startPos) = move(.down, steps, startPos)
        default:
            print("Unknown Direction")
        }

        wirePoints += newWirePoints
    }

    return wirePoints
}

func ArrayToSet<T>(_ array: [T]) -> Set<T> {
    var newSet = Set<T>()

    array.forEach { item in
        newSet.insert(item)
    }

    return newSet
}

func Part1(_ wire1: [String], _ wire2: [String]) -> Int {
    let startPoint = Point(x: 1, y: 1)
    let wire1PointsArray = drawWire(wire1, startPoint)
    let wire2PointsArray = drawWire(wire2, startPoint)

    let wire1Points = ArrayToSet(wire1PointsArray)
    let wire2Points = ArrayToSet(wire2PointsArray)

    // print("Wire 1: \(wire1Points)")
    let intersections = wire1Points.intersection(wire2Points)

    var smallestDistance = intersections.map { $0.manhattanDistance(startPoint) }
    smallestDistance.sort()

    // print("Smallest Distance: \(smallestDistance)")
    // print("Intersection of wire1 and wire2: \(intersections)")

    return smallestDistance[0]
}

func Part2(_ wire1: [String], _ wire2: [String]) -> Int {
    let startPoint = Point(x: 1, y: 1)
    let wire1PointsArray = drawWire(wire1, startPoint)
    let wire2PointsArray = drawWire(wire2, startPoint)

    let wire1Points = ArrayToSet(wire1PointsArray)
    let wire2Points = ArrayToSet(wire2PointsArray)

    // print("Wire 1: \(wire1Points)")
    // print("Wire 2: \(wire2Points)")
    let intersections = wire1Points.intersection(wire2Points)

    // print(intersections)
    // print(wire1PointsArray)

    var point2Distance = [Point : Int]()
    intersections.forEach { point in
        var stepsPoint1: Int? = wire1PointsArray.firstIndex{ $0 == point }
        var stepsPoint2: Int? = wire2PointsArray.firstIndex{ $0 == point }

        stepsPoint1! += 1
        stepsPoint2! += 1
        // print("Steps for Point1: \(stepsPoint1!), Steps for Point2: \(stepsPoint2!)")
        point2Distance[point] = stepsPoint1! + stepsPoint2!
    }

    // print("Point2Distance: \(point2Distance)")
    let closestPoint = point2Distance.min{a, b in a.value < b.value}
    let distance = closestPoint!.value
    // print("Distance \(distance)")

    return distance
}
