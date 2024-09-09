import NuitBridgeSwiftUICore

enum Event: Codable, Hashable {
    case buttonTap
    case gesture(gesture: GestureEvent)
    case updateText(content: String)
    case updatePickerSelection(id: Id)
    case updateSliderValue(value: Double)
    case updateNavigationPath(path: [Value])
    case getNavigationDestination(value: Value)
}
