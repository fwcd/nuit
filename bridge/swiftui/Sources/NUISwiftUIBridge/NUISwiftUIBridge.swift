import CNUISwiftUIBridge

@_cdecl("run_app")
func runApp(view: UnsafePointer<CView>) {
    print(String(cString: view.pointee.render_json(view)!))
    NUIApp.main()
}
