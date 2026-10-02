use dioxus::prelude::*;
use dioxus_leaflet::{
    Color, LatLng, Map, MapOptions, MapPosition, Marker, MarkerIcon, PathOptions, Polyline, Popup
};

mod jersey;

const CSS: Asset = asset!("/assets/demo.scss");
const STAR: Asset = asset!("/assets/star.png");
const CIRCLE: Asset = asset!("/assets/circle.png");

#[component]
fn App() -> Element {
    let mut marker_star = MarkerIcon::new(STAR);
    marker_star.icon_size = Some((24, 24));
    let mut marker_circle = MarkerIcon::new(CIRCLE);
    marker_circle.icon_size = Some((24, 24));

    let markers = use_signal(|| {
        vec![
            ("London", "Capital of the UK", LatLng::new(51.505, -0.09), marker_star.clone()),
            ("Paris", "Capital of France", LatLng::new(48.8566, 2.3522), marker_circle.clone()),
            ("Berlin", "Capital of Germany", LatLng::new(52.52, 13.4), marker_star.clone()),
            ("La Rochelle", "Some place at the coast", LatLng::new(46.153, -1.1375), marker_star.clone()),
        ]
    });

    let route = use_signal(|| {
        vec![vec![
            LatLng::new(51.505, -0.09),
            LatLng::new(51.124, 1.312),
            LatLng::new(50.958, 1.753),
            LatLng::new(48.8566, 2.3522),
        ],vec![
            LatLng::new(48.8566, 2.3522),
            LatLng::new(50.1100, 8.665),
            LatLng::new(52.52, 13.4),
        ]
        ,vec![
            LatLng::new(48.8566, 2.3522),
            LatLng::new(46.153, -1.1375),
        ]]
    });

    let options = use_signal(|| MapOptions::default());

    rsx! {
        document::Style { href: CSS }
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            options: options(),
            on_click: move |pos: LatLng| {
                info!("Map clicked at: {:?}", pos);
            },
            on_move: move |pos: MapPosition| {
                info!("Map moved to: {:?}", pos);
            },
            for marker in markers() {
                Marker {
                    coordinate: marker.2,
                    icon: marker.3,
                    on_click: move |evt| {
                        info!("Marker clicked, {}", marker.0);
                    },
                    Popup {
                        b { "{marker.0}" }
                        br {}
                        "{marker.1}"
                    }
                }
            }
            Polyline {
                coordinates: route,
                options: PathOptions {
                    color: Color::new([0., 0., 0.]),
                    weight: 5,
                    fill: false,
                    ..Default::default()
                },
                Popup { "Route connecting capitals" }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
