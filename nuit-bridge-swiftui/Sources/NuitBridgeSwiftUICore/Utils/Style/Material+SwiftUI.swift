import SwiftUI

public extension SwiftUI.Material {
    init(_ material: Material) {
        switch material {
        case .ultraThin: self = .ultraThin
        case .thin: self = .thin
        case .regular: self = .regular
        case .thick: self = .thick
        case .ultraThick: self = .ultraThick
        case .bar: self = .bar
        }
    }
}
