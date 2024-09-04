import SwiftUI

struct Sector: Shape {
    let startAngle: SwiftUI.Angle
    let endAngle: SwiftUI.Angle
    let innerRadiusFraction: Double

    func path(in rect: CGRect) -> Path {
        Path { path in
            let center = CGPoint(x: rect.midX, y: rect.midY)
            let outerRadius = min(rect.width, rect.height) / 2
            let innerRadius = innerRadiusFraction * outerRadius

            path.addArc(center: center, radius: outerRadius, startAngle: startAngle, endAngle: endAngle, clockwise: false)
            if innerRadius <= 0 {
                path.addLine(to: center)
            } else {
                path.addArc(center: center, radius: innerRadius, startAngle: endAngle, endAngle: startAngle, clockwise: true)
            }
        }
    }
}
