import SwiftUI

extension SwiftUI.Edge.Set {
    init(_ set: EdgeSet) {
        self.init()
        for edge in set {
            insert(SwiftUI.Edge.Set(SwiftUI.Edge(edge)))
        }
    }
}
