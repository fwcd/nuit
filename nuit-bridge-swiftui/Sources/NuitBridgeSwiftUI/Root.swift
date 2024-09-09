import CNuitBridgeSwiftUI
import NuitBridgeSwiftUICore
import Foundation
import Combine
import SwiftUI

/// The central state of the Nuit application.
class Root: ObservableObject {
    private let cRoot: UnsafePointer<CRoot>

    /// A manually installed publisher since we don't use `@Published`.
    var objectWillChange = ObservableObjectPublisher()

    /// The rendered root node.
    var node: Node {
        render()
    }

    init(cRoot: UnsafePointer<CRoot>) {
        self.cRoot = cRoot
    }

    func trigger(update: Update) {
        withAnimation(update.animation.map(SwiftUI.Animation.init)) {
            objectWillChange.send()
        }
    }

    // MARK: High-level FFI wrappers

    private func render() -> Node {
        let json = renderJson()
        let node = try! JSONDecoder().decode(Node.self, from: json.data(using: .utf8)!)
        return node
    }

    @discardableResult
    func fire(event: Event, for idPath: [Id]) -> EventResponse {
        let encoder = JSONEncoder()
        let idPathJson = String(data: try! encoder.encode(idPath), encoding: .utf8)!
        let eventJson = String(data: try! encoder.encode(event), encoding: .utf8)!
        let responseJson = fire(eventJson: eventJson, for: idPathJson)
        return try! JSONDecoder().decode(EventResponse.self, from: responseJson.data(using: .utf8)!)
    }

    // MARK: JSON FFI wrappers

    private func renderJson() -> String {
        let cString = cRoot.pointee.render_json(cRoot)!
        defer { nuit_c_string_drop(cString) }
        return String(cString: cString)
    }

    @discardableResult
    private func fire(eventJson: String, for idPathJson: String) -> String {
        let cString = cRoot.pointee.fire_event_json(cRoot, idPathJson, eventJson)!
        defer { nuit_c_string_drop(cString) }
        return String(cString: cString)
    }
}
