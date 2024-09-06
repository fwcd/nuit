import SwiftUI

extension SwiftUI.VerticalAlignment {
    init(_ alignment: VerticalAlignment) {
        switch alignment {
        case .top: self = .top
        case .center: self = .center
        case .bottom: self = .bottom
        case .firstTextBaseline: self = .firstTextBaseline
        case .lastTextBaseline: self = .lastTextBaseline
        }
    }
}
