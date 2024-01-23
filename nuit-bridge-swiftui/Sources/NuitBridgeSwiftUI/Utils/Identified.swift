struct Identified<Value>: Identifiable {
    let id: Id
    let value: Value
}

extension Identified: Equatable where Value: Equatable {}

extension Identified: Hashable where Value: Hashable {}

extension Identified: Encodable where Value: Encodable {}

extension Identified: Decodable where Value: Decodable {}
