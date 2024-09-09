public struct Shadow: Codable, Hashable {
    public let kind: Kind
    public let color: Color
    public let radius: Double
    public let offset: Vec2<Double>

    public enum Kind: String, Codable, Hashable {
        case drop
        case inner
    }
    
    public init(kind: Kind, color: Color, radius: Double, offset: Vec2<Double>) {
        self.kind = kind
        self.color = color
        self.radius = radius
        self.offset = offset
    }
}
