import xs from 'xstream';
import type { Driver } from '@cycle/run'
import type { Stream } from 'xstream';
import { Map, View, Feature, Overlay } from 'ol';
import * as layer from 'ol/layer';
import * as source from 'ol/source';
import * as prj from 'ol/proj';
import * as controls from 'ol/control';
import * as geom from 'ol/geom';
import * as style from 'ol/style';
import { circular } from 'ol/geom/Polygon';
import type { Coordinate } from 'ol/coordinate';
// import Attribution from 'ol/control/Attribution';
export type { Coordinate } from 'ol/coordinate';

function transformWGS84toWebMercator(o: Coordinate): Coordinate {
  return prj.fromLonLat(o);
}

function transformWebMercatortoWGS84(o: Coordinate): Coordinate {
  return prj.toLonLat(o);
}

export type OlCommand = OlCommandUpdateCurrentPosition | OlCommandFocus | OlCommandAddWaypoint;
export interface OlCommandUpdateCurrentPosition {
  type: "updateCurrentPosition";
  longtiude: number;
  latitude: number;
  accuracy: number;
}
export interface OlCommandFocus {
  type: "focus";
  longitude: number;
  latitude: number;
};
export interface OlCommandAddWaypoint {
  type: "addWaypoint";
  longitude: number;
  latitude: number;
}
export interface ClickMapEvent {
  longitude: number;
  latitude: number;
}
export interface ClickGpsEvent {
}
export interface ClickPopupEvent {
}
export interface ClickAddWaypointEvent {
  longitude: number;
  latitude: number;
}
export interface OlSource {
  clickMap$: Stream<ClickMapEvent>,
  clickGps$: Stream<ClickGpsEvent>,
  clickAddWaypoint$: Stream<ClickAddWaypointEvent>,
  clickPopup$: Stream<ClickPopupEvent>,
}
export function makeOpenLayersDriver(): Driver<Stream<OlCommand>, OlSource> {

  const gpsLayer = new layer.Vector({
    source: new source.Vector(),
  });

  // const selectedPointLayer = new layer.Vector({
  //   source: new source.Vector(),
  // });


  
  const pointsLayer = new layer.Vector({
    source: new source.Vector(),
  });
  const CENTOR: Coordinate = [138.7313889, 35.3622222];
  
  const map = new Map({
    target: 'map',
    layers: [
      new layer.Tile({
        source: new source.XYZ({
          url: "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
          // EPSG:3857 は Webメルカトルのこと (google map みたいに極点を表示しない)
          projection: "EPSG:3857"
        })
      }),
      gpsLayer,
      pointsLayer
    ],
    controls: controls.defaults({
      attributionOptions: ({
        collapsible: false
      })
    }),
    view: new View({
      // Webメルカトル
      projection: "EPSG:3857",
      center: transformWGS84toWebMercator(CENTOR),
      maxZoom: 18,
      zoom: 15
    }),
    overlays: [
    ],
  });

  const clickMap$ = xs.create<ClickMapEvent>({
    start: (listener) => {
      console.log("start");
      map.on("click", (evt) => {
        const features = map.getFeaturesAtPixel(evt.pixel);
        if (features.length > 0) {
          // console.log(features);
          // (features[0] as any).setStyle(new style.Style({
          //   image: new style.Circle({
          //     radius: 10,
          //     fill: new style.Fill({ color: 'red' }),
          //     stroke: new style.Stroke({
          //       color: 'black', width: 2
          //     })
          //   })
          // }));
          // // クリックした Feature を選択中にする
          // selectedPointLayer.getSource()!.addFeatures([
          //   new Feature({
          //     type: "marker",
          //     geometry: features[0].getGeometry()! as any,
          //     style: new style.Style({
          //       image: new style.Circle({
          //         radius: 7,
          //         fill: new style.Fill({ color: 'black' }),
          //         stroke: new style.Stroke({
          //           color: 'black', width: 2
          //         })
          //       })
          //     })
          //   })
          // ]);
          // ポイントレイヤからは削除
          // pointsLayer.getSource()!.removeFeature(features[0] as any);
        } else {
          const [longitude, latitude] = transformWebMercatortoWGS84(evt.coordinate);
          listener.next({
            longitude,
            latitude,
          });

          // 選択中状態のFeatureを元に戻す
          // const currentSelectedFetures = selectedPointLayer.getSource()!.getFeatures();
          // for (const f of currentSelectedFetures) {
          //   selectedPointLayer.getSource()!.removeFeature(f);
          //   pointsLayer.getSource()!.addFeatures([
          //     new Feature({
          //       type: "marker",
          //       geometry: new geom.Point(evt.coordinate),
          //       style: new style.Style({
          //         image: new style.Circle({
          //           radius: 7,
          //           fill: new style.Fill({ color: 'red' }),
          //           stroke: new style.Stroke({
          //             color: 'black', width: 2
          //           })
          //         })
          //       })
          //     })
          //   ]);
          // }
        }
      });

    },
    stop: () => {
      console.log("stop");
    },
  });

  const popupOverlay = new Overlay({
    element: (() => {
      const elm = document.createElement('div');
      elm.classList.add("ol-popup");
      elm.appendChild(document.createElement("textarea"));
      // elm.appendChild(document.createTextNode(""));
      return elm;
    })(),
    positioning: 'bottom-center',
    stopEvent: true,
    offset: [0, 0],
    position: [0, 0],
  });
  map.addOverlay(popupOverlay);
  const clickPopup$ = xs.create<ClickPopupEvent>({
    start: (listener) => {
      const elm = popupOverlay.getElement()!;
      elm.addEventListener("click", (evt) => {
        console.log(evt);
        evt.preventDefault();
        evt.stopPropagation();
        listener.next({});
      }, true);
    },
    stop: () => {
      console.log("stop");
    },
  });

  const clickAddWaypoint$ = xs.create<ClickAddWaypointEvent>({
    start: (listener) => {
      console.log("start");
      const locate = document.createElement('div');
      locate.className = 'ol-control ol-unselectable';
      locate.innerHTML = '<button title="Locate me">🚩</button>';
      locate.style.bottom = '3em';
      locate.style.right = '.5em';
      locate.addEventListener('click', function () {
        const pos = transformWebMercatortoWGS84(map.getView()!.getCenter()!);
        listener.next({
          longitude: pos[0],
          latitude: pos[1],
        });
      });
      const currentPositionControl = new controls.Control({
        element: locate,
      });
      map.addControl(currentPositionControl);
    },
    stop: () => {
      console.log("stop");
    },
  });

  const clickGps$ = xs.create<ClickGpsEvent>({
    start: (listener) => {
      console.log("start");
      const locate = document.createElement('div');
      locate.className = 'ol-control ol-unselectable';
      locate.innerHTML = '<button title="Locate me">◎</button>';
      locate.style.bottom = '1em';
      locate.style.right = '.5em';
      locate.addEventListener('click', function () {
        listener.next({});
      });
      const currentPositionControl = new controls.Control({
        element: locate,
      });
      map.addControl(currentPositionControl);
    },
    stop: () => {
      console.log("stop");
    },
  });


  const centerCrossControl = new controls.Control({
    element: (() => {
      const locate = document.createElement('div');
      locate.className = 'ol-unselectable';
      locate.innerHTML = '+';
      locate.style.position = 'absolute';
      locate.style.top = '50%';
      locate.style.left = '50%';
      locate.style.transform = 'translate(-50%, -50%)';
      locate.style.fontSize = '24px';
      locate.style.color = "black";
      locate.style.pointerEvents = 'none';
      return locate;
    })(),
  });
  map.addControl(centerCrossControl);

  function openLayersDriver(outgoing$: Stream<OlCommand>): OlSource {
    outgoing$.addListener({
      next: (outgoing) => {
        switch (outgoing.type) {
          case "updateCurrentPosition": {
            const wgs84Coords: Coordinate = [outgoing.longtiude, outgoing.latitude];
            const accuracy = new Feature(circular(wgs84Coords, outgoing.accuracy).transform('EPSG:4326', map.getView().getProjection()))
            const src = gpsLayer.getSource()!;
            src.clear(true);
            src.addFeatures([
              accuracy,
            ]);
            break;
          }
          case "focus": {
            map.getView().setCenter(transformWGS84toWebMercator([outgoing.longitude, outgoing.latitude]));
            break;
          }
          case "addWaypoint": {
            const wgs84Coords: Coordinate = [outgoing.longitude, outgoing.latitude];
            const feat = new Feature({
              type: "marker",
              geometry: new geom.Point(transformWGS84toWebMercator((wgs84Coords))),
            });
            feat.setStyle(new style.Style({
              image: new style.Circle({
                radius: 5,
                fill: new style.Fill({ color: 'red' }),
                stroke: new style.Stroke({
                  color: 'black', width: 2
                })
              })
            }));
            pointsLayer.getSource()!.addFeatures([feat]);
            // ポップアップ表示
            const elm = popupOverlay.getElement()!;
            elm.firstChild!.textContent = `${wgs84Coords[0]}, ${wgs84Coords[1]}`;
            popupOverlay.setPosition(prj.fromLonLat(wgs84Coords));
            map.getView().setCenter(transformWGS84toWebMercator([outgoing.longitude, outgoing.latitude]));
            break;
          }
        }
      },
      error: (err) => {
        console.error(err);
      },
      complete: () => {
        console.log("complete");
      },
    });
    return {
      clickAddWaypoint$,
      clickMap$,
      clickGps$,
      clickPopup$,
    };
  }
  return openLayersDriver;
}
