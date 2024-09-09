import SwiftUI

public extension SwiftUI.Edge {
    init(_ edge: Edge) {
        switch edge {
        case .top: self = .top
        case .bottom: self = .bottom
        case .leading: self = .leading
        case .trailing: self = .trailing
        }
    }
}

public extension SwiftUI.Edge.Set {
    init(_ edge: Edge) {
        self.init(SwiftUI.Edge(edge))
    }
}
