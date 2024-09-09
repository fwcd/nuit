public indirect enum Style: Codable, Hashable {
    // MARK: Primitive
    case color(color: Color)
    case hierarchical(level: Int)
    case semantic(style: SemanticStyle)

    // MARK: Modifiers
    case blendMode(wrapped: Style, blendMode: BlendMode)
    case opacity(wrapped: Style, opacity: Double)
    case shadow(wrapped: Style, shadow: Shadow)
}
