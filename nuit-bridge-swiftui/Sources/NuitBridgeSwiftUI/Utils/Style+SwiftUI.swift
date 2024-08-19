import SwiftUI

extension AnyShapeStyle {
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
        }
    }
}
