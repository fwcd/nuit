enum Event: Codable, Hashable {
    case click
    case gesture(gesture: GestureEvent)
    case updateText(content: String)
    case updatePickerSelection(id: Id)
}
