//
//  Regex.swift
//  Advent of Code
//
//  Created by Dave DeLong on 12/24/17.
//  Copyright © 2017 Dave DeLong. All rights reserved.
//

public struct Regex {

    public static let integers: Regex = #"(-?\d+)"#

    private let pattern: NSRegularExpression?

    public init(_ pattern: StaticString, options: NSRegularExpression.Options = []) {
        self.pattern = try! NSRegularExpression(pattern: "\(pattern)", options: options)
    }

    public init(pattern: String, options: NSRegularExpression.Options = []) {
        self.pattern = try? NSRegularExpression(pattern: pattern, options: options)
    }

    public func matches<S: StringProtocol>(_ string: S) -> Bool {
        guard let pattern = pattern else {
            return false
        }

        let str = String(string)
        let range = NSRange(location: 0, length: str.utf16.count)
        return pattern.numberOfMatches(in: str, options: [.withTransparentBounds], range: range) > 0
    }

    public func match<S: StringProtocol>(_ string: S) -> RegexMatch? {
        guard let pattern = pattern else {
            return nil
        }

        let str = String(string)
        let range = NSRange(location: 0, length: str.utf16.count)
        guard let match = pattern.firstMatch(in: str, options: [.withTransparentBounds], range: range) else {
            return nil
        }
        return RegexMatch(result: match, source: str as NSString)
    }

    public func matches<S: StringProtocol>(in string: S) -> [RegexMatch] {
        var matches = [RegexMatch]()

        let str = String(string)
        let range = NSRange(location: 0, length: str.utf16.count)
        pattern?.enumerateMatches(in: str, options: [], range: range) { (result, _, _) in
            if let result = result {
                let match = RegexMatch(result: result, source: str as NSString)
                matches.append(match)
            }
        }

        return matches
    }
}

extension Regex: ExpressibleByStringLiteral {
    public init(stringLiteral value: StaticString) {
        self.init(value)
    }

    public init(extendedGraphemeClusterLiteral value: StaticString) {
        self.init(value)
    }

    public init(unicodeScalarLiteral value: StaticString) {
        self.init(value)
    }
}

public struct RegexMatch {
    private let matches: [String?]

    public var numberOfCaptures: Int {
        matches.count
    }

    fileprivate init(result: NSTextCheckingResult, source: NSString) {
        var matches = [String?]()
        for i in 0..<result.numberOfRanges {
            let r = result.range(at: i)
            if r.location == NSNotFound {
                matches.append(nil)
            } else {
                matches.append(source.substring(with: r))
            }
        }

        self.matches = matches
    }

    public subscript(index: Int) -> String? {
        matches[index]
    }

    public subscript(int index: Int) -> Int? {
        guard let string = self[index] else {
            return nil
        }
        return Int(string)
    }

    public subscript(char index: Int) -> Character? {
        self[index]?.first
    }

    public subscript(array index: Int) -> [Character]? {
        guard let string = self[index] else {
            return nil
        }
        return Array(string)
    }

    public func int(_ index: Int) -> Int? {
        self[int: index]
    }

    public func char(_ index: Int) -> Character? {
        self[char: index]
    }

    public func array(_ index: Int) -> [Character]? {
        self[array: index]
    }
}

public func ~=(left: Regex, right: String) -> Bool {
    left.matches(right)
}

public func ~=(left: Regex, right: String) -> RegexMatch? {
    left.match(right)
}

public extension String {

    func match(_ pattern: String) -> RegexMatch {
        let regex = Regex(pattern: pattern)
        return regex.match(self)!
    }

    func match(_ regex: Regex) -> RegexMatch {
        regex.match(self)!
    }

}
