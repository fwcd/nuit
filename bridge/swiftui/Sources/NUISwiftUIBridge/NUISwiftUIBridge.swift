import CNUISwiftUIBridge
import Foundation

@_cdecl("run_app")
func runApp(cRoot: UnsafePointer<CRoot>) {
    NUIApp.root = Root(cRoot: cRoot)
    cRoot.pointee.set_update_callback(cRoot) {
        NUIApp.root.triggerUpdate()
    }
    NUIApp.main()
}
