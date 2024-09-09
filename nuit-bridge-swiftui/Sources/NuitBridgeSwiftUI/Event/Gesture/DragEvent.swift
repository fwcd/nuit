import NuitBridgeSwiftUICore

struct DragEvent: Codable, Hashable {
    let kind: Kind
    let startLocation: Vec2<Double>
    let location: Vec2<Double>

    enum Kind: String, Codable, Hashable {
        case updated
        case ended
    }
}
