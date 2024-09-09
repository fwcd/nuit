public struct Geometry: Codable, Hashable {
    public let size: Vec2<Double>

    public init(size: Vec2<Double>) {
        self.size = size
    }
}
