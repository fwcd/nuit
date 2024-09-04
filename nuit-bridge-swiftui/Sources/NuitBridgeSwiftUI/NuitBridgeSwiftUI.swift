import CNuitBridgeSwiftUI
import Foundation

@_cdecl("nuit_bridge_swiftui_run_app")
func runApp(cRoot: UnsafePointer<CRoot>) {
    NuitApp.root = Root(cRoot: cRoot)
    cRoot.pointee.set_update_callback(cRoot) {
        NuitApp.root.triggerUpdate()
    }
    NuitApp.main()
}
