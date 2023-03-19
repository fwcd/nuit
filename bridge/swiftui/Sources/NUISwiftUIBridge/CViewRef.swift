import CNUISwiftUIBridge
import Foundation

struct CViewRef {
    private let cView: UnsafePointer<CView>

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
}
