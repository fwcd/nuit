enum GestureNode: Codable, Hashable {
    case tap(count: Int)
    case drag(minimumDistance: Double)
}
