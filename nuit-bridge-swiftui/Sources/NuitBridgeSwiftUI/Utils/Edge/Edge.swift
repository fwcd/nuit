enum Edge: UInt8, CaseIterable, Codable, Hashable {
    case top = 0
    case bottom = 1
    case leading = 2
    case trailing = 3

    static let count = allCases.count
}
