import CoreGraphics

extension CGPoint {
    init(_ vec2: Vec2<Double>) {
        self.init(
            x: CGFloat(vec2.x),
            y: CGFloat(vec2.y)
        )
    }
}

extension CGSize {
    init(_ vec2: Vec2<Double>) {
        self.init(
            width: CGFloat(vec2.x),
            height: CGFloat(vec2.y)
        )
    }
}
