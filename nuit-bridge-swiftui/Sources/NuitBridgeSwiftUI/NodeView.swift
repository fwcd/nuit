import NuitBridgeSwiftUICore
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
                childView(for: label)
            }
        case let .picker(title: title, selection: selection, content: content):
            Picker(title, selection: Binding(
                get: { selection },
                set: { root.fire(event: .updatePickerSelection(id: $0), for: idPath) }
            )) {
                childView(for: content)
            }
        case let .slider(value: value, lowerBound: lowerBound, upperBound: upperBound, step: step):
            let binding = Binding(
                get: { value },
                set: { root.fire(event: .updateSliderValue(value: $0), for: idPath) }
            )
            if let step {
                Slider(value: binding, in: lowerBound...upperBound, step: step)
            } else {
                Slider(value: binding, in: lowerBound...upperBound)
            }

        // MARK: Aggregation
        case let .child(wrapped: wrapped):
            childView(for: wrapped)
        case let .group(children: children):
            ForEach(children) { child in
                childView(for: child)
            }

        // MARK: Layout
        case .geometryReader:
            GeometryReader { proxy in
                if case let .node(node: node) = root.fire(event: .getGeometryReaderView(geometry: .init(proxy)), for: idPath) {
                    childView(for: node)
                }
            }
        case let .vStack(alignment: alignment, spacing: spacing, wrapped: wrapped):
            VStack(alignment: .init(alignment), spacing: spacing) {
                childView(for: wrapped)
            }
        case let .hStack(alignment: alignment, spacing: spacing, wrapped: wrapped):
            HStack(alignment: .init(alignment), spacing: spacing) {
                childView(for: wrapped)
            }
        case let .zStack(alignment: alignment, spacing: _, wrapped: wrapped):
            // TODO: Apply spacing on visionOS
            ZStack(alignment: .init(alignment)) {
                childView(for: wrapped)
            }
        case let .list(wrapped: wrapped):
            List {
                childView(for: wrapped)
            }
        case let .scrollView(axes: axes, showIndicators: showIndicators, wrapped: wrapped):
            ScrollView(axes.swiftUIAxes, showsIndicators: showIndicators) {
                childView(for: wrapped)
            }
        case let .overlay(wrapped: wrapped, alignment: alignment, overlayed: overlayed):
            childView(for: wrapped)
                .overlay(alignment: .init(alignment)) {
                    childView(for: overlayed)
                }
        
        // MARK: Navigation
        case let .navigationStack(path: path, wrapped: wrapped):
            if let path {
                NavigationStack(path: Binding(
                    get: { path },
                    set: { root.fire(event: .updateNavigationPath(path: $0), for: idPath) }
                )) {
                    childView(for: wrapped)
                }
            } else {
                NavigationStack {
                    childView(for: wrapped)
                }
            }
        case let .navigationSplitView(sidebar: sidebar, content: content, detail: detail):
            if content.value.isEmpty {
                NavigationSplitView {
                    childView(for: sidebar)
                } detail: {
                    childView(for: detail)
                }
            } else {
                NavigationSplitView {
                    childView(for: sidebar)
                } content: {
                    childView(for: content)
                } detail: {
                    childView(for: detail)
                }
            }
        case let .navigationLink(label: label, value: value):
            NavigationLink(value: value) {
                NodeView(node: label.value, idPath: idPath + [label.id])
            }
        case let .navigationDestination(wrapped: wrapped):
            childView(for: wrapped)
                .navigationDestination(for: Value.self) { value in
                    if case let .node(node: destination) = root.fire(event: .getNavigationDestination(value: value), for: idPath) {
                        childView(for: destination)
                    }
                }

        // MARK: Wrapper
        case let .shape(shape: shape):
            ShapeNodeView(shape: shape)
        case let .gestured(wrapped: wrapped, gesture: gesture):
            childView(for: wrapped)
                .modifier(GestureNodeViewModifier(node: gesture.value, idPath: idPath + [gesture.id]))
        case let .modified(wrapped: wrapped, modifier: modifier):
            childView(for: wrapped)
                .modifier(ModifierNodeViewModifier(modifier: modifier))
        }
    }

    private func childView(for child: Identified<Node>) -> some View {
        NodeView(node: child.value, idPath: idPath + [child.id])
    }
}
