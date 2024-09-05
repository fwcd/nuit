import SwiftUI

struct GestureNodeGesture: Gesture {
    let node: GestureNode
    let idPath: [Id]

    @EnvironmentObject private var root: Root

    var body: some Gesture {
        switch node {
        case let .tap(count: count):
            TapGesture(count: count)
                .onEnded { _ in root.fire(event: .gesture(gesture: .tap), for: idPath) }
        }
    }
}
