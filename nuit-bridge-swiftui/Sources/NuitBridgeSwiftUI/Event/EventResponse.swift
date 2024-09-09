import NuitBridgeSwiftUICore

enum EventResponse: Codable, Hashable {
    case empty
    case node(node: Identified<Node>)
}
