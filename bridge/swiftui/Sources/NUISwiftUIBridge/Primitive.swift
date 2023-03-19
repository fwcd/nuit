indirect enum Primitive: Codable, Hashable {
    case text(String)
    case vStack(Primitive)
    case hStack(Primitive)
    case zStack(Primitive)
    case group([Primitive])
}
