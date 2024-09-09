import SwiftUI

public extension ShadowStyle {
    init(_ shadow: Shadow) {
        switch shadow.kind {
        case .drop:
            self = .drop(
                color: .init(shadow.color),
                radius: CGFloat(shadow.radius),
                x: CGFloat(shadow.offset.x),
                y: CGFloat(shadow.offset.y)
            )
        case .inner:
            self = .inner(
                color: .init(shadow.color),
                radius: CGFloat(shadow.radius),
                x: CGFloat(shadow.offset.x),
                y: CGFloat(shadow.offset.y)
            )
        }
    }
}
