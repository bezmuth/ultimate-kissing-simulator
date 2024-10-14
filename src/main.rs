use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Ultimate Kissing Simulator".to_owned(),
        fullscreen: false,
        window_height: 464,
        window_width: 824,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let no_kiss_texture: Texture2D = load_texture("assets/1.png").await.unwrap();
    let kiss_texture: Texture2D = load_texture("assets/2.png").await.unwrap();
    let mut latch = false;
    let mut score = 0;
    loop {
        clear_background(WHITE);
        if is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
            draw_texture(&kiss_texture, 0., 0., WHITE);
            if !latch{
                score+=1;
            }
            latch = true;
        } else {
            latch = false;
            draw_texture(&no_kiss_texture, 0., 0., WHITE);
        }
        draw_text("ULTIMATE KISSING SIMULATOR", 40.0, 20.0, 30.0, BLACK);
        draw_text(&format!("KISSES: {}", score), 650.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
