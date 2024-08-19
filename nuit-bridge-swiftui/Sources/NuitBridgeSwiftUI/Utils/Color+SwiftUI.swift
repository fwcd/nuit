import SwiftUI

extension SwiftUI.Color {
    init(_ color: Color) {
        self.init(
            red: color.red,
            green: color.green,
            blue: color.blue,
            opacity: color.alpha
        )
    }
}
