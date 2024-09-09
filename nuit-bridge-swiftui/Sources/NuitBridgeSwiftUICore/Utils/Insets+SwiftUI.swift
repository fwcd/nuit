import SwiftUI

public extension EdgeInsets {
    init(_ insets: Insets) {
        self.init(
            top: insets.top,
            leading: insets.leading,
            bottom: insets.bottom,
            trailing: insets.trailing
        )
    }
}
