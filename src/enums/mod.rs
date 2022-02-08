use termion::color;

type Bg = color::Bg<color::Rgb>;
type Fg = color::Fg<color::Rgb>;

#[derive(Copy, Clone)]
pub enum State {
    Correct,
    Present,
    Absent,
    Unused,
}

impl State {
    pub fn get_color(&self) -> (Bg, Fg) {
        let bg = match self {
            State::Correct => color::Bg(color::Rgb(83, 141, 78)),
            State::Present => color::Bg(color::Rgb(181, 159, 59)),
            State::Absent => color::Bg(color::Rgb(58, 58, 60)),
            State::Unused => color::Bg(color::Rgb(129, 131, 132)),
        };
        (bg, color::Fg(color::Rgb(215, 218, 220)))
    }
}
