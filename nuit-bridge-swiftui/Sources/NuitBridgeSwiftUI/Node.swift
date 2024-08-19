indirect enum Node: Codable, Hashable {
    case empty

    // MARK: Shape
    case capsule
    case circle
    case ellipse
    case rectangle
    case roundedRectangle(cornerSize: Vec2<Double>)

    // MARK: Widget
    case text(content: String)
    case textField(content: String)
    case button(label: Identified<Node>)
    case picker(title: String, selection: Id, content: Identified<Node>)

    // MARK: Aggregation
    case child(wrapped: Identified<Node>)
    case group(children: [Identified<Node>])

    // MARK: Layout
    case vStack(wrapped: Identified<Node>)
    case hStack(wrapped: Identified<Node>)
    case zStack(wrapped: Identified<Node>)
    case list(wrapped: Identified<Node>)

    // MARK: Modifier
    case modified(wrapped: Identified<Node>, modifier: ModifierNode)
}
