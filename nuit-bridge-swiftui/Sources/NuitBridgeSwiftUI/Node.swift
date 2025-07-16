import NuitBridgeSwiftUICore

indirect enum Node: Codable, Hashable {
    case empty

    // MARK: Widget
    case text(content: String)
    case textField(content: String)
    case button(label: Identified<Node>)
    case picker(title: String, selection: Id, content: Identified<Node>)
    case slider(value: Double, lowerBound: Double, upperBound: Double, step: Double?)
    case toggle(isOn: Bool)

    // MARK: Aggregation
    case child(wrapped: Identified<Node>)
    case group(children: [Identified<Node>])

    // MARK: Layout
    case geometryReader
    case vStack(alignment: HorizontalAlignment, spacing: Double, wrapped: Identified<Node>)
    case hStack(alignment: VerticalAlignment, spacing: Double, wrapped: Identified<Node>)
    case zStack(alignment: Alignment, spacing: Double, wrapped: Identified<Node>)
    case list(wrapped: Identified<Node>)
    case overlay(wrapped: Identified<Node>, alignment: Alignment, overlayed: Identified<Node>)

    // MARK: Navigation
    case navigationStack(path: [Value]?, wrapped: Identified<Node>)
    case navigationSplitView(sidebar: Identified<Node>, content: Identified<Node>, detail: Identified<Node>)
    case navigationLink(label: Identified<Node>, value: Value)
    case navigationDestination(wrapped: Identified<Node>)

    // MARK: Wrapper
    case shape(shape: ShapeNode)
    case gestured(wrapped: Identified<Node>, gesture: Identified<GestureNode>)
    case modified(wrapped: Identified<Node>, modifier: ModifierNode)

    var isEmpty: Bool {
        self == .empty
    }
}
