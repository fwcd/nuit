import SwiftUI

extension SwiftUI.Font.Weight {
    init(_ weight: FontWeight) {
        switch weight {
        case .black: self = .black
        case .bold: self = .bold
        case .heavy: self = .heavy
        case .light: self = .light
        case .medium: self = .medium
        case .regular: self = .regular
        case .semibold: self = .semibold
        case .thin: self = .thin
        case .ultraLight: self = .ultraLight
        }
    }
}
