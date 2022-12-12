//
//  2022-Day7.swift
//  Solutions for Day 7
//  https://adventofcode.com/2022/day/7
//
//  Created by Ingo Richter on 11/13/22.
//  Copyright © 2022 Ingo Richter. All rights reserved.
//

protocol FSEntry {
    var name: String { get set }
    var parent: Directory? { get set }
    var size: Int { get }
}

class Directory: FSEntry {
    public var name: String
    public var parent: Directory?
    public var children: [any FSEntry]

    init(name: String, parent: Directory? = nil, children: [any FSEntry]) {
        self.name = name
        self.parent = parent
        self.children = children
    }

    func childDirectories(_ includeSelf: Bool = false) -> [Directory] {
        var childDirs: [Directory] = []

        if includeSelf {
            childDirs.append(self)
        }

        self.children.forEach { child in
            if child is Directory {
                let tempDir = child as! Directory
                childDirs.append(tempDir)
                childDirs.append(contentsOf: tempDir.childDirectories())
            }
        }

        return childDirs
    }

    public lazy var size: Int = {
        return children.reduce(0) { acc, child in
            return acc + child.size
        }
    }()

    var debugDescription: String {
        return "Directory: \(name), Parent: \(String(describing: parent))"
    }
}

class File: FSEntry {
    public var name: String
    public var size: Int
    public var parent: Directory?

    init(name: String, size: Int, parent: Directory? = nil) {
        self.name = name
        self.size = size
        self.parent = parent
    }

    var debugDescription: String {
        return "File: \(name), Size: \(size), Parent: \(parent!.name)"
    }
}

class Day7: Day {
//    static var rawInput: String? = """
//    $ cd /
//    $ ls
//    dir a
//    14848514 b.txt
//    8504156 c.dat
//    dir d
//    $ cd a
//    $ ls
//    dir e
//    29116 f
//    2557 g
//    62596 h.lst
//    $ cd e
//    $ ls
//    584 i
//    $ cd ..
//    $ cd ..
//    $ cd d
//    $ ls
//    4060174 j
//    8033020 d.log
//    5626152 d.ext
//    7214296 k
//    """

    let dirTree: [String: Directory]
    var currentDir: Directory?
    var currentDirName: String?

    init() {
        self.dirTree = ["/": Directory(name: "/", children: [])]
        self.currentDir = dirTree["/"]
        self.currentDirName = nil
    }

    fileprivate func buildDirectoryTree() {
        input().lines.forEach { line in
            if line.raw.starts(with: "$") {
                let parts = line.words(separatedBy: CharacterSet.whitespaces)

                if parts[1].raw == "cd" {
                    let destination = parts[2].raw
                    if destination == ".." {
                        currentDir = currentDir?.parent
                        currentDirName = currentDir?.name
                    } else if destination == "/" {
                        currentDir = dirTree["/"]
                        currentDirName = "/"
                    }
                    else {
                        var tempDir = currentDir?.children.first(where: { $0.name == destination })
                        if tempDir == nil {
                            tempDir = Directory(name: destination, parent: currentDir, children: [])
                        }
                        currentDirName = destination
                        currentDir = tempDir as? Directory
                    }
                }
            }
            else {
                let parts = line.words(separatedBy: CharacterSet.whitespaces)
                if parts[0].raw == "dir" {
                    currentDirName = parts[1].raw
                    if !(currentDir?.children.contains(where: { $0.name == currentDirName! }))! {
                        let newDir = Directory(name: currentDirName!, parent: currentDir, children: [])
                        currentDir?.children.append(newDir)
                    }
                } else {
                    let name = parts[1].raw
                    if !(currentDir?.children.contains(where: { $0.name == name }))! {
                        let newFile = File(name: name, size: parts[0].integer!, parent: currentDir)
                        currentDir?.children.append(newFile)
                    }
                }
            }
        }
    }

    func part1() async throws -> String {
        buildDirectoryTree();

        let totalSize: Int = (dirTree["/"]?.childDirectories().filter { dir in
            dir.size <= 100000
        }
            .map { $0.size }
            .reduce(0, +))!

        return String(totalSize)
    }

    func part2() async throws -> String {
        buildDirectoryTree()

        // filesystem capacity
        let fsCapacity = 70_000_000
        let requiredUnusedSpace = 30_000_000

        let usedSpace = fsCapacity - dirTree["/"]!.size
        let stillNeeded = requiredUnusedSpace - usedSpace
        let directoryToDelete = dirTree["/"]?.childDirectories(true).filter { dir in
            dir.size >= stillNeeded
        }
        .min(by: { dirA, dirB in
            dirA.size < dirB.size
        })

        return String(directoryToDelete?.size ?? 0)
    }

    func run() async throws -> (String, String) {
        let p1 = try await part1()
        let p2 = try await part2()
        return (p1, p2)
    }
}
