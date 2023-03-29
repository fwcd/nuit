import SwiftUI

struct RootView: View {
    @EnvironmentObject private var root: Root

    var body: some View {
        PrimitiveView(primitive: root.primitive)
    }
}
