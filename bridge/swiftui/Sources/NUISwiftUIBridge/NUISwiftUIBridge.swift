import CNUISwiftUIBridge
import Foundation

@_cdecl("run_app")
func runApp(view: UnsafePointer<CView>) {
    NUIApp.rootView = CViewRef(cView: view)
    NUIApp.main()
}
