indirect enum Node: Codable, Hashable {
    // MARK: Primitive
    case empty
    case text(content: String)
    case textField(content: String)
    case button(label: Id<Node>)

    // MARK: Aggregation
    case group(children: [Id<Node>])

    // MARK: Layout
    case vStack(wrapped: Id<Node>)
    case hStack(wrapped: Id<Node>)
    case zStack(wrapped: Id<Node>)

    // MARK: Modifier
    case padding(wrapped: Id<Node>, insets: Insets)
}
