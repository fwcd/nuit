import CNuitBridgeSwiftUI
import Foundation

@_cdecl("nuit_bridge_swiftui_run_app")
func runApp(cRoot: UnsafePointer<CRoot>) {
    NuitApp.root = Root(cRoot: cRoot)

    cRoot.pointee.set_update_callback(cRoot) { updateJsonCString in
        let updateJson = String(cString: updateJsonCString!)
        let update = try! JSONDecoder().decode(Update.self, from: updateJson.data(using: .utf8)!)
        NuitApp.root.triggerUpdate()
    }

    NuitApp.main()
}
