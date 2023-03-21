pub mod external;

use ext_php_rs::prelude::*;
use external::geographiclib;

/// A class representing a dummy (with no convention) geodesic point.
#[php_class]
pub struct LatLng {
    /// @var float $lat the latitude of the point.
    pub lat: f64,
    /// @var float $lng the latitude of the point.
    pub lng: f64,
}

#[php_impl]
impl LatLng {
    /// constructor.
    ///
    /// @param float $lat the latitude of the point.
    /// @param float $lng the longitude of the point.
    pub fn __construct(lat: f64, lng: f64) -> LatLng {
        LatLng { lat, lng }
    }
}

/// Calculates the geodesic area of a geodesic polygon.
///
/// Points of this polygon are given in the array as LatLng objects.
///
/// @param LatLng[] $values the points forming the polygon.
///
/// @return float the geodesic area of the polygon, in m².
#[php_function]
pub fn geod_poly_area(values: Vec<&LatLng>) -> f64 {
    let mut opaque_vec = Vec::<geographiclib::LatLng>::new();

    for v in values.iter() {
        opaque_vec.push(geographiclib::LatLng::new(v.lat, v.lng));
    }

    geographiclib::geodesic_poly_area(opaque_vec)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
