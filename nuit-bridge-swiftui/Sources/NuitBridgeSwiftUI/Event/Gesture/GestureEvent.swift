enum GestureEvent: Codable, Hashable {
    case tap
    case drag(drag: DragEvent)
}
