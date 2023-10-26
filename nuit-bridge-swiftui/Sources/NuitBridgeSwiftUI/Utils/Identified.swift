struct Identified<Value>: Identifiable {
    let idPath: [Id]
    let value: Value

    var id: [Id] { idPath }
}

extension Identified: Equatable where Value: Equatable {}

extension Identified: Hashable where Value: Hashable {}

extension Identified: Encodable where Value: Encodable {}

extension Identified: Decodable where Value: Decodable {}
