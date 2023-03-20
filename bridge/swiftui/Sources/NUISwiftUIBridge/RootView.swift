import SwiftUI

struct RootView: View {
    @EnvironmentObject private var root: NUIRoot

    var body: some View {
        PrimitiveView(primitive: root.primitive)
    }
}
