import SwiftUI

struct NodeView: View {
    let node: Id<Node>

    @EnvironmentObject private var root: Root

    var body: some View {
        switch node.value {
        // MARK: Primitive
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

        // MARK: Aggregation
        case let .group(children: children):
            ForEach(children) { child in
                NodeView(node: child)
            }

        // MARK: Layout
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

        // MARK Modifier
        case let .padding(wrapped: wrapped, insets: insets):
            NodeView(node: wrapped)
                .padding(EdgeInsets(insets))
        }
    }
}
