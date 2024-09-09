import SwiftUI

public extension SwiftUI.Alignment {
    init(_ alignment: Alignment) {
        self.init(
            horizontal: .init(alignment.horizontal),
            vertical: .init(alignment.vertical)
        )
    }
}
