import CNUISwiftUIBridge
import Foundation
import Combine

class NUIViewFacade: ObservableObject {
    private let cView: UnsafePointer<CView>

    /// A manually installed publisher since we don't use `@Published`.
    var objectWillChange = ObservableObjectPublisher()

    /// The root view primitive.
    var primitive: Primitive {
        let cString = cView.pointee.render_json(cView)!
        defer { nui_c_string_drop(cString) }
        let json = String(cString: cString)
        let primitive = try! JSONDecoder().decode(Primitive.self, from: json.data(using: .utf8)!)
        return primitive
    }

    init(cView: UnsafePointer<CView>) {
        self.cView = cView
    }

    func triggerUpdate() {
        objectWillChange.send()
    }
}
