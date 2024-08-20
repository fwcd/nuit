import SwiftUI

struct ShapeNodeView: View {
    let shape: ShapeNode

    var body: some View {
        AnyShape(shape)
    }
}
