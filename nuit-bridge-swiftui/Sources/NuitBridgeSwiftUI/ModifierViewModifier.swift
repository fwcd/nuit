import SwiftUI

struct ModifierViewModifier: ViewModifier {
    let modifier: Modifier

    func body(content: Content) -> some View {
        switch modifier {
        case let .padding(insets: insets):
            content.padding(EdgeInsets(insets))
        case let .position(position: position):
            content.position(CGPoint(position))
        case let .frame(frame: frame):
            switch frame {
            case let .constrained(minWidth: minWidth, idealWidth: idealWidth, maxWidth: maxWidth, minHeight: minHeight, idealHeight: idealHeight, maxHeight: maxHeight):
                content.frame(
                    minWidth: minWidth.map { CGFloat($0) },
                    idealWidth: idealWidth.map { CGFloat($0) },
                    maxWidth: maxWidth.map { CGFloat($0) },
                    minHeight: minHeight.map { CGFloat($0) },
                    idealHeight: idealHeight.map { CGFloat($0) },
                    maxHeight: maxHeight.map { CGFloat($0) }
                )
            case let .exact(width: width, height: height):
                content.frame(
                    width: width.map { CGFloat($0) },
                    height: height.map { CGFloat($0) }
                )
            }
        }
    }
}
