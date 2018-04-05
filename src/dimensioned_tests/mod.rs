//use dimensioned::traits::Root;
//
////TODO Testable trait that returns an iterator of Self? as well as new()
//
//#[test]
//fn test_root() {
//    use dimensioned::typenum::consts::*;
//    let radicands = &[0.0f32, 0.5, 1.0, 2.0];
//
//    for &r in radica    nds {
//        assert_eq!(r, r.root(P1::new()));
//        assert_eq!(r, (r * r).root(P2::new()));
//        assert_eq!(r, (r * r * r).root(P3::new()));
//        assert_eq!(r, (r * r * r * r * r).root(P5::new()));
//    }
//}