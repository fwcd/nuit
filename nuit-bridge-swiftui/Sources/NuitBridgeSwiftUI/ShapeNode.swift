import NuitBridgeSwiftUICore

indirect enum ShapeNode: Codable, Hashable {
    // MARK: Primitive
    case capsule
    case circle
    case ellipse
    case rectangle
    case roundedRectangle(cornerSize: Vec2<Double>)
    case sector(startAngle: Angle, endAngle: Angle, innerRadiusFraction: Double)

    // MARK: Styled
    case fill(wrapped: ShapeNode, style: Style)
    case stroke(wrapped: ShapeNode, style: Style)
}
