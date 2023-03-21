#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// internal representation of a dummy geographic point as a Latitude/Longitude pair.
/// it is called dummy becauses it does not specify it's geodesic system.
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

/// calculates the geodesic polygonal area of an area given
/// a vector of Latitude/Longitude points.
pub fn geodesic_poly_area(points: Vec<LatLng>) -> f64 {
    unsafe {
        // equatorial radius
        let a = 6378137_f64;
        // flattening factor
        let f = 1.0 / 298.257223563;

        let mut area: f64 = 0.0;
        let mut perimeter: f64 = 0.0;

        let mut g: ::std::mem::MaybeUninit<geod_geodesic> = ::std::mem::MaybeUninit::uninit();
        let mut p: ::std::mem::MaybeUninit<geod_polygon> = ::std::mem::MaybeUninit::uninit();

        geod_init(g.as_mut_ptr(), a, f);
        geod_polygon_init(p.as_mut_ptr(), 0);

        for pt in points.iter() {
            let lat = pt.lat;
            let lng = pt.lng;
            geod_polygon_addpoint(g.as_mut_ptr(), p.as_mut_ptr(), lat, lng);
        }

        geod_polygon_compute(
            g.as_mut_ptr(),
            p.as_mut_ptr(),
            0,
            1,
            &mut area,
            &mut perimeter,
        );

        area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// tests that a given array of Lat/Lng points are calculating a correct
    /// geodesic polygon area.
    ///
    /// you can find the calculation proof here : https://geographiclib.sourceforge.io/cgi-bin/Planimeter?type=polygon&rhumb=geodesic&radius=6378137&flattening=1%2F298.257223563&input=48.07489520626800000+2.58828990630500000%0D%0A48.07482912520600000+2.58831611823900000%0D%0A48.07310904704400000+2.59433615095660000%0D%0A48.07315709280000000+2.59446012530510000%0D%0A48.07375730721600000+2.59477469528160000%0D%0A48.07360094978200000+2.59544436092220000%0D%0A48.07363881470900000+2.59552214062500000%0D%0A48.08045202088900000+2.59954689923130000%0D%0A48.08047751577800000+2.59951739332570000%0D%0A48.08132143969000000+2.59254629899750000%0D%0A48.08128443751700000+2.59247676381690000%0D%0A48.07489520626800000+2.58828990630500000&norm=decdegrees&option=Submit
    #[test]
    fn geodesic_polygon_area() {
        #[allow(clippy::excessive_precision)]
        let vec = vec![
            LatLng::new(48.07489520626800000, 2.58828990630500000),
            LatLng::new(48.07482912520600000, 2.58831611823900000),
            LatLng::new(48.07310904704400000, 2.59433615095660000),
            LatLng::new(48.07315709280000000, 2.59446012530510000),
            LatLng::new(48.07375730721600000, 2.59477469528160000),
            LatLng::new(48.07360094978200000, 2.59544436092220000),
            LatLng::new(48.07363881470900000, 2.59552214062500000),
            LatLng::new(48.08045202088900000, 2.59954689923130000),
            LatLng::new(48.08047751577800000, 2.59951739332570000),
            LatLng::new(48.08132143969000000, 2.59254629899750000),
            LatLng::new(48.08128443751700000, 2.59247676381690000),
            LatLng::new(48.07489520626800000, 2.58828990630500000),
        ];

        let area = geodesic_poly_area(vec);
        assert_eq!(450290.8423774112, area);
    }
}
