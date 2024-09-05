enum Event: Codable, Hashable {
    case buttonTap
    case gesture(gesture: GestureEvent)
    case updateText(content: String)
    case updatePickerSelection(id: Id)
}
