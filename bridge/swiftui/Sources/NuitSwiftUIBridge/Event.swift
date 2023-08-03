enum Event: Codable, Hashable {
    case click
    case updateText(content: String)
}
