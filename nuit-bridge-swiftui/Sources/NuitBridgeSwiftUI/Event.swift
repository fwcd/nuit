enum Event: Codable, Hashable {
    case click
    case updateText(content: String)
    case updatePickerSelection(id: Id)
}
