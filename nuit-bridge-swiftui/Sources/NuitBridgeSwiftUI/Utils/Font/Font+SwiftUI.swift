import SwiftUI

extension SwiftUI.Font {
    init(_ font: Font) {
        switch font {
        case let .system(size: size, design: design, weight: weight):
            switch size {
            case let .level(level: level):
                self = .system(.init(level), design: design.map { .init($0) }, weight: weight.map { .init($0) })
            case let .custom(size: size):
                self = .system(size: CGFloat(size), weight: weight.map { .init($0) }, design: design.map { .init($0) })
            }
        case let .custom(name: name, size: size):
            self = .custom(name, size: CGFloat(size))
        }
    }
}
