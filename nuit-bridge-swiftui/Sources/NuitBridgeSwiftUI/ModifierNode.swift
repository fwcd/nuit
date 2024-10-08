import NuitBridgeSwiftUICore

enum ModifierNode: Codable, Hashable {
    case padding(insets: Insets)
    case position(position: Vec2<Double>)
    case offset(delta: Vec2<Double>)
    case opacity(opacity: Double)
    case frame(frame: Frame, alignment: Alignment)
    case fill(style: Style)
    case font(font: Font)
    case foregroundStyle(style: Style)
    case background(style: Style, safeAreaIgnoringEdges: EdgeSet)
    case scaleEffect(factor: Double, anchor: UnitPoint)
    case rotationEffect(angle: Angle, anchor: UnitPoint)
    case help(text: String)
    case navigationTitle(title: String)
    case navigationSubtitle(subtitle: String)
    case navigationTitleDisplayMode(displayMode: NavigationTitleDisplayMode)
}
