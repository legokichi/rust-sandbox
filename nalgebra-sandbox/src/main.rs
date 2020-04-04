use nalgebra::base::*;
use nalgebra_lapack::Eigen;
use plotters::prelude::*;
use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use font_kit::properties::{Properties, Style, Weight};
use rusttype::{point, Error, Font, FontCollection, Scale, SharedBytes};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    
    // let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480))
    //     .into_drawing_area();
    // root.fill(&WHITE)?;

    // let mut chart = ChartBuilder::on(&root)
    //     .caption("y=x^2",
    //         plotters::style::FontDesc::new(
    //             plotters::style::FontFamily::Name("FreeMono"),
    //             10.0,
    //             plotters::style::FontStyle::Normal
    //         )
    //     )
    //     .margin(10)
    //     .x_label_area_size(30)
    //     .y_label_area_size(30)
    //     .build_ranged(-1f32..1f32, -0.1f32..1f32)?;
    // let mut chart = chart.configure_mesh();
    // chart.draw()?;



    // let m = Matrix2::new(
    //     1.0, 1.0,
    //     0.0, 1.0);
    // let o = m.symmetric_eigen();
    // println!("m: {}", m);
    // println!("eigenvectors: {}", o.eigenvectors);
    // println!("eigenvalues: {}", o.eigenvalues);
    // let o = m.transpose().symmetric_eigen();
    // println!("m: {}", m.transpose());
    // println!("eigenvectors: {}", o.eigenvectors);
    // println!("eigenvalues: {}", o.eigenvalues);

    real_linear_space_2d();
    Ok(())
}

fn real_linear_space_2d(){
    // scalar (field)
    let zero = 0.0;
    let one = 1.0;
    // field operation
    // addition
    // zero as addition identity element
    assert_eq!(zero + zero, zero);
    assert_eq!(zero + one, one);
    assert_eq!(one + one, one + one);
    assert_eq!(one + -one, zero);
    // subtraction
    assert_eq!(zero - zero, zero);
    assert_eq!(zero - one, - one);
    assert_eq!(one - zero, one);
    assert_eq!(one - one, zero);
    // multiplication
    // zero as multiplication zero element
    assert_eq!(zero * zero, zero);
    assert_eq!(one * zero, zero);
    // one as multiplication identity element
    assert_eq!(zero * one, zero);
    assert_eq!(one * one, one);
    // division
    assert!((zero / zero as f64).is_nan());
    assert!((one / zero as f64).is_infinite());
    assert_eq!(zero / one, zero);
    assert_eq!(one / one, one);
    assert_eq!((one + one) / one, (one + one));
    // vector
    let zero_vec = Vector2::new(0.0,0.0);
    // canonical basis
    let one_x = Vector2::new(1.0,0.0);
    let one_y = Vector2::new(0.0,1.0);
    // vector addition
    // zero_vec as addition identity element
    assert_eq!(zero_vec + zero_vec, zero_vec);
    assert_eq!(zero_vec + one_x, one_x);
    assert_eq!(one_x + one_x, one_x + one_x);
    assert_eq!(one_x + -one_x, zero_vec);
    // scalar multiplication
    assert_eq!(zero * zero_vec, zero_vec);
    assert_eq!(zero * one_x, zero_vec);
    assert_eq!(one * zero_vec, zero_vec);
    assert_eq!(one * one_x, one_x);
    assert_eq!((one + one) * one_x, (one * one_x) + (one * one_x));
    // linearly dependent (one_x and one_x)
    let a = one; let b = -one;
    assert_eq!(a * one_x + b * one_x, zero_vec);
    // linearly independent (one_x and one_y)
    // basis are linearly independent
    let a = zero; let b = zero;
    assert_eq!(a * one_x + b * one_y, zero_vec);

    // Vector spaces with additional structure
    // Normed vector spaces and inner product spaces(metric vector spaces)
    // inner product
    assert_eq!(zero_vec.dot(&zero_vec), zero);
    assert_eq!(zero_vec.dot(&one_x), zero);
    assert_eq!(one_x.dot(&zero_vec), zero);
    assert_eq!(one_x.dot(&one_x), one);
    assert_eq!(one_x.dot(&one_y), zero);
    assert_eq!(one_y.dot(&one_x), zero);
    assert_eq!(one_y.dot(&one_y), one);
    // norm
    assert_eq!(one_y.norm(), one_y.dot(&one_y).sqrt());
    assert_eq!(one_x.norm(), one_x.dot(&one_x).sqrt());
    // metric distance
    assert_eq!(zero_vec.metric_distance(&zero_vec), ((zero_vec - zero_vec) + (zero_vec - zero_vec)).norm());
    assert_eq!(one_x.metric_distance(&zero_vec), ((one_x - one_x) + (zero_vec - one_x)).norm());
    assert_eq!(one_x.metric_distance(&one_x), ((one_x - one_x) + (one_x - one_x)).norm());
    assert_eq!(one_x.metric_distance(&one_y), ((one_x - one_y) + (one_y - one_y)).norm());
    assert_eq!(one_y.metric_distance(&one_x), ((one_y - one_x) + (one_x - one_x)).norm());
    
    // linear mapping
    // identity mapping (as matrix)
    let id = Matrix2::new(
        1.0, 0.0,
        0.0, 1.0
    );
    assert_eq!(id.determinant(), one);
    assert_eq!(id.trace(), one + one);
    assert_eq!(id.try_inverse().unwrap(), id);
    // identity mapping
    assert_eq!(id * zero_vec, zero_vec);
    assert_eq!(id * one_x, one_x);
    assert_eq!(id * one_y, one_y);
    // linearity raw: additivity
    assert_eq!((id * one_x) + (id * one_y), id * (one_x + one_y));
    // linearity raw: homogeneity
    assert_eq!(one * (id * one_x), id * (one * one_x));

    
    
}