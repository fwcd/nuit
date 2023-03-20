indirect enum Primitive: Codable, Hashable {
    case empty
    case text(content: String)
    case button(label: Id<Primitive>)
    case vStack(wrapped: Id<Primitive>)
    case hStack(wrapped: Id<Primitive>)
    case zStack(wrapped: Id<Primitive>)
    case tuple2(child1: Id<Primitive>, child2: Id<Primitive>)
    case tuple3(child1: Id<Primitive>, child2: Id<Primitive>, child3: Id<Primitive>)
    case tuple4(child1: Id<Primitive>, child2: Id<Primitive>, child3: Id<Primitive>, child4: Id<Primitive>)
    case tuple5(child1: Id<Primitive>, child2: Id<Primitive>, child3: Id<Primitive>, child4: Id<Primitive>, child5: Id<Primitive>)
}
