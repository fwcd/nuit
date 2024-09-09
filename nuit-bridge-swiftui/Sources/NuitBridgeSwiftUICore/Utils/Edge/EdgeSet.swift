public struct EdgeSet: OptionSet, Codable, Hashable {
    public typealias Element = Edge

    public var rawValue: UInt8

    public init(rawValue: UInt8) {
        self.rawValue = rawValue
    }

    public init(_ edge: Edge) {
        self.init(rawValue: 1 << edge.rawValue)
    }

    @discardableResult
    public mutating func update(with edge: Edge) -> Edge? {
        let inserted = !contains(edge)
        rawValue |= 1 << edge.rawValue
        return inserted ? nil : edge
    }

    @discardableResult
    public mutating func insert(_ edge: Edge) -> (inserted: Bool, memberAfterInsert: Edge) {
        let inserted = !contains(edge)
        update(with: edge)
        return (inserted: inserted, memberAfterInsert: edge)
    }

    @discardableResult
    public mutating func remove(_ edge: Edge) -> Edge? {
        let removed = contains(edge)
        rawValue &= ~(1 << edge.rawValue)
        return removed ? edge : nil
    }

    public func contains(_ edge: Edge) -> Bool {
        (rawValue & (1 << edge.rawValue)) != 0
    }
}

extension EdgeSet: Sequence {
    public struct Iterator: IteratorProtocol {
        let set: EdgeSet
        var i: UInt8 = 0

        public mutating func next() -> Edge? {
            while i < Edge.count {
                let edge = Edge(rawValue: i)!
                i += 1
                if set.contains(edge) {
                    return edge
                }
            }
            return nil
        }
    }

    public func makeIterator() -> Iterator {
        Iterator(set: self)
    }
}
