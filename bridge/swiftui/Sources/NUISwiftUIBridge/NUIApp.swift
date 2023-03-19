import SwiftUI
import CNUISwiftUIBridge

class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        // https://stackoverflow.com/questions/68884499/make-swiftui-app-appear-in-the-macos-dock/71177509
        NSApplication.shared.setActivationPolicy(.regular)
        NSApplication.shared.activate(ignoringOtherApps: true)
    }
}

struct NUIApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    static var view: UnsafePointer<CView>!
    private static var primitive: Primitive {
        let cString = view.pointee.render_json(view)!
        let json = String(cString: cString)
        let primitive = try! JSONDecoder().decode(Primitive.self, from: json.data(using: .utf8)!)
        nui_c_string_drop(cString)
        return primitive
    }

    var body: some Scene {
        WindowGroup {
            PrimitiveView(primitive: Self.primitive)
        }
    }
}
