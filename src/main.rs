mod mathvetor;
mod esfera;
mod raio;
mod fonte_luz;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use mathvetor::Vec3;
use esfera::Esfera;
use raio:: Raio;
use fonte_luz::FonteLuz;

fn main() {

    let d_janela = 1.0;
    let r_esfera = 0.75;
    let centro_esfera = Vec3::new(0.0, 0.0, -(d_janela + r_esfera));

    let k_e = 5.0;
    let k_amb_esfera = Vec3::new(0.3, 0.0, 0.0);
    let k_dif_esfera = Vec3::new(0.7, 0.0, 0.0);
    let k_esp_esfera = Vec3::new(0.3, 0.3, 0.3);

    let e_amb = Vec3::new(1.0, 1.0, 1.0);

    let esfera = Esfera::new(centro_esfera, r_esfera, k_e, k_amb_esfera, k_dif_esfera, k_esp_esfera);
    let p0 = Vec3::new(0.0,0.0,0.0);
    let h_janela = 1.5;
    let w_janela = 1.5;
    let n_linhas = 512;
    let n_colunas = 512;
    let deltax = w_janela/(n_colunas as f32);
    let deltay = h_janela/(n_linhas as f32);
    let pse = Vec3::new(-w_janela/2.0, h_janela/2.0, -d_janela);
    let centro_00 = pse + Vec3::new(deltax/2.0, -deltay/2.0 ,0.0);
    let bg_cor = Color::RGB(100, 100, 100);

    let posicao_luz = Vec3::new(-1.6, 0.8, 0.0);
    let cor_fonte = Vec3::new(1.0, 1.0, 1.0);
    let intensidade_luz = 1.0;
    let fonte_luz = FonteLuz::new(posicao_luz, cor_fonte, intensidade_luz);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let window = video_subsystem
        .window("Computação Gráfica", n_colunas, n_linhas)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    for linha in 0..n_linhas {
        for coluna in 0..n_colunas {
            let dr = (centro_00 + Vec3::new(deltax * (coluna as f32), -deltay * (linha as f32), 0.0) - p0).normalize();
            let raio = Raio::new(p0, dr);

            let (intersecta, t1, t2) = raio.deteccao(&esfera);
            if intersecta == true && (t1 > 0.0 || t2 > 0.0) {
                
                let menor_t;
                if t1 > t2 && t2 > 0.0 {
                    menor_t = t2;
                } else {
                    menor_t = t1;
                }

                let ponto_intersercao = raio.em_t(menor_t);
                let v = -raio.dr;
                let n = (ponto_intersercao - esfera.c).normalize();
                let l = (fonte_luz.posicao - ponto_intersercao).normalize();
                let r = (2.0 * (l.dot(n))*n - l).normalize();

                let cor_luz = fonte_luz.cor * fonte_luz.intensidade;
                let i_amb = e_amb * esfera.k_amb;
                let i_dif = esfera.k_dif * (n.dot(l)) * cor_luz;
                let i_esp = esfera.k_esp * (r.dot(v)).powf(esfera.k_e) * cor_luz;
                let i_olho = (i_amb + i_dif + i_esp).clamp(0.0, 1.0);

                canvas.set_draw_color(vec_to_color(i_olho));
                canvas.draw_point(Point::new(coluna as i32, linha as i32)).unwrap();
            } else {
                canvas.set_draw_color(bg_cor);
                canvas.draw_point(Point::new(coluna as i32, linha as i32)).unwrap();
            }
        }
    }
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{ .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
    }
}

fn vec_to_color(v: Vec3) -> Color{
    let v = v * 255.0;
    return Color::RGB(v.x as u8, v.y as u8, v.z as u8);
}