//
//  main.swift
//  
//
//  Created by Ingo Richter on 11/28/21.
//  Borrowed from https://github.com/davedelong/AOC/blob/master/Sources/advent/main.swift
//

import AOC2021

let day = Day.day(for: Date())

let date1 = Date()
let (part1, part2) = day.run()
let date2 = Date()
print("Executing \(type(of: day))")
print("Part 1: \(part1)")
print("Part 2: \(part2)")
print("Time: \(date2.timeIntervalSince(date1))")
