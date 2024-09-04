import SwiftUI

extension SwiftUI.Alignment {
    init(_ alignment: Alignment) {
        switch (alignment) {
        case .topLeading: self = .topLeading
        case .top: self = .top
        case .topTrailing: self = .topTrailing
        case .leading: self = .leading
        case .center: self = .center
        case .trailing: self = .trailing
        case .bottomLeading: self = .bottomLeading
        case .bottom: self = .bottom
        case .bottomTrailing: self = .bottomTrailing
        }
    }
}
