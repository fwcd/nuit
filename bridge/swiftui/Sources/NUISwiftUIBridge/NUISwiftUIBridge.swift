import CNUISwiftUIBridge
import Foundation

@_cdecl("run_app")
func runApp(cRoot: UnsafePointer<CNUIRoot>) {
    NUIApp.root = NUIRoot(cRoot: cRoot)
    cRoot.pointee.set_update_callback(cRoot) {
        NUIApp.root.triggerUpdate()
    }
    NUIApp.main()
}
