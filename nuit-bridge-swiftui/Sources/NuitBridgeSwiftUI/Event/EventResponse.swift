enum EventResponse: Codable, Hashable {
    case empty
    case node(node: Node)
}
