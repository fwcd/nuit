import SwiftUI

public extension SwiftUI.HorizontalAlignment {
    init(_ alignment: HorizontalAlignment) {
        switch alignment {
        case .leading: self = .leading
        case .center: self = .center
        case .trailing: self = .trailing
        }
    }
}
