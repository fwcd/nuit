struct Vec2<T> {
    let x: T
    let y: T
}

extension Vec2: Equatable where T: Equatable {}

extension Vec2: Hashable where T: Hashable {}

extension Vec2: Encodable where T: Encodable {}

extension Vec2: Decodable where T: Decodable {}
