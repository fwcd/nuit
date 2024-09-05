import SwiftUI

struct NodeView: View {
    let node: Node
    let idPath: [Id]

    @EnvironmentObject private var root: Root

    var body: some View {
        switch node {
        case .empty:
            EmptyView()
        
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
                root.fire(event: .buttonTap, for: idPath)
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
        case let .vStack(spacing: spacing, wrapped: wrapped):
            VStack(spacing: spacing) {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }
        case let .hStack(spacing: spacing, wrapped: wrapped):
            HStack(spacing: spacing) {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }
        case let .zStack(spacing: _, wrapped: wrapped):
            // TODO: Apply spacing on visionOS
            ZStack {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }
        case let .list(wrapped: wrapped):
            List {
                NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
            }
        case let .overlay(wrapped: wrapped, alignment: alignment, overlayed: overlayed):
            NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
                .overlay(alignment: .init(alignment)) {
                    NodeView(node: overlayed.value, idPath: idPath + [overlayed.id])
                }

        // MARK: Wrapper
        case let .shape(shape: shape):
            ShapeNodeView(shape: shape)
        case let .gestured(wrapped: wrapped, gesture: gesture):
            let view = NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
                .modifier(GestureNodeViewModifier(node: gesture.value, idPath: idPath + [gesture.id]))
        case let .modified(wrapped: wrapped, modifier: modifier):
            NodeView(node: wrapped.value, idPath: idPath + [wrapped.id])
                .modifier(ModifierNodeViewModifier(modifier: modifier))
        }
    }
}
