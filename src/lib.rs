mod utils;

extern crate web_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

#[wasm_bindgen]
pub fn log_num(v: f64) {
    log!("{}", v);
}

#[wasm_bindgen]
pub fn gje(flat_a: Vec<f64>, mut b: Vec<f64>) {
    let nsiz: usize = b.len();
    let mut a: Vec<Vec<f64>> = Vec::new();
    for i in 0..nsiz {
        let mut temp: Vec<f64> = Vec::new();
        for j in 0..nsiz {
            temp.push(flat_a[nsiz * i + j]);
        }
        a.push(temp.clone());
    }
    log!("{:?} {:?}", a, b);
    for i in 0..(nsiz-1) {
        for j in i..nsiz {
            let ratio: f64 = a[j][i] / a[i][i];
            log!("{:?}", ratio);
            for k in i..nsiz {
                a[j][k] -= ratio * a[i][k];
                b[j] -= ratio * b[i];
            }
        }
    }
    let mut x: Vec<f64> = vec![0.0; nsiz];
    for (i, j) in (0..nsiz).rev().zip(0..nsiz) {
        let mut val: f64 = b[i];
        for k in 0..j {
            val -= a[i][a.len() - 1 - k] * b[a.len() - 1 - k];
        }
        val /= a[i][i];
        x.insert(0, val);
    }
}

#[wasm_bindgen]
pub struct State {
    points: Vec<(f64, f64)>,        // Airfoil Points
    q_inf: f64,                     // Freestream Velocity
    rho_inf: f64,                   // Freestream Density
    p_inf: f64,                     // Freestream Static Pressure
    aoa: f64,                       // Angle of Attack
    co_a: Vec<Vec<f64>>,            // Coefficient Matrix a
    rhs: Vec<f64>,                  // RHS of XFoil eq. 7
    gamma_0: Vec<Vec<f64>>,         // Vortex Sheet Str. @ aoa=0
    gamma_90: Vec<Vec<f64>>,        // Vortex Sheet Str. @ aoa=90
}

#[wasm_bindgen]
impl State {
    fn psi_gp(&self) {
        
    }

    pub fn log_points(&self) {
        log!("{:?}", self.points);
    }

    pub fn new(x: Vec<f64>, y: Vec<f64>, q_inf: f64, rho_inf: f64, p_inf: f64, aoa: f64) -> State {
        let mut points: Vec<(f64, f64)> = Vec::new();
        let co_a: Vec<Vec<f64>> = vec![vec![1.0; points.len() + 1]; points.len() + 1];
        let mut rhs: Vec<f64> = Vec::new();
        let gamma_0: Vec<Vec<f64>> = vec![vec![0.0; points.len()]; points.len()];
        let gamma_90 = gamma_0.clone();
        for i in x.iter().zip(y.iter()) {
            points.push((*i.0, *i.1));
        }
        for i in points.iter() {
            rhs.push(-q_inf * aoa.sin() * i.1 + q_inf * aoa.cos() * i.0);
        }
        log!("State Initialized. q_inf: {}, α: {}, ρ_inf: {}, p_inf: {}", q_inf, aoa, rho_inf, p_inf);
        
        State {
            points,
            q_inf,
            rho_inf,
            p_inf,
            aoa,
            rhs,
            co_a,
            gamma_0,
            gamma_90,
        }
    }
}