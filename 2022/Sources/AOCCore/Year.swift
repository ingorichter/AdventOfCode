//
//  File.swift
//
//
//  Created by Dave DeLong on 11/14/19.
//

import Foundation
import ObjectiveC

public class Year {

    private let year: Int

    public init(_ year: Int) {
        self.year = year
    }

    public func day(_ day: Int) -> any Day {
        let dayClass = objc_getClass("AOC\(year).Day\(day)")
        return (dayClass as! NSObject.Type).init() as! (any Day)
    }

    public func allDays() -> [any Day] {
        (1...25).map {
            day($0)
        }
    }
}
