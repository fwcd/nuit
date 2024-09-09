import SwiftUI

public extension Geometry {
    init(_ proxy: GeometryProxy) {
        self.init(size: .init(proxy.size))
    }
}
