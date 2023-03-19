import SwiftUI

struct PrimitiveView: View {
    let primitive: Primitive

    var body: some View {
        switch primitive {
        case let .text(content: content):
            Text(content)
        case let .vStack(wrapped: wrapped):
            VStack {
                PrimitiveView(primitive: wrapped)
            }
        case let .hStack(wrapped: wrapped):
            HStack {
                PrimitiveView(primitive: wrapped)
            }
        case let .zStack(wrapped: wrapped):
            ZStack {
                PrimitiveView(primitive: wrapped)
            }
        case let .group(children: children):
            Group {
                // TODO: Figure out a proper way to identify these views
                ForEach(children, id: \.self) { child in
                    PrimitiveView(primitive: child)
                }
            }
        }
    }
}
