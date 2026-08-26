
/// Возвращаем просто связку с путем, без лишних усложнений
pub struct RouteResult {
    pub source: String,
    pub destination: String,
    pub weight: f64,
}