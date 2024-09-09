import SwiftUI

public extension SwiftUI.Animation {
    init(_ animation: Animation) {
        switch animation {
        case .default: self = .default
        case let .curve(curve: curve, durationSeconds: duration):
            switch curve {
            case .linear: self = duration.map { .linear(duration: $0) } ?? .linear
            case .easeIn: self = duration.map { .easeIn(duration: $0) } ?? .easeIn
            case .easeOut: self = duration.map { .easeOut(duration: $0) } ?? .easeOut
            case .easeInOut: self = duration.map { .easeInOut(duration: $0) } ?? .easeInOut
            }
        }
    }
}
