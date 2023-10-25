indirect enum Node: Codable, Hashable {
    case empty
    case text(content: String)
    case textField(content: String)
    case button(label: Id<Node>)
    case vStack(wrapped: Id<Node>)
    case hStack(wrapped: Id<Node>)
    case zStack(wrapped: Id<Node>)
    case tuple2(child1: Id<Node>, child2: Id<Node>)
    case tuple3(child1: Id<Node>, child2: Id<Node>, child3: Id<Node>)
    case tuple4(child1: Id<Node>, child2: Id<Node>, child3: Id<Node>, child4: Id<Node>)
    case tuple5(child1: Id<Node>, child2: Id<Node>, child3: Id<Node>, child4: Id<Node>, child5: Id<Node>)
}
