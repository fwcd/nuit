indirect enum Primitive: Codable, Hashable {
    case empty
    case text(content: String)
    case vStack(wrapped: Primitive)
    case hStack(wrapped: Primitive)
    case zStack(wrapped: Primitive)
    case tuple2(child1: Primitive, child2: Primitive)
    case tuple3(child1: Primitive, child2: Primitive, child3: Primitive)
    case tuple4(child1: Primitive, child2: Primitive, child3: Primitive, child4: Primitive)
    case tuple5(child1: Primitive, child2: Primitive, child3: Primitive, child4: Primitive, child5: Primitive)
}
