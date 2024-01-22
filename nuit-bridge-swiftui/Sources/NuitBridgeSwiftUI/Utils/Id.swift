public enum Id: Codable, Hashable {
    case index(value: Int)
    case string(value: String)

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if let index = try? container.decode(Int.self) {
            self = .index(value: index)
        } else if let value = try? container.decode(String.self) {
            self = .string(value: value)
        } else {
            throw DecodingError.dataCorrupted(.init(
                codingPath: decoder.codingPath,
                debugDescription: "Expected id, but found neither integer index nor string"
            ))
        }
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .index(value: let value):
            try container.encode(value)
        case .string(value: let value):
            try container.encode(value)
        }
    }
}
