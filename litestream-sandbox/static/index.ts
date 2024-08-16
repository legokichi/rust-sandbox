import Map from 'ol/Map';
import View from 'ol/View';
import TileLayer from 'ol/layer/Tile';
import XYZ from 'ol/source/XYZ';
import { fromLonLat } from 'ol/proj';
import 'ol/ol.css';

let draw;
let save;
let dibujo;
let coordinates = [];
let old_coordinates = [];
let map;
function waypointsUndo() {
    if (save == false) {
        draw.removeLastPoint();
        const lastCoord = coordinates.pop();
        drawVector.getSource().removeFeature(lastCoord);
        old_coordinates.push(lastCoord);
        drawVector.getSource().changed();
    }
}
const drawVector = new ol.layer.Vector({
    source: new ol.source.Vector({ wrapX: false }),
    style: function (feature) {
        return style[feature.get("type")] || style.line;
    },
});
function waypoints() {
    save = false;
    dibujo = true;
    map.addInteraction(draw)
    // If you want to clear the map of the previous drawing
    drawVector.getSource().clear();
    drawVector.getSource().clear();
    coordinates = [];
    old_coordinates = [];
}
window.addEventListener("DOMContentLoaded", () => {

    map = new ol.Map({
        target: "map",
        renderer: ['canvas', 'dom'],
        layers: [
            new ol.layer.Tile({
                source: new ol.source.XYZ({
                    attributions: [
                        new ol.Attribution({
                            html: "<a href='https://maps.gsi.go.jp/development/ichiran.html' target='_blank'>地理院タイル</a>"
                        })
                    ],
                    url: "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
                    projection: "EPSG:3857"
                })
            })
        ],
        controls: ol.control.defaults({
            attributionOptions: ({
                collapsible: false
            })
        }),
        view: new ol.View({
            projection: "EPSG:3857",
            center: ol.proj.transform([138.7313889, 35.3622222], "EPSG:4326", "EPSG:3857"),
            maxZoom: 18,
            zoom: 15
        })
    });
    map.on("click", function (evt) {
        console.log(evt);
        if (dibujo == true) {
            point = new ol.geom.Point(evt.coordinate);
            marker = new ol.Feature({
                type: "marker",
                geometry: point,
            });
            coordinates.push(marker);
            old_coordinates = [];
            drawVector.getSource().addFeature(marker);
        }
    });
}, false);