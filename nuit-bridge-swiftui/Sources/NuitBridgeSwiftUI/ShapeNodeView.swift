import SwiftUI

struct ShapeNodeView: View {
    let shape: ShapeNode

    var body: some View {
        switch shape {
        // TODO: Support multi-level/nested stroking properly
        case let .fill(wrapped: .stroke(wrapped: wrapped, style: strokeStyle), style: fillStyle):
            AnyShape(wrapped)
                .stroke(AnyShapeStyle(strokeStyle))
                .fill(AnyShapeStyle(fillStyle))
        case let .fill(wrapped: wrapped, style: style):
            AnyShape(wrapped)
                .fill(AnyShapeStyle(style))
        case let .stroke(wrapped: wrapped, style: style):
            AnyShape(wrapped)
                .stroke(AnyShapeStyle(style))
        default:
            AnyShape(shape)
        }
    }
}
