extern crate image;

use std::f32;
use f32::consts::PI;
use f32::consts::FRAC_PI_2;

const PROD_PI_2: f32 = 2.0*PI;
const BORNE_INF_DICHO: f32 = FRAC_PI_2;
const BORNE_SUP_DICHO: f32 = FRAC_PI_2+PROD_PI_2;

// this is the maximum error of height
// you can change this as you wish
// 0.01 looks far enought in this example
const PRECISION: f32 = 0.01;

// all the mattering functions :


/// return x modulo y
fn real_mod(x: f32, y: f32) -> f32
{
    x - y*(x/y).floor()
}


/// the function φ whom inverse function Φ is needed
fn phi(x: f32) -> f32
{
    x + x.cos()
}

/**
* the dichotomic algorithm to get Φ(y)
* when starting < Φ(y) < ending 
*/
fn phi_inv_aux(
    y: f32,
    starting: f32,
    ending: f32,
) -> f32
{
    let middle = (starting+ending)/2.0;
    if ending-starting < PRECISION
    {
        middle
    }
    else if phi(middle) >= y
    {
        phi_inv_aux(y, starting, middle)
    }
    else
    {
        phi_inv_aux(y, middle, ending)
    }
}

/// the implementation of the inverse function Φ
fn phi_inv(y: f32) -> f32
{
    // we only need to calculate it on [pi/2, 5pi/2]
    // because it'll used in a sinus, and
    // for any real x and any integer k,
    // Φ(x+k*pi) = Φ(x) + k*pi (trust me (really))
    let y_mod = real_mod(y, PROD_PI_2)+FRAC_PI_2;
    phi_inv_aux(
        y_mod,
        BORNE_INF_DICHO,
        BORNE_SUP_DICHO
    )
}


/**
* the function that returns the height of a wave
* at a given spatio-temporal advancement.
* wave(x) = R*sin(Φ(x/R)) if R is the amplitude
*/
fn wave(
    progression: f32,
    amplitude: f32
) -> f32
{
    amplitude*( phi_inv( progression/amplitude ) ).sin()
}


/**
* same as wave(), but space is discrete, so both temporal and spatial dimensions
* has to be processed separatly
*/
fn discrete_wave(
    spatial_progression: f32,
    temporal_progression: f32,
    amplitude: f32,
    window: f32
) -> f32
{
    let progression =
        spatial_progression
        - real_mod( spatial_progression, window )
        + window/2.0
        + temporal_progression;
    
    wave( progression, amplitude )
}

////////////////////////////////////////////////////////////////////////////
// don't bother for the rest, it's just to make the pictures and run a test.
// and it might hurt your feelings

fn place_pixel(
    img: &mut image::RgbImage,
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

fn display_to_real(
    (x, y): (u32,u32),
    (res_x, res_y): (f32,f32),
    (w, h): (u32,u32)
) -> (f32,f32)
{
    let real_x =   ( (x as f32) - ((w/2) as f32) )*res_x;
    let real_y = - ( (y as f32) - ((h/2) as f32) )*res_y;
    
    (real_x, real_y)
}

fn real_to_display(
    (x, y): (f32,f32),
    (res_x, res_y): (f32,f32),
    (w, h): (u32,u32)
) -> (u32,u32)
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



fn main() {
    
    let w = 1000;
    let h = 500;
    let pi = f32::consts::PI;
    let res_x = 1.0/40.0;
    let res_y = 1.0/20.0;

    let res = (res_x, res_y);
    let dims = (w, h);

    let pix_bleu = image::Rgb([0, 0, 255]);    
    let pix_rouge = image::Rgb([255, 0, 0]);    

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
            let z = wave(x+speed*t, amplitude);
            let (_,j) = real_to_display((0.0, z), res, dims);
            place_pixel(&mut img, (i, j), pix_rouge);
            
        }
        img.save(
            format!("images/frame_{:03}.jpg", frame )
            ).unwrap();

    }

}

