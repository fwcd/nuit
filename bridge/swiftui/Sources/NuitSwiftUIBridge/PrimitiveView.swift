import SwiftUI

struct PrimitiveView: View {
    let primitive: Id<Primitive>

    @EnvironmentObject private var root: Root

    var body: some View {
        switch primitive.value {
        case .empty:
            EmptyView()
        case let .text(content: content):
            Text(content)
        case let .textField(content: content):
            TextField(text: Binding(
                get: { content },
                set: { root.fire(event: .updateText(content: $0), for: primitive.idPath) }
            )) {
                // TODO: Investigate adding a label
            }
        case let .button(label: label):
            Button {
                root.fire(event: .click, for: primitive.idPath)
            } label: {
                PrimitiveView(primitive: label)
            }
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
        case let .tuple2(child1: child1, child2: child2):
            PrimitiveView(primitive: child1)
            PrimitiveView(primitive: child2)
        case let .tuple3(child1: child1, child2: child2, child3: child3):
            PrimitiveView(primitive: child1)
            PrimitiveView(primitive: child2)
            PrimitiveView(primitive: child3)
        case let .tuple4(child1: child1, child2: child2, child3: child3, child4: child4):
            PrimitiveView(primitive: child1)
            PrimitiveView(primitive: child2)
            PrimitiveView(primitive: child3)
            PrimitiveView(primitive: child4)
        case let .tuple5(child1: child1, child2: child2, child3: child3, child4: child4, child5: child5):
            PrimitiveView(primitive: child1)
            PrimitiveView(primitive: child2)
            PrimitiveView(primitive: child3)
            PrimitiveView(primitive: child4)
            PrimitiveView(primitive: child5)
        }
    }
}
