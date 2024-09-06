enum FontSize: Codable, Hashable {
    case level(level: FontLevel)
    case custom(size: Double)
}
