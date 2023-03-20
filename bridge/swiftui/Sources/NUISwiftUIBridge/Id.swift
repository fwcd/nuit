struct Id<Value>: Identifiable {
    let idPath: [Int]
    let value: Value

    var id: [Int] { idPath }
}

extension Id: Equatable where Value: Equatable {}

extension Id: Hashable where Value: Hashable {}

extension Id: Encodable where Value: Encodable {}

extension Id: Decodable where Value: Decodable {}
