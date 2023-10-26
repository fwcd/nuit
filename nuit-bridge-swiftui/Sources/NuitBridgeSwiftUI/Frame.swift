enum Frame: Codable, Hashable {
    case constrained(
        minWidth: Double?,
        idealWidth: Double?,
        maxWidth: Double?,
        minHeight: Double?,
        idealHeight: Double?,
        maxHeight: Double?
    )
    case exact(
        width: Double?,
        height: Double?
    )
}
