indirect enum Node: Codable, Hashable {
    case empty
    case text(content: String)
    case textField(content: String)
    case button(label: Id<Node>)
    case vStack(wrapped: Id<Node>)
    case hStack(wrapped: Id<Node>)
    case zStack(wrapped: Id<Node>)
    case group(children: [Id<Node>])
}
