enum Modifier: Codable, Hashable {
    case padding(insets: Insets)
    case frame(frame: Frame)
}
