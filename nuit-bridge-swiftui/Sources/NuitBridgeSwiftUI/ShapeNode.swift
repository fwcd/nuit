enum ShapeNode: Codable, Hashable {
    case capsule
    case circle
    case ellipse
    case rectangle
    case roundedRectangle(cornerSize: Vec2<Double>)
}
