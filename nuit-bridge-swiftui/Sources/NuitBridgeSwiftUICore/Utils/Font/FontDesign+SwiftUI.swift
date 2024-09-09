import SwiftUI

public extension SwiftUI.Font.Design {
    init(_ design: FontDesign) {
        switch design {
        case .default: self = .default
        case .monospaced: self = .monospaced
        case .rounded: self = .rounded
        case .serif: self = .serif
        }
    }
}
