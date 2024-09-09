import SwiftUI

public extension AnyShapeStyle {
    init(_ style: Style) {
        switch style {
        case let .color(color: color):
            self.init(SwiftUI.Color(color))
        case let .hierarchical(level: level):
            switch level {
            case 0: self.init(.primary)
            case 1: self.init(.secondary)
            case 2: self.init(.tertiary)
            case 3: self.init(.quaternary)
            default: self.init(.quinary)
            }
        case let .semantic(style: style):
            switch style {
            case .foreground: self.init(.foreground)
            case .background: self.init(.background)
            case .selection: self.init(.selection)
            case .separator: self.init(.separator)
            case .tint: self.init(.tint)
            case .placeholder: self.init(.placeholder)
            case .link: self.init(.link)
            case .fill: self.init(.fill)
            case .windowBackground: self.init(.windowBackground)
            }
        case let .blendMode(wrapped: wrapped, blendMode: blendMode):
            self.init(Self(wrapped).blendMode(.init(blendMode)))
        case let .opacity(wrapped: wrapped, opacity: opacity):
            self.init(Self(wrapped).opacity(CGFloat(opacity)))
        case let .shadow(wrapped: wrapped, shadow: shadow):
            self.init(Self(wrapped).shadow(.init(shadow)))
        }
    }
}
