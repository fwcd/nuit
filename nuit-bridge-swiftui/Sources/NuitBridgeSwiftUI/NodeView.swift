import SwiftUI

struct NodeView: View {
    let node: Id<Node>

    @EnvironmentObject private var root: Root

    var body: some View {
        switch node.value {
        case .empty:
            EmptyView()
        case let .text(content: content):
            Text(content)
        case let .textField(content: content):
            TextField(text: Binding(
                get: { content },
                set: { root.fire(event: .updateText(content: $0), for: node.idPath) }
            )) {
                // TODO: Investigate adding a label
            }
        case let .button(label: label):
            Button {
                root.fire(event: .click, for: node.idPath)
            } label: {
                NodeView(node: label)
            }
        case let .vStack(wrapped: wrapped):
            VStack {
                NodeView(node: wrapped)
            }
        case let .hStack(wrapped: wrapped):
            HStack {
                NodeView(node: wrapped)
            }
        case let .zStack(wrapped: wrapped):
            ZStack {
                NodeView(node: wrapped)
            }
        case let .tuple2(child1: child1, child2: child2):
            NodeView(node: child1)
            NodeView(node: child2)
        case let .tuple3(child1: child1, child2: child2, child3: child3):
            NodeView(node: child1)
            NodeView(node: child2)
            NodeView(node: child3)
        case let .tuple4(child1: child1, child2: child2, child3: child3, child4: child4):
            NodeView(node: child1)
            NodeView(node: child2)
            NodeView(node: child3)
            NodeView(node: child4)
        case let .tuple5(child1: child1, child2: child2, child3: child3, child4: child4, child5: child5):
            NodeView(node: child1)
            NodeView(node: child2)
            NodeView(node: child3)
            NodeView(node: child4)
            NodeView(node: child5)
        }
    }
}
