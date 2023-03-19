import CNUISwiftUIBridge

@_cdecl("bootstrap")
func bootstrap(view: UnsafePointer<CView>) {
    print(String(cString: view.pointee.render_json(view)!))
    NUIApp.main()
}
