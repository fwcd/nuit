indirect enum Node: Codable, Hashable {
    // MARK: Primitive
    case empty
    case text(content: String)
    case textField(content: String)
    case button(label: Identified<Node>)

    // MARK: Aggregation
    case group(children: [Identified<Node>])

    // MARK: Layout
    case vStack(wrapped: Identified<Node>)
    case hStack(wrapped: Identified<Node>)
    case zStack(wrapped: Identified<Node>)
    case list(wrapped: Identified<Node>)

    // MARK: Modifier
    case modified(wrapped: Identified<Node>, modifier: Modifier)
}
