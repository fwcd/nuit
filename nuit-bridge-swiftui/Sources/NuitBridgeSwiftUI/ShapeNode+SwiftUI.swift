import SwiftUI

extension AnyShape {
    init(_ shape: ShapeNode) {
        switch shape {
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
        }
    }
}
