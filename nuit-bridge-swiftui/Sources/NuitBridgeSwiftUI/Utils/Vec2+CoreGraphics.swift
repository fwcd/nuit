import CoreGraphics

extension Vec2 where T == Double {
    init(_ cgPoint: CGPoint) {
        self.init(
            x: Double(cgPoint.x),
            y: Double(cgPoint.y)
        )
    }

    init(_ cgSize: CGSize) {
        self.init(
            x: Double(cgSize.width),
            y: Double(cgSize.height)
        )
    }
}

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
