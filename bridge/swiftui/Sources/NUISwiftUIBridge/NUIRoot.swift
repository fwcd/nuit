import CNUISwiftUIBridge
import Foundation
import Combine

class NUIRoot: ObservableObject {
    private let cRoot: UnsafePointer<CNUIRoot>

    /// A manually installed publisher since we don't use `@Published`.
    var objectWillChange = ObservableObjectPublisher()

    /// The rendered root primitive.
    var primitive: Id<Primitive> {
        let json = renderJson()
        let primitive = try! JSONDecoder().decode(Id<Primitive>.self, from: json.data(using: .utf8)!)
        return primitive
    }

    init(cRoot: UnsafePointer<CNUIRoot>) {
        self.cRoot = cRoot
    }

    func triggerUpdate() {
        objectWillChange.send()
    }

    private func renderJson() -> String {
        let cString = cRoot.pointee.render_json(cRoot)!
        defer { nui_c_string_drop(cString) }
        return String(cString: cString)
    }
}
