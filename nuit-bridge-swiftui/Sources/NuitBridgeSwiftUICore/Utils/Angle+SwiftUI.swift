import SwiftUI

public extension SwiftUI.Angle {
    init(_ angle: Angle) {
        self.init(radians: angle.radians)
    }
}
