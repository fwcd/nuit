import SwiftUI

extension SwiftUI.Font.TextStyle {
    init(_ weight: FontLevel) {
        switch weight {
        #if os(macOS)
        case .extraLargeTitle2: self = .largeTitle
        case .extraLargeTitle: self = .largeTitle
        #else
        case .extraLargeTitle2: self = .extraLargeTitle2
        case .extraLargeTitle: self = .extraLargeTitle
        #endif
        case .largeTitle: self = .largeTitle
        case .title: self = .title
        case .title2: self = .title2
        case .title3: self = .title3
        case .headline: self = .headline
        case .subheadline: self = .subheadline
        case .body: self = .body
        case .callout: self = .callout
        case .caption: self = .caption
        case .caption2: self = .caption2
        case .footnote: self = .footnote
        }
    }
}
