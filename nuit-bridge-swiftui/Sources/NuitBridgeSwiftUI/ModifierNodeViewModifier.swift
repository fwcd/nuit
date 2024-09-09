import NuitBridgeSwiftUICore
import SwiftUI

struct ModifierNodeViewModifier: ViewModifier {
    let modifier: ModifierNode

    func body(content: Content) -> some View {
        switch modifier {
        case let .padding(insets: insets):
            content.padding(EdgeInsets(insets))
        case let .position(position: position):
            content.position(CGPoint(position))
        case let .offset(delta: delta):
            content.offset(x: delta.x, y: delta.y)
        case let .opacity(opacity: opacity):
            content.opacity(opacity)
        case let .frame(frame: frame, alignment: alignment):
            switch frame {
            case let .constrained(minWidth: minWidth, idealWidth: idealWidth, maxWidth: maxWidth, minHeight: minHeight, idealHeight: idealHeight, maxHeight: maxHeight):
                content.frame(
                    minWidth: minWidth.map { CGFloat($0) },
                    idealWidth: idealWidth.map { CGFloat($0) },
                    maxWidth: maxWidth.map { CGFloat($0) },
                    minHeight: minHeight.map { CGFloat($0) },
                    idealHeight: idealHeight.map { CGFloat($0) },
                    maxHeight: maxHeight.map { CGFloat($0) },
                    alignment: .init(alignment)
                )
            case let .exact(width: width, height: height):
                content.frame(
                    width: width.map { CGFloat($0) },
                    height: height.map { CGFloat($0) },
                    alignment: .init(alignment)
                )
            }
        case let .fill(style: style):
            // FIXME: This doesn't work yet
            if let content = content as? any Shape {
                AnyShape(content).fill(AnyShapeStyle(style))
            } else {
                content
            }
        case let .font(font: font):
            content.font(.init(font))
        case let .foregroundStyle(style: style):
            content.foregroundStyle(AnyShapeStyle(style))
        case let .background(style: style, safeAreaIgnoringEdges: edges):
            content.background(AnyShapeStyle(style), ignoresSafeAreaEdges: .init(edges))
        case let .scaleEffect(factor: factor, anchor: anchor):
            content.scaleEffect(CGFloat(factor), anchor: .init(anchor))
        case let .rotationEffect(angle: angle, anchor: anchor):
            content.rotationEffect(.init(angle), anchor: .init(anchor))
        case let .help(text: text):
            content.help(text)
        }
    }
}

private func open<T: View>(_ view: T) -> some View {
    view
}
