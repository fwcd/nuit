import SwiftUI

public extension SwiftUI.BlendMode {
    init(_ blendMode: BlendMode) {
        switch blendMode {
        case .normal: self = .normal
        case .darken: self = .darken
        case .multiply: self = .multiply
        case .colorBurn: self = .colorBurn
        case .plusDarker: self = .plusDarker
        case .lighten: self = .lighten
        case .screen: self = .screen
        case .colorDodge: self = .colorDodge
        case .plusLighter: self = .plusLighter
        case .overlay: self = .overlay
        case .softLight: self = .softLight
        case .hardLight: self = .hardLight
        case .difference: self = .difference
        case .exclusion: self = .exclusion
        case .hue: self = .hue
        case .saturation: self = .saturation
        case .color: self = .color
        case .luminosity: self = .luminosity
        case .sourceAtop: self = .sourceAtop
        case .destinationOver: self = .destinationOver
        case .destinationOut: self = .destinationOut
        }
    }
}
