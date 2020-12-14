//
//  Group.swift
//  AdventOfCode-Swift
//
//  Created by Ingo Richter on 12/13/20.
//  Copyright © 2020 Thomas Durand. All rights reserved.
//

import Foundation

class Group: CustomStringConvertible {
    var answers: [String]

    init() {
        answers = []
    }

    func addAnswer(answer: String) -> Void {
        answers.append(answer)
    }

    func persons() -> Int {
        answers.count
    }

    func combinedAnswersOfEveryone() -> Int {
        var everyonesAnswers: Set<Character>? = nil

        answers.forEach { answer in
            var answersFromPersonSet = Set<Character>()
            answer.forEach { char in
                answersFromPersonSet.insert(char)
            }

            if everyonesAnswers == nil {
                everyonesAnswers = answersFromPersonSet
            } else {
                everyonesAnswers = everyonesAnswers?.intersection(answersFromPersonSet)
            }
        }

        return everyonesAnswers!.count
    }

    func combinedAnswersOfAnyone() -> Int {
        var combinedAnswers = Set<Character>()

        answers.forEach { answer in
            answer.forEach { char in
                combinedAnswers.insert(char)
            }
        }

        return combinedAnswers.count
    }

    var description: String {
        return "\(answers.count) Person(s) provided answers: \(answers)"
    }
}

