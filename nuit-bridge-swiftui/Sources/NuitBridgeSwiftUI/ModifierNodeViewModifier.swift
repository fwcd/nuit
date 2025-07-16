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
        case let .navigationTitle(title: title):
            content.navigationTitle(title)
        case let .navigationSubtitle(subtitle: subtitle):
            #if os(macOS) || targetEnvironment(macCatalyst)
            content.navigationSubtitle(subtitle)
            #else
            content
            #endif
        case let .navigationTitleDisplayMode(displayMode: displayMode):
            #if !os(macOS)
            content.navigationBarTitleDisplayMode(.init(displayMode))
            #else
            content
            #endif
        case let .border(style: style, width: width):
            content.border(AnyShapeStyle(style), width: CGFloat(width))
        case let .shadow(color: color, radius: radius, offset: offset):
            // For shadow, we need a concrete color. If the style is not a color, use black as fallback
            if case let .color(color: shadowColor) = color {
                content.shadow(color: Color(shadowColor), radius: CGFloat(radius), x: CGFloat(offset.x), y: CGFloat(offset.y))
            } else {
                content.shadow(color: .black, radius: CGFloat(radius), x: CGFloat(offset.x), y: CGFloat(offset.y))
            }
        case let .blur(radius: radius):
            content.blur(radius: CGFloat(radius))
        case let .cornerRadius(radius: radius):
            content.cornerRadius(CGFloat(radius))
        case let .overlay(style: style, alignment: alignment):
            content.overlay(alignment: .init(alignment)) {
                Rectangle().fill(AnyShapeStyle(style))
            }
        case let .zIndex(zIndex: zIndex):
            content.zIndex(zIndex)
        case let .hidden(isHidden: isHidden):
            if isHidden {
                content.hidden()
            } else {
                content
            }
        case let .disabled(isDisabled: isDisabled):
            content.disabled(isDisabled)
        case let .grayscale(intensity: intensity):
            content.grayscale(intensity)
        case let .brightness(amount: amount):
            content.brightness(amount)
        case let .contrast(amount: amount):
            content.contrast(amount)
        case let .saturation(amount: amount):
            content.saturation(amount)
        case let .hueRotation(angle: angle):
            content.hueRotation(.init(angle))
        case .clipped:
            content.clipped()
        }
    }
}

private func open<T: View>(_ view: T) -> some View {
    view
}
