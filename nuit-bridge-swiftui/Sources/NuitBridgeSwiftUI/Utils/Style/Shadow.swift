struct Shadow: Codable, Hashable {
    let kind: Kind
    let color: Color
    let radius: Double
    let offset: Vec2<Double>

    enum Kind: String, Codable, Hashable {
        case drop
        case inner
    }
}
