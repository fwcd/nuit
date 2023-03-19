import CNUISwiftUIBridge
import Foundation

@_cdecl("run_app")
func runApp(view: UnsafePointer<CView>) {
    NUIApp.view = view
    NUIApp.main()
}
