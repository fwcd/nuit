public struct Alignment: Codable, Hashable {
    public let horizontal: HorizontalAlignment
    public let vertical: VerticalAlignment

    public init(
        horizontal: HorizontalAlignment,
        vertical: VerticalAlignment
    ) {
        self.horizontal = horizontal
        self.vertical = vertical
    }
}
