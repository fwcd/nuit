import SwiftUI

public extension SwiftUI.UnitPoint {
    init(_ unitPoint: UnitPoint) {
        self.init(x: unitPoint.value.x, y: unitPoint.value.y)
    }
}
