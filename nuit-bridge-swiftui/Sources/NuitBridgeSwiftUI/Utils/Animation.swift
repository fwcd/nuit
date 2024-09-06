import Foundation

enum Animation: Codable, Hashable {
    case `default`
    case curve(curve: Curve, durationSeconds: TimeInterval?)

    enum Curve: String, Codable, Hashable {
        case linear
        case easeIn
        case easeOut
        case easeInOut
    }
}
