import Foundation

// https://adventofcode.com/2020/day/6

struct Day06: Day {
    static func test(input: String) {
        let group1 = Group()
        group1.addAnswer(answer: "abc")
        assert(group1.combinedAnswersOfAnyone() == 3)

        let group2 = Group()
        group2.addAnswer(answer: "a")
        group2.addAnswer(answer: "b")
        group2.addAnswer(answer: "c")
        assert(group2.combinedAnswersOfAnyone() == 3)

        let group3 = Group()
        group3.addAnswer(answer: "ab")
        group3.addAnswer(answer: "ac")
        assert(group3.combinedAnswersOfAnyone() == 3)

        let group4 = Group()
        group4.addAnswer(answer: "a")
        group4.addAnswer(answer: "a")
        group4.addAnswer(answer: "a")
        group4.addAnswer(answer: "a")
        assert(group4.combinedAnswersOfAnyone() == 1)

        let group5 = Group()
        group5.addAnswer(answer: "b")
        assert(group5.combinedAnswersOfAnyone() == 1)
    }

    static func run(input: String) {
        let answers = input.components(separatedBy: .controlCharacters)

        var groups = [Group]()
        var currentGroup = Group()
        for line in answers {
            if line == "" {
                groups.append(currentGroup)
                currentGroup = Group()
            } else {
                currentGroup.addAnswer(answer: line)
            }
        }

        let yesAnswers = groups.reduce(0, {
            return $0 + $1.combinedAnswersOfAnyone()
        })

        print("Sum of questions anyone answered yes \(yesAnswers)")
    }
}
