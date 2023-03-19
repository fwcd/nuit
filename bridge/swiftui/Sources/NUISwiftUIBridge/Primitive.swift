indirect enum Primitive: Codable, Hashable {
    case text(content: String)
    case vStack(wrapped: Primitive)
    case hStack(wrapped: Primitive)
    case zStack(wrapped: Primitive)
    case group(children: [Primitive])
}
