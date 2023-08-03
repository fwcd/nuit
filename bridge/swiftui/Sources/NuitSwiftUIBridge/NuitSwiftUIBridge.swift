import CNuitSwiftUIBridge
import Foundation

@_cdecl("run_app")
func runApp(cRoot: UnsafePointer<CRoot>) {
    NuitApp.root = Root(cRoot: cRoot)
    cRoot.pointee.set_update_callback(cRoot) {
        NuitApp.root.triggerUpdate()
    }
    NuitApp.main()
}
