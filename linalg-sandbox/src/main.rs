use ndarray::prelude::*;
use ndarray_linalg::solve::Determinant;
use ndarray_linalg::eigh::Eigh;
use ndarray_linalg::trace::Trace;
use ndarray_linalg::eigh::EigValsh;
use ndarray_linalg::lapack::UPLO;
use plotters::prelude::*;
use ndarray_linalg::solve::Inverse;

fn diagonal_matrix(){
    // 対角行列(角度を保った拡縮)
    {
        // 対角行列
        // Stretching 伸縮変換
        // +----+    +------+
        // |    | => |      |
        // +----+    +------+
        // x 軸方向に 1.5 倍
        let a = array![
            [1.5, 0.0],
            [0.0, 1.0]
        ];
        // 次元は潰れないので rank A = 2 で 正則行列
        // 面積は 1.5 倍
        // 対角行列では対角成分の積が行列式
        assert_eq!(a.det().unwrap(), 1.5);
        // 逆行列
        assert_eq!(a.inv().unwrap(), array![
            [1.0/1.5, 0.0],
            [0.0,  1.0]
        ]);
        // 単位ベクトルの変換先
        assert_eq!(a.dot(&array![[1.0, 0.0]].t()), array![[1.5, 0.0]].t());
        assert_eq!(a.dot(&array![[0.0, 1.0]].t()), array![[0.0, 1.0]].t());
        // 固有ベクトルとその固有値は
        // [0.0, 1.0] (y軸)方向に 1.0 倍
        // [1.0, 0.0] (x軸)方向に 1.5 倍
        // ※対角行列では対角成分が固有値
        let (eigval, eigvec) = a.eigh(UPLO::Upper).unwrap();
        assert_eq!(eigval, array![1.0, 1.5]);
        assert_eq!(eigvec, array![[0.0, 1.0], [1.0, 0.0]]);
        // A * 固有ベクトル = 固有値 * 固有ベクトル (固有ベクトル != 0)
        assert_eq!(
            a.dot(&eigvec.index_axis(Axis(0), 0)),
            eigval[0] * &eigvec.index_axis(Axis(0), 0)
        );
        assert_eq!(
            a.dot(&eigvec.index_axis(Axis(0), 1)),
            eigval[1] * &eigvec.index_axis(Axis(0), 1)
        );
        // 対角和は固有値和
        assert_eq!(a.trace().unwrap(), 2.5);
        assert_eq!(eigval.sum(), 2.5);
    }
    {
        // 対角行列
        // y 軸方向に -1 倍、つまり y 軸方向に裏返る
        // +----+
        // |    |
        // +----+ => +----+
        //           |    |
        //           +----+
        let a = array![
            [1.0, 0.0],
            [0.0, -1.0]
        ];
        // 次元は潰れないので rank A = 2 で 正則行列
        // 面積は -1.0 倍
        // 対角行列では対角成分の積が行列式
        assert_eq!(a.det().unwrap(), -1.0);
        // 逆行列
        assert_eq!(a.inv().unwrap(), array![
            [1.0, 0.0],
            [0.0, -1.0]
        ]);
        // 単位ベクトルの変換先
        assert_eq!(a.dot(&array![[1.0, 0.0]].t()), array![[1.0, 0.0]].t());
        assert_eq!(a.dot(&array![[0.0, 1.0]].t()), array![[0.0, -1.0]].t());
        // 固有ベクトルとその固有値は
        // [0.0, 1.0] (y軸)方向に -1.0 倍
        // [1.0, 0.0] (x軸)方向に 1.0 倍
        // ※対角行列では対角成分が固有値
        let (eigval, eigvec) = a.eigh(UPLO::Upper).unwrap();
        assert_eq!(eigval, array![-1.0, 1.0]);
        assert_eq!(eigvec, array![[0.0, 1.0], [1.0, 0.0]]);
        // A * 固有ベクトル = 固有値 * 固有ベクトル (固有ベクトル != 0)
        for (eigval, eigvec) in eigval.iter().zip(eigvec.axis_iter(Axis(0))) {
            assert_eq!(
                a.dot(&eigvec),
                *eigval * &eigvec
            );
        }
        // 対角和は固有値和
        assert_eq!(a.trace().unwrap(), 0.0);
        assert_eq!(eigval.sum(), 0.0);
    }
    {
        // 対角行列
        // y 軸方向の次元が潰れる
        // +----+
        // |    |
        // +----+ => +----+
        let a = array![
            [1.0, 0.0],
            [0.0, 0.0]
        ];
        // 次元が潰れるので rank A = 1 で 特異行列
        // 面積は 0.0 倍
        // 対角行列では対角成分の積が行列式
        assert_eq!(a.det().unwrap(), 0.0);
        // 逆行列は存在しない
        assert!(a.inv().is_err());
        // 単位ベクトルの変換先
        assert_eq!(a.dot(&array![[1.0, 0.0]].t()), array![[1.0, 0.0]].t());
        assert_eq!(a.dot(&array![[0.0, 1.0]].t()), array![[0.0, 0.0]].t());
        // 固有ベクトルとその固有値は
        // [0.0, 1.0] (y軸)方向に 0.0 倍
        // [1.0, 0.0] (x軸)方向に 1.0 倍
        // ※対角行列では対角成分が固有値
        let (eigval, eigvec) = a.eigh(UPLO::Upper).unwrap();
        assert_eq!(eigval, array![0.0, 1.0]);
        assert_eq!(eigvec, array![[0.0, 1.0], [1.0, 0.0]]);
        // A * 固有ベクトル = 固有値 * 固有ベクトル (固有ベクトル != 0)
        for (eigval, eigvec) in eigval.iter().zip(eigvec.axis_iter(Axis(0))) {
            assert_eq!(
                a.dot(&eigvec),
                *eigval * &eigvec
            );
        }
        // 対角和は固有値和
        assert_eq!(a.trace().unwrap(), 1.0);
        assert_eq!(eigval.sum(), 1.0);
    }
    {
        // 対角行列
        // Squeezing mapping
        // 面積を保つ Stretching
        // +----+
        // |    |    +--------+
        // +----+ => +--------+
        let a = array![
            [2.0, 0.0],
            [0.0, 1.0/2.0]
        ];
        // 逆行列
        assert_eq!(a.inv().unwrap(), array![
            [1.0/2.0, 0.0],
            [0.0, 2.0]
        ]);
        // 固有値
        // ※対角行列では対角成分が固有値
        let eigval = a.eigvalsh(UPLO::Upper).unwrap();
        assert_eq!(eigval, array![0.5, 2.0]);
        // 対角和は固有値和
        assert_eq!(a.trace().unwrap(), 5.0/2.0);
        assert_eq!(eigval.sum(), 5.0/2.0);
    }
}
fn shear_mapping(){
    // 剪断写像(菱形への変換)
    {
        // 水平剪断写像
        // 底辺と高さは変わらないので面積同じ
        // +----+      +----+
        // |    | =>  /    / 45 deg
        // +----+    +----+
        let a = array![
            [1.0, 1.0],
            [0.0, 1.0]
        ];
        // 次元は潰れないので rank A = 2 で 正則行列
        // 面積は 1.0 倍
        // 三角行列では対角成分の積が行列式
        assert_eq!(a.det().unwrap(), 1.0);
        // 逆行列
        assert_eq!(a.inv().unwrap(), array![
            [1.0, -1.0],
            [0.0, 1.0]
        ]);
        // 単位ベクトルの変換先
        assert_eq!(a.dot(&array![[1.0, 0.0]].t()), array![[1.0, 0.0]].t());
        assert_eq!(a.dot(&array![[0.0, 1.0]].t()), array![[1.0, 1.0]].t());
        // 固有ベクトルとその固有値は
        // [1.0, 0.0] (x軸)方向に 1.0 倍
        // ※対角行列と三角行列では対角成分が固有値
        let eigval = 1.0;
        let eigvec = array![1.0, 0.0];
        // A * 固有ベクトル = 固有値 * 固有ベクトル (固有ベクトル != 0)
        assert_eq!(
            a.dot(&eigvec),
            eigval * &eigvec
        );
        // 固有値は重根
        let eigval = a.eigvalsh(UPLO::Lower).unwrap();
        assert_eq!(eigval, array![1.0, 1.0]);
        // 対角和は固有値和
        assert_eq!(a.trace().unwrap(), 2.0);
        assert_eq!(eigval.sum(), 2.0);
    }
    {
        // 鉛直剪断写像
        // 面積同じ
        //             +
        //            /|
        // +----+    + + 
        // |    | => |/ 45 deg
        // +----+    +
        let a = array![
            [1.0, 0.0],
            [1.0, 1.0]
        ];
        // 次元は潰れないので rank A = 2 で 正則行列
        // 面積は 1.0 倍
        // 三角行列では対角成分の積が行列式
        assert_eq!(a.det().unwrap(), 1.0);
        // 逆行列
        assert_eq!(a.inv().unwrap(), array![
            [1.0, 0.0],
            [-1.0, 1.0]
        ]);
        // 単位ベクトルの変換先
        assert_eq!(a.dot(&array![[1.0, 0.0]].t()), array![[1.0, 1.0]].t());
        assert_eq!(a.dot(&array![[0.0, 1.0]].t()), array![[0.0, 1.0]].t());
        // 固有ベクトルとその固有値は
        // [0.0, 1.0] (x軸)方向に 1.0 倍
        // ※対角行列と三角行列では対角成分が固有値
        let eigval = 1.0;
        let eigvec = array![0.0, 1.0];
        // A * 固有ベクトル = 固有値 * 固有ベクトル (固有ベクトル != 0)
        assert_eq!(
            a.dot(&eigvec),
            eigval * &eigvec
        );
        // 固有値は重根
        let eigval = a.eigvalsh(UPLO::Upper).unwrap();
        assert_eq!(eigval, array![1.0, 1.0]);
        // 対角和は固有値和
        assert_eq!(a.trace().unwrap(), 2.0);
        assert_eq!(eigval.sum(), 2.0);
    }

}

fn rotation_matrix(){
    // 回転写像(回転行列)
    let x = std::f64::consts::PI/4.0; // 45 deg
    let a = array![
        [x.cos(), -x.sin()],
        [x.sin(), x.cos()]
    ];
    // 次元は潰れないので rank A = 2 で 正則行列
    // 面積は 1.0 倍
    assert_eq!(a.det().unwrap(), 1.0);
    // 逆行列は転置
    assert_eq!(a.inv().unwrap(), array![
        [x.cos(), x.sin()],
        [-x.sin(), x.cos()]
    ]);
    // 単位ベクトルの変換先
    assert_eq!(a.dot(&array![[1.0, 0.0]].t()), array![[x.cos(), x.sin()]].t());
    assert_eq!(a.dot(&array![[0.0, 1.0]].t()), array![[-x.sin(), x.cos()]].t());
    // 固有ベクトルとその固有値は
    // (x,y平面上には)存在しない
    // x,y,z 上では z 軸ともみなせる: https://ishikawash.hatenadiary.org/entry/20080712/1215838044
    // 対角和
    assert_eq!(a.trace().unwrap(), x.cos() * 2.0);
}

fn general_linear_mapping(){
    // 一般の線形変換
    let a = array![
        [1.0, -0.3],
        [-0.7, 0.6]
    ];
    // 面積は 1.0 倍 で次元は潰れないので正則行列
    assert_eq!(a.det().unwrap(), 0.39);
    // 単位ベクトルの変換先
    assert_eq!(a.dot(&array![[1.0, 0.0]].t()), array![[1.0, -0.7]].t());
    assert_eq!(a.dot(&array![[0.0, 1.0]].t()), array![[-0.3, 0.6]].t());
    // 固有ベクトルとその固有値
    let eigval1 = 3.0/10.0;
    let eigvec1 = array![3.0/7.0, 1.0];
    assert_eq!(a.dot(&eigvec1), eigval1 * &eigvec1);
    let eigval2 = 13.0/10.0;
    let eigvec2 = array![-1.0, 1.0];
    // assert_eq!(a.dot(&eigvec2), eigval2 * &eigvec2); // 誤差で通らない
    // 対角和は固有値和
    assert_eq!(a.trace().unwrap(), 1.6);
    assert_eq!(eigval1 + eigval2, 1.6);
    // 特性方程式の導出
    // 固有値の性質の変形
    assert_eq!((array![
        [eigval1, 0.0],
        [0.0, eigval1]
    ] - &a).dot(&eigvec1), array![0.0,0.0]);
    // から導出
    // 特性方程式
    assert_eq!((array![
        [eigval1, 0.0],
        [0.0, eigval1]
    ] - &a).det().unwrap(), 0.0);
}

fn main() {
    // 線形変換
    diagonal_matrix();
    shear_mapping();
    rotation_matrix();
    general_linear_mapping();
}

fn elementary_matrix(){
    let p = array![
        [0.0, 1.0],
        [1.0, 0.0]
    ];
    // p*x   すると i 行と j 行の交換
    //   x*p すると i 列と j 列の交換
    let c = 2.0;
    let q = array![
        //    i 列
        [1.0, 0.0],
        [0.0, c] // i 行
    ];
    // q*x   すると i 行が c 倍
    //   x*q すると i 列が c 倍
    let r = array![
        //    j 列
        [1.0, c], // i 行
        [0.0, 1.0]
    ];
    // r*x   すると i 行に j 行の c 倍
    //   x*r すると i 列に j 列の c 倍

}