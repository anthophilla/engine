use macros::VectOps;

#[test]
fn vect3() {
    #[derive(VectOps, PartialEq, Debug, Copy, Clone)]
    struct Vector3 {
        x: f32,
        y: f32,
        z: f32,
    }
    let a = Vector3{x: 1.0, z: 2.0, y: 6.5};
    let b = Vector3{x: 1.5, z: -2.0, y: 3.05};

    assert_eq!(a+b, Vector3{x: a.x+b.x, y: a.y+b.y, z: a.z+b.z});
    assert_eq!(a-b, Vector3{x: a.x-b.x, y: a.y-b.y, z: a.z-b.z});
    assert_eq!(a*b, Vector3{x: a.x*b.x, y: a.y*b.y, z: a.z*b.z});
    assert_eq!(a/b, Vector3{x: a.x/b.x, y: a.y/b.y, z: a.z/b.z});

    assert_eq!(a+0.2, Vector3{x: a.x+0.2, y: a.y+0.2, z: a.z+0.2})
}
#[test]
fn vect4() {
    #[derive(VectOps, PartialEq, Debug, Copy, Clone)]
    struct Vector4 {
        x: f32,
        y: f32,
        z: f32,
        w: f32
    }
    let a = Vector4{x: 1.0, z: 2.0, y: 6.5, w: 1.0};
    let b = Vector4{x: 1.5, z: -2.0, y: 3.05, w: 2.0};

    assert_eq!(a+b, Vector4{x: a.x+b.x, y: a.y+b.y, z: a.z+b.z, w: a.w+b.w});
    assert_eq!(a-b, Vector4{x: a.x-b.x, y: a.y-b.y, z: a.z-b.z, w: a.w-b.w});
    assert_eq!(a*b, Vector4{x: a.x*b.x, y: a.y*b.y, z: a.z*b.z, w: a.w*b.w});
    assert_eq!(a/b, Vector4{x: a.x/b.x, y: a.y/b.y, z: a.z/b.z, w: a.w/b.w});

    assert_eq!(a+0.2, Vector4{x: a.x+0.2, y: a.y+0.2, z: a.z+0.2, w: a.w+0.2})
}