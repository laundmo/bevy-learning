use crate::misc::resources::ScreenLoc;
use bevy::prelude::*;

pub fn screen_edge_system(
    mut screen: ResMut<ScreenLoc>,
    // need to get window dimensions
    wnds: Res<Windows>,
) {
    // get the window that the camera is displaying to

    let wnd = wnds.primary();

    let w = wnd.width() / 2.0;
    let h = wnd.height() / 2.0;

    screen.sides.bottom = -h;
    screen.sides.left = -w;

    screen.sides.top = h;
    screen.sides.right = w;
}
