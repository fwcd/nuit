public struct Identified<Value>: Identifiable {
    public let id: Id
    public let value: Value
    
    public init(id: Id, value: Value) {
        self.id = id
        self.value = value
    }
}

extension Identified: Equatable where Value: Equatable {}

extension Identified: Hashable where Value: Hashable {}

extension Identified: Encodable where Value: Encodable {}

extension Identified: Decodable where Value: Decodable {}
