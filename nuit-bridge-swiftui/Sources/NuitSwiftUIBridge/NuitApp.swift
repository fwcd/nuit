import SwiftUI

class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        // https://stackoverflow.com/questions/68884499/make-swiftui-app-appear-in-the-macos-dock/71177509
        NSApplication.shared.setActivationPolicy(.regular)
        NSApplication.shared.activate(ignoringOtherApps: true)
    }
}

struct NuitApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    static var root: Root!

    var body: some Scene {
        WindowGroup {
            RootView()
                .environmentObject(Self.root)
        }
    }
}
