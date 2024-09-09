import NuitBridgeSwiftUICore
import SwiftUI
import XCTest

final class ValueTests: XCTestCase {
    func testCoding() throws {
        let cases: [(String, Value, UInt)] = [
            ("true", true, #line),
            ("false", false, #line),
            ("42", 42, #line),
            ("42.5", 42.5, #line),
            ("[]", [], #line),
            ("[1,2,3]", [1, 2, 3], #line),
            ("[1,\"a\",3.2,null]", [1, "a", 3.2, nil], #line),
            ("{\"b\":[9,null,null]}", ["b": [9, nil, nil]], #line),
        ]
        
        for (json, value, line) in cases {
            XCTAssertEqual(
                try JSONDecoder().decode(Value.self, from: json.data(using: .utf8)!),
                value,
                "Could not decode JSON",
                line: line
            )

            XCTAssertEqual(
                String(data: try JSONEncoder().encode(value), encoding: .utf8)!,
                json,
                "Could not encode to JSON",
                line: line
            )
        }
    }
}
