indirect enum Node: Codable, Hashable {
    case empty

    // MARK: Widget
    case text(content: String)
    case textField(content: String)
    case button(label: Identified<Node>)
    case picker(title: String, selection: Id, content: Identified<Node>)

    // MARK: Aggregation
    case child(wrapped: Identified<Node>)
    case group(children: [Identified<Node>])

    // MARK: Layout
    case vStack(spacing: Double, wrapped: Identified<Node>)
    case hStack(spacing: Double, wrapped: Identified<Node>)
    case zStack(wrapped: Identified<Node>)
    case list(wrapped: Identified<Node>)

    // MARK: Wrapper
    case shape(shape: ShapeNode)
    case modified(wrapped: Identified<Node>, modifier: ModifierNode)
}
