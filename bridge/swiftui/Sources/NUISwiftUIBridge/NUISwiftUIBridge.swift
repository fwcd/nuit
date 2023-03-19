import CNUISwiftUIBridge
import Foundation

@_cdecl("run_app")
func runApp(view: UnsafePointer<CView>) {
    let json = String(cString: view.pointee.render_json(view)!)
    let primitive = try! JSONDecoder().decode(Primitive.self, from: json.data(using: .utf8)!)
    print(primitive)
    NUIApp.main()
}
