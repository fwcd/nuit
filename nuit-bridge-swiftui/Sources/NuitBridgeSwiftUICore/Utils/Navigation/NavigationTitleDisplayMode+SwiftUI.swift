import SwiftUI

#if !os(macOS)
public extension NavigationBarItem.TitleDisplayMode {
    init(_ displayMode: NavigationTitleDisplayMode) {
        switch displayMode {
        case .automatic: self = .automatic
        case .inline: self = .inline
        case .large: self = .large
        }
    }
}
#endif
