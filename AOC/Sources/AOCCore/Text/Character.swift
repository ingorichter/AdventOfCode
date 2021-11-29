//
//  Character.swift
//  AOC
//
//  Created by Dave DeLong on 12/4/18.
//  Copyright © 2018 Dave DeLong. All rights reserved.
//

import Foundation

public extension Character {
    static let alphabet = Array("abcdefghijklmnopqrstuvwxyz")

    init?(ascii: Int) {
        guard let scalar = Unicode.Scalar(ascii) else {
            return nil
        }
        self.init(scalar)
    }

    var isWhitespaceOrNewline: Bool {
        isWhitespace || isNewline
    }

    var uppercased: Character {
        uppercased().first!
    }

    var lowercased: Character {
        lowercased().first!
    }

    var isUppercase: Bool {
        uppercased == self
    }
}
