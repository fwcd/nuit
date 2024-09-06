import SwiftUI

struct GestureNodeViewModifier: ViewModifier {
    let node: GestureNode
    let idPath: [Id]

    @EnvironmentObject private var root: Root

    func body(content: Content) -> some View {
        switch node {
        case let .tap(count: count):
            content.gesture(
                TapGesture(count: count)
                    .onEnded { _ in root.fire(event: .gesture(gesture: .tap), for: idPath) }
            )
        case let .drag(minimumDistance: minimumDistance):
            content.gesture(
                DragGesture(minimumDistance: minimumDistance)
                    .onChanged { value in
                        let event = DragEvent(
                            kind: .updated,
                            startLocation: Vec2(value.startLocation),
                            location: Vec2(value.location)
                        )
                        root.fire(event: .gesture(gesture: .drag(drag: event)), for: idPath)
                    }
                    .onEnded { value in
                        let event = DragEvent(
                            kind: .ended,
                            startLocation: Vec2(value.startLocation),
                            location: Vec2(value.location)
                        )
                        root.fire(event: .gesture(gesture: .drag(drag: event)), for: idPath)
                    }
            )
        }
    }
}
