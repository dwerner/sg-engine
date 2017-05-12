use Identity;

#[derive(Debug, Copy, Clone)]
pub enum UIEvent {
    Clicked(Identity),
    GainedFocus(Identity),
    LeftFocus(Identity),
}