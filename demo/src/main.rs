use dioxus::prelude::*;
use dioxus_leaflet::{
    Color, LatLng, Map, MapOptions, MapPosition, Marker, PathOptions, Polygon, Polyline, Popup,
    TileLayer,
};

mod jersey;

const CSS: Asset = asset!("/assets/demo.scss");

#[component]
fn App() -> Element {
    let mut markers = use_signal(|| {
        vec![
            ("London", "Capital of the UK", LatLng::new(51.505, -0.09)),
            ("Paris", "Capital of France", LatLng::new(48.8566, 2.3522)),
            ("Berlin", "Capital of Germany", LatLng::new(52.52, 13.4)),
        ]
    });

    let route = use_signal(|| {
        vec![vec![
            LatLng::new(51.505, -0.09),
            LatLng::new(48.8566, 2.3522),
            LatLng::new(52.52, 13.4),
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
                Marker { coordinate: marker.2,
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
