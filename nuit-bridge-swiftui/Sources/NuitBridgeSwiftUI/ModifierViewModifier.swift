import SwiftUI

struct ModifierViewModifier: ViewModifier {
    let modifier: Modifier

    func body(content: Content) -> some View {
        switch modifier {
        case let .padding(insets: insets):
            content.padding(EdgeInsets(insets))
        }
    }
}
