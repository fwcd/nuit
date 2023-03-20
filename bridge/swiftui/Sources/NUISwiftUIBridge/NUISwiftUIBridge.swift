import CNUISwiftUIBridge
import Foundation

@_cdecl("run_app")
func runApp(cRoot: UnsafePointer<CNUIRoot>) {
    NUIApp.root = NUIRoot(cRoot: cRoot)
    NUIApp.main()
}
