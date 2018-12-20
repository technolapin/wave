extern crate image;


// implémentation de fonctions diverses

fn real_mod(x: f32, y: f32) -> f32
{
    x - y*(x/y).floor()
}

fn phi(x: f32) -> f32
{
    x + x.cos()
}


fn phi_inv_aux(y: f32, d: f32, f: f32, pre: f32) -> f32
{
    let m = (d+f)/2.0;
    if f-d < pre
    {
        m
    }
    else if phi(m) >= y
    {
        phi_inv_aux(y, d, m, pre)
    }
    else
    {
        phi_inv_aux(y, m, f, pre)
    }
}

/**
* la fonction clée pas du tout optimisée
* la précision se règle ici (et donc la rapidité)
* 0.01 me semble amplement suffisant
*/
fn phi_inv(y: f32) -> f32
{
    let pre = 0.01; // la précision, sachant que la complexité est en log(1/pre)
    let pi = f32::consts::PI;
    
    let d = pi / 2.0;
    let f = 2.0*pi+d;
    let y_mod = real_mod(y, 2.0*pi)+d;
    phi_inv_aux(y_mod, d, f, pre)
}


// Après c'est pour l'affichage


fn trace(
    img: &mut image::RgbImage,
    x: f32,
    y: f32,
    w: u32,
    h: u32,
    dx: f32,
    dy: f32,
    res_x: f32,
    res_y: f32,
    pix: image::Rgb<u8>
)
{
    let real_x = x + dx*2.0;
    let real_y = y + dy*2.0;

    if real_x >= 0.0 && real_x < res_x*(w as f32)
    {
        (*img).put_pixel((real_x/res_x) as u32,
                         (real_y/res_y) as u32,
                      pix);
    }
    else
    {
        (*img).put_pixel(0,
                      0,
                      image::Rgb([255,0,0]));
    }
}

fn place_pixel(img: &mut image::RgbImage,
               (x, y): (u32,u32),
               pix: image::Rgb<u8>
)
{
    let (w, h) = (*img).dimensions();
    if w > x && h > y
    {
        (*img).put_pixel(x, y, pix)
    }
    else
    {
        println!("ERROR: ({}, {}) out of bound.", x, y);
    }
}

fn display_to_real( (x, y): (u32,u32),
                     (res_x, res_y): (f32,f32),
                     (w, h): (u32,u32) ) -> (f32,f32)
{
    let real_x =   ( (x as f32) - ((w/2) as f32) )*res_x;
    let real_y = - ( (y as f32) - ((h/2) as f32) )*res_y;
    
    (real_x, real_y)
}

fn real_to_display( (x, y): (f32,f32),
                     (res_x, res_y): (f32,f32),
                     (w, h): (u32,u32) ) -> (u32,u32)
{
    let i = (x/res_x) as i32 + (w/2) as i32;
    let j = -(y/res_y) as i32 + (h/2) as i32;

    if i >= 0 && (i as u32) < w &&
        j >= 0 && (j as u32) < h
    {
        (i as u32, j as u32)
    }
    else
    {
        (w/2, h/2)
    }
}

fn wave(x: f32, ampl: f32) -> f32
{
    ampl*(phi_inv(x/ampl)).sin()
}


fn discrete_wave(x: f32, t: f32, ampl: f32, window: f32) -> f32
{
    let progression = x - real_mod(x, window)+window/2.0+t;
    wave(progression, ampl)
}



use std::f32;
fn main() {

    let w = 1000;
    let h = 500;
    let pi = f32::consts::PI;
    let res_x = 1.0/40.0;
    let res_y = 1.0/20.0;

    let res = (res_x, res_y);
    let dims = (w, h);

    let pix_bleu = image::Rgb([0, 0, 255]);    

    let amplitude = 4.0;
    let nombre_frames = 200;
    
    let period = 2.0*pi*amplitude;
    let window = 1.0;

    let speed = period/(nombre_frames as f32);
    
    for frame in 0..nombre_frames
    {
        let t = frame as f32;
        let mut img = image::RgbImage::new(w, h);
        for i in 0..w
        {
            let (x, _) = display_to_real((i, 0), (res_x, res_y), (w, h));
            let y = discrete_wave(x, speed*t, amplitude, window);

            let (_,j) = real_to_display((0.0, y), res, dims);
            for k in j..h
            {
                place_pixel(&mut img, (i, k), pix_bleu);
            }            
            
        }
        img.save(
            format!("images/frame_{:03}.jpg", frame )
            ).unwrap();

    }

}

