import SwiftUI

struct ShapeNodeView: View {
    let shape: ShapeNode

    var body: some View {
        switch shape {
        case .capsule:
            Capsule()
        case .circle:
            Circle()
        case .ellipse:
            Ellipse()
        case .rectangle:
            Rectangle()
        case let .roundedRectangle(cornerSize: cornerSize):
            RoundedRectangle(cornerSize: CGSize(cornerSize))
        }
    }
}
