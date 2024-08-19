import SwiftUI

struct NodeView: View {
    let node: Node
    let idPath: [Id]

    @EnvironmentObject private var root: Root

    var body: some View {
        switch node {
        case .empty:
            EmptyView()
        
        // MARK: Shape
        case .capsule:
            Capsule()
        case .circle:
            Circle()
        case .ellipse:
            Ellipse()
        case .rectangle:
            Rectangle()
        case let .roundedRectangle(cornerSize: cornerSize):
            RoundedRectangle(cornerSize: CGSize(cornerSize))

        // MARK: Widget
        case let .text(content: content):
            Text(content)
        case let .textField(content: content):
            TextField(text: Binding(
                get: { content },
                set: { root.fire(event: .updateText(content: $0), for: idPath) }
            )) {
                // TODO: Investigate adding a label
            }
        case let .button(label: label):
            Button {
                root.fire(event: .click, for: idPath)
            } label: {
                NodeView(node: label.value, idPath: idPath + [label.id])
            }
        case let .picker(title: title, selection: selection, content: content):
            Picker(title, selection: Binding(
                get: { selection },
                set: { root.fire(event: .updatePickerSelection(id: $0), for: idPath) }
            )) {
                NodeView(node: content.value, idPath: idPath + [content.id])
            }

        // MARK: Aggregation
        case let .child(wrapped: wrapped):
            NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
        case let .group(children: children):
            ForEach(children) { child in
                NodeView(node: child.value, idPath: idPath + [child.id])
            }

        // MARK: Layout
        case let .vStack(wrapped: wrapped):
            VStack {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }
        case let .hStack(wrapped: wrapped):
            HStack {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }
        case let .zStack(wrapped: wrapped):
            ZStack {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }
        case let .list(wrapped: wrapped):
            List {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }

        // MARK: Modifier
        case let .modified(wrapped: wrapped, modifier: modifier):
            NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
                .modifier(ModifierViewModifier(modifier: modifier))
        }
    }
}
