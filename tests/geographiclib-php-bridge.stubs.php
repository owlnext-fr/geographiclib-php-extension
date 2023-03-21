<?php

// Stubs for geographiclib-php-bridge

namespace {
    /**
     * Calculates the geodesic area of a geodesic polygon.
     *
     * Points of this polygon are given in the array as LatLng objects.
     *
     * @param LatLng[] $values the points forming the polygon.
     *
     * @return float the geodesic area of the polygon, in m².
     */
    function geod_poly_area(array $values): float {}

    /**
     * A class representing a dummy (with no convention) geodesic point.
     */
    class LatLng {
        /**
         * constructor.
         *
         * @param float $lat the latitude of the point.
         * @param float $lng the longitude of the point.
         */
        public function __construct(float $lat, float $lng) {}
    }
}
