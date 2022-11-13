//
//  AOC2022.swift
//
//
//  Created by Ingo Richter on 11/13/22.
//

import Foundation
import ArgumentParser

@_exported import AOCCore

@main
struct AOC2022: ParsableCommand {
    @Argument(help: "Day to run")
    var day = 1

    mutating func run() throws {
        print("Run Day \(day)")
    }
}
