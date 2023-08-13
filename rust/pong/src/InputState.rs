pub struct InputState {
    xaxis: f32,
    yaxis: f32,
    fire: bool,
}
impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            fire: false,
        }
    }
}