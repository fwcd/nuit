import SwiftUI

struct RootView: View {
    @EnvironmentObject private var root: Root

    var body: some View {
        NodeView(node: root.node, idPath: [])
    }
}
