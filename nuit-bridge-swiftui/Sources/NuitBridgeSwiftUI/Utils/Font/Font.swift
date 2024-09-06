enum Font: Codable, Hashable {
    case system(size: FontSize, design: FontDesign?, weight: FontWeight?)
    case custom(name: String, size: Double)
}
