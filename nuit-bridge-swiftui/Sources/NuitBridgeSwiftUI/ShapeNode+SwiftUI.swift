import SwiftUI

extension AnyShape {
    init(_ shape: ShapeNode) {
        switch shape {
        // MARK: Primitive
        case .capsule:
            self.init(Capsule())
        case .circle:
            self.init(Circle())
        case .ellipse:
            self.init(Ellipse())
        case .rectangle:
            self.init(Rectangle())
        case let .roundedRectangle(cornerSize: cornerSize):
            self.init(RoundedRectangle(cornerSize: CGSize(cornerSize)))

        // Filling is unfortunately only supported at the top-level since
        // .fill(...) returns a View rather than a Shape, so we'll have to
        // ignore any intermediate fills. Intermediate stroking is supported,
        // but not via Shape, but StrokeShapeView:
        // https://developer.apple.com/documentation/swiftui/strokeshapeview

        // MARK: Wrapper
        case .fill(wrapped: let wrapped, style: _):
            self.init(wrapped)
        case .stroke(wrapped: let wrapped, style: _):
            self.init(wrapped)
        }
    }
}
