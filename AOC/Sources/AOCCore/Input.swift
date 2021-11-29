//
//  Input.swift
//  Advent of Code
//
//  Created by Dave DeLong on 12/1/18.
//  Copyright © 2018 Dave DeLong. All rights reserved.
//

import Foundation

extension CharacterSet {
    public static var comma = CharacterSet(charactersIn: ",")
}

public protocol StringInput {
    init(_ raw: String)
    var raw: String { get }
    var integer: Int? { get }
    var characters: [Character] { get }

    var trimmed: Self { get }
    var lines: [Line] { get }
    var words: [Word] { get }
    var csvWords: [Word] { get }
    func words(separatedBy: CharacterSet) -> [Word]
    func words(separatedBy: String) -> [Word]
}

public final class Input: StringInput {
    public convenience init(file: String) {
        self.init(try! String(contentsOfFile: file))
    }

    public init(_ raw: String) {
        self.raw = raw.trimmingCharacters(in: .newlines)
    }

    public let raw: String
    public lazy var integer: Int? = {
        Int(raw)
    }()
    public lazy var characters: [Character] = {
        Array(raw)
    }()

    public lazy var trimmed: Input = {
        Input(raw.trimmingCharacters(in: .whitespacesAndNewlines))
    }()
    public lazy var lines: Array<Line> = {
        raw.components(separatedBy: .newlines).map {
            Line($0)
        }
    }()
    public lazy var words: Array<Word> = {
        self.words(separatedBy: .whitespaces)
    }()
    public lazy var csvWords: Array<Word> = {
        self.words(separatedBy: .comma)
    }()
    public lazy var integers: Array<Int> = {
        let matches = Regex.integers.matches(in: raw)
        return matches.compactMap {
            $0.int(1)
        }
    }()

    public func words(separatedBy: CharacterSet) -> Array<Word> {
        raw.components(separatedBy: separatedBy).filter {
            $0.isNotEmpty
        }.map {
            Word($0)
        }
    }

    public func words(separatedBy: String) -> Array<Word> {
        raw.components(separatedBy: separatedBy).filter {
            $0.isNotEmpty
        }.map {
            Word($0)
        }
    }

    public lazy var rawLines: Array<String> = {
        lines.raw
    }()
    public lazy var rawWords: Array<String> = {
        words.raw
    }()
    public lazy var rawLineWords: Array<Array<String>> = {
        lines.words.raw
    }()
}

public final class Line: StringInput {
    public init(_ raw: String) {
        self.raw = raw
    }

    public let raw: String
    public lazy var integer: Int? = {
        Int(raw)
    }()
    public lazy var characters: Array<Character> = {
        Array(raw)
    }()

    public lazy var trimmed: Line = {
        Line(raw.trimmingCharacters(in: .whitespacesAndNewlines))
    }()
    public var lines: Array<Line> {
        [self]
    }
    public lazy var words: Array<Word> = {
        self.words(separatedBy: .whitespaces)
    }()
    public lazy var csvWords: Array<Word> = {
        self.words(separatedBy: .comma)
    }()

    public lazy var integers: Array<Int> = {
        let matches = Regex.integers.matches(in: raw)
        return matches.compactMap {
            $0.int(1)
        }
    }()

    public func words(separatedBy: CharacterSet) -> Array<Word> {
        raw.components(separatedBy: separatedBy).filter {
            $0.isNotEmpty
        }.map {
            Word($0)
        }
    }

    public func words(separatedBy: String) -> Array<Word> {
        raw.components(separatedBy: separatedBy).filter {
            $0.isNotEmpty
        }.map {
            Word($0)
        }
    }
}

public final class Word: StringInput {
    public init(_ raw: String) {
        self.raw = raw
    }

    public let raw: String
    public lazy var integer: Int? = {
        Int(raw)
    }()
    public lazy var characters: Array<Character> = {
        Array(raw)
    }()

    public lazy var trimmed: Word = {
        Word(raw.trimmingCharacters(in: .whitespacesAndNewlines))
    }()
    public lazy var lines: Array<Line> = {
        [Line(raw)]
    }()
    public var words: Array<Word> {
        [self]
    }
    public var csvWords: Array<Word> {
        [self]
    }

    public func words(separatedBy: CharacterSet) -> Array<Word> {
        [self]
    }

    public func words(separatedBy: String) -> Array<Word> {
        [self]
    }
}

extension Collection where Element: StringInput {
    public var raw: Array<String> {
        map {
            $0.raw
        }
    }
    public var integers: Array<Int> {
        map {
            $0.integer!
        }
    }
    public var characters: Array<Array<Character>> {
        map {
            $0.characters
        }
    }

    public var trimmed: Array<Element> {
        map {
            $0.trimmed
        }
    }
    public var lines: Array<Array<Line>> {
        map {
            $0.lines
        }
    }
    public var words: Array<Array<Word>> {
        map {
            $0.words
        }
    }
    public var csvWords: Array<Array<Word>> {
        map {
            $0.csvWords
        }
    }

    public func words(separatedBy: CharacterSet) -> Array<Array<Word>> {
        map {
            $0.words(separatedBy: separatedBy)
        }
    }

    public func words(separatedBy: String) -> Array<Array<Word>> {
        map {
            $0.words(separatedBy: separatedBy)
        }
    }
}

extension Collection where Element: Collection, Element.Element: StringInput {
    public var raw: Array<Array<String>> {
        map {
            $0.raw
        }
    }
    public var integers: Array<Array<Int>> {
        map {
            $0.integers
        }
    }
}

extension Collection where Element == Character {
    public var integers: Array<Int> {
        map {
            Int("\($0)")!
        }
    }
}
