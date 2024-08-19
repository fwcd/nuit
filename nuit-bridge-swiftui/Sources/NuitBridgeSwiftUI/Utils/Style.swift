enum Style: Codable, Hashable {
    case color(color: Color)
    case hierarchical(level: Int)
}
