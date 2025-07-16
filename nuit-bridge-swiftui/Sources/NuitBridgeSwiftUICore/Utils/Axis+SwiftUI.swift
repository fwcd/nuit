import SwiftUI

extension Axis {
    public var swiftUIAxes: SwiftUI.Axis.Set {
        switch self {
        case .horizontal:
            return .horizontal
        case .vertical:
            return .vertical
        case .both:
            return [.horizontal, .vertical]
        }
    }
}