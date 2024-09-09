import NuitBridgeSwiftUICore
import SwiftUI
import XCTest

final class EdgeSetTests: XCTestCase {
    func testConversions() {
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet()), [])
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet(.top)), .top)
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet(.bottom)), .bottom)
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet(.leading)), .leading)
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet(.trailing)), .trailing)
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet([.leading, .trailing])), .horizontal)
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet([.top, .bottom])), .vertical)
        XCTAssertEqual(SwiftUI.Edge.Set(NuitBridgeSwiftUICore.EdgeSet([.trailing, .top, .bottom, .leading])), .all)
    }

    func testSequenceImplementation() {
        XCTAssertEqual(Array(NuitBridgeSwiftUICore.EdgeSet([.leading])), [.leading])
        XCTAssertEqual(Array(NuitBridgeSwiftUICore.EdgeSet([.top, .bottom])), [.top, .bottom])
        XCTAssertEqual(Array(NuitBridgeSwiftUICore.EdgeSet([.leading, .bottom, .trailing, .trailing])), [.bottom, .leading, .trailing])
        XCTAssertEqual(Array(NuitBridgeSwiftUICore.EdgeSet([.top, .bottom, .leading, .trailing])), [.top, .bottom, .leading, .trailing])
    }
}
