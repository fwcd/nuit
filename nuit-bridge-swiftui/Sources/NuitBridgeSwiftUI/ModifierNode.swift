enum ModifierNode: Codable, Hashable {
    case padding(insets: Insets)
    case position(position: Vec2<Double>)
    case offset(delta: Vec2<Double>)
    case opacity(opacity: Double)
    case frame(frame: Frame, alignment: Alignment)
    case fill(style: Style)
    case foregroundStyle(style: Style)
}
