enum Modifier: Codable, Hashable {
    case padding(insets: Insets)
    case position(position: Vec2<Double>)
    case frame(frame: Frame)
    case fill(style: Style)
}
