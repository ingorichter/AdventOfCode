//
//  Day.swift
//  test
//
//  Created by Dave DeLong on 12/22/17.
//  Copyright © 2017 Dave DeLong. All rights reserved.
//

import Foundation

fileprivate let yearRegex = Regex(#"/AOC(\d+)/"#)
fileprivate let dayRegex = Regex(#".+?Day (\d+).+?\.txt$"#)
fileprivate let classNameRegex = Regex(#"AOC(\d+).Day(\d+)"#)

public protocol Day {
    associatedtype Part1Result: CustomStringConvertible = String
    associatedtype Part2Result: CustomStringConvertible = String
    
    static var rawInput: String? { get }
    
    func part1() async throws -> Part1Result
    func part2() async throws -> Part2Result
    func run() async throws -> (Part1Result, Part2Result)
}

extension Day {
    public static var rawInput: String? { nil }
    
    public func input(_ file: StaticString = #file) -> Input {
        if let raw = Self.rawInput {
            return Input(raw)
        } else {
            return Input.makeInput(caller: file)
        }
    }
    
    public func part1() async throws -> Part1Result {
        fatalError("Implement \(#function)")
    }
    
    public func part2() async throws -> Part2Result {
        fatalError("Implement \(#function)")
    }
    
    public func run() async throws -> (Part1Result, Part2Result) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
