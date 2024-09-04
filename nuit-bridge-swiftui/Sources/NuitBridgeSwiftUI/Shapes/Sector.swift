import SwiftUI

struct Sector: Shape {
    let startAngle: SwiftUI.Angle
    let endAngle: SwiftUI.Angle
    let outerRadius: Double
    let innerRadius: Double

    func path(in rect: CGRect) -> Path {
        Path { path in
            let center = CGPoint(x: rect.midX, y: rect.midY)
            path.addArc(center: center, radius: outerRadius, startAngle: startAngle, endAngle: endAngle, clockwise: false)
            path.addLine(to: center)
        }
    }
}
