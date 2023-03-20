import CNUISwiftUIBridge
import Foundation
import Combine

class NUIRoot: ObservableObject {
    private let cRoot: UnsafePointer<CNUIRoot>

    /// A manually installed publisher since we don't use `@Published`.
    var objectWillChange = ObservableObjectPublisher()

    /// The root view primitive.
    var primitive: Primitive {
        let cString = cRoot.pointee.render_json(cRoot)!
        defer { nui_c_string_drop(cString) }
        let json = String(cString: cString)
        let primitive = try! JSONDecoder().decode(Primitive.self, from: json.data(using: .utf8)!)
        return primitive
    }

    init(cRoot: UnsafePointer<CNUIRoot>) {
        self.cRoot = cRoot
    }

    func triggerUpdate() {
        objectWillChange.send()
    }
}
