import './style.css';
import { Map, View, Feature, Overlay } from 'ol';
import * as layer from 'ol/layer';
import * as source from 'ol/source';
import * as prj from 'ol/proj';
import * as controls from 'ol/control';
import * as geom from 'ol/geom';
import * as style from 'ol/style';
import { circular } from 'ol/geom/Polygon';
import { Coordinate } from 'ol/coordinate';
import xs from 'xstream';
import { Stream } from 'xstream';
import { run, Driver } from '@cycle/run'
// import Attribution from 'ol/control/Attribution';


interface GeoLocation {
  longtiude: number;
  latitude: number;
  accuracy: number;
}
interface GeoSource {
  pos$: Stream<GeoLocation>
}
type GeoCommand = never;

function makeGeoLocationDriver(): Driver<Stream<GeoCommand>, GeoSource> {
  let watchId: number | null = null;
  function geolocationDriver(outgoing$: Stream<GeoCommand>) {
    const pos$ = xs.create<GeoLocation>({
      start: (listener) => {
        console.log("start");
        watchId = navigator.geolocation.watchPosition(
          (pos) => {
            console.log(pos);
            pos.coords.accuracy
            listener.next({
              longtiude: pos.coords.longitude,
              latitude: pos.coords.latitude,
              accuracy: pos.coords.accuracy
            });
          },
          (err) => {
            console.error(err);
          },
          {
            enableHighAccuracy: true,
          }
        );
        outgoing$.addListener({
          next: () => { console.log("next"); },
          error: (err) => { console.error(err); },
          complete: () => { console.log("complete"); },
        });
      },
      stop: () => {
        console.log("stop");
        if (watchId !== null) {
          navigator.geolocation.clearWatch(watchId);
          watchId = null;
        }
      },
    });
    return {
      pos$: pos$,
    };
  }
  return geolocationDriver;
}

function transformWGS84toWebMercator(o: Coordinate): Coordinate {
  return prj.fromLonLat(o);
}

function transformWebMercatortoWGS84(o: Coordinate): Coordinate {
  return prj.toLonLat(o);
}

type OlCommand = OlCommandUpdateCurrentPosition | OlCommandFocus | OlCommandAddWaypoint;
interface OlCommandUpdateCurrentPosition {
  type: "updateCurrentPosition";
  longtiude: number;
  latitude: number;
  accuracy: number;
}
interface OlCommandFocus {
  type: "focus";
  longitude: number;
  latitude: number;
};
interface OlCommandAddWaypoint {
  type: "addWaypoint";
  longitude: number;
  latitude: number;
}
interface ClickMapEvent {
  longitude: number;
  latitude: number;
}
interface ClickGpsEvent {
}
interface ClickAddWaypointEvent {
  longitude: number;
  latitude: number;
}
interface OlSource {
  clickMap$: Stream<ClickMapEvent>,
  clickGps$: Stream<ClickGpsEvent>,
  clickAddWaypoint$: Stream<ClickAddWaypointEvent>,
}
function makeOpenLayersDriver(): Driver<Stream<OlCommand>, OlSource> {

  const popupOverlay = new Overlay({
    element: (() => {
      const elm = document.createElement('div');
      elm.classList.add("ol-popup");
      elm.addEventListener("click", (evt) => {
        console.log(evt);
        evt.preventDefault();
        evt.stopPropagation();
      }, true);
      elm.appendChild(document.createTextNode(""));
      return elm;
    })(),
    positioning: 'bottom-center',
    stopEvent: false,
    offset: [0, 0],
    position: [0, 0],
  });

  const gpsLayer = new layer.Vector({
    source: new source.Vector(),
  });

  // const pointsLayer = new layer.Vector({
  //   source: new source.Vector(),
  // });

  // const selectedPointLayer = new layer.Vector({
  //   source: new source.Vector(),
  // });


  // EPSG:4326 (WGS 84 の EPSGコード) 座標系での富士山の位置
  const FUJI: Coordinate = [138.7313889, 35.3622222];

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
      // pointsLayer
    ],
    controls: controls.defaults({
      attributionOptions: ({
        collapsible: false
      })
    }),
    view: new View({
      // Webメルカトル
      projection: "EPSG:3857",
      center: transformWGS84toWebMercator(FUJI),
      maxZoom: 18,
      zoom: 15
    }),
    overlays: [
      popupOverlay
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

          // // ポイントを追加
          // const wgs84Coords = transformWebMercatortoWGS84(evt.coordinate);
          // const feat = new Feature({
          //   type: "marker",
          //   geometry: new geom.Point(evt.coordinate),
          // });
          // feat.setStyle(new style.Style({
          //   image: new style.Circle({
          //     radius: 10,
          //     fill: new style.Fill({ color: 'red' }),
          //     stroke: new style.Stroke({
          //       color: 'black', width: 2
          //     })
          //   })
          // }));
          // pointsLayer.getSource()!.addFeatures([
          //   feat
          // ]);

          // // ポップアップ表示
          // const elm = popupOverlay.getElement()!;
          // elm.firstChild!.textContent = `東経: ${wgs84Coords[0]}, 北緯: ${wgs84Coords[1]}`;
          // popupOverlay.setPosition(evt.coordinate);
        }
      });

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
      locate.style.top = '4em';
      locate.style.left = '.5em';
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
      locate.style.top = '6em';
      locate.style.left = '.5em';
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

  return function openLayersDriver(outgoing$: Stream<OlCommand>): OlSource {
    outgoing$.addListener({
      next: (outgoing) => {
        switch (outgoing.type) {
          case "updateCurrentPosition": {
            const wgs84Coords: Coordinate = [outgoing.longtiude, outgoing.latitude];
            const accuracy = new Feature(circular(wgs84Coords, outgoing.accuracy).transform('EPSG:4326', map.getView().getProjection()))
            // const centor = new Feature(new geom.Point(prj.fromLonLat(wgs84Coords)));
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
              geometry: new geom.Point(prj.fromLonLat(wgs84Coords)),
            });
            feat.setStyle(new style.Style({
              image: new style.Circle({
                radius: 10,
                fill: new style.Fill({ color: 'red' }),
                stroke: new style.Stroke({
                  color: 'black', width: 2
                })
              })
            }));
            gpsLayer.getSource()!.addFeature(feat);
            // ポップアップ表示
            const elm = popupOverlay.getElement()!;
            elm.firstChild!.textContent = `東経: ${wgs84Coords[0]}, 北緯: ${wgs84Coords[1]}`;
            popupOverlay.setPosition(prj.fromLonLat(wgs84Coords));
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
    };
  }
}

interface Sources {
  OL: OlSource;
  GEO: GeoSource;
}

interface Sinks {
  OL: Stream<OlCommand>;
  GEO: Stream<GeoCommand>;
}

function main(o: Sources): Sinks {
  const {
    OL: { clickGps$, clickMap$, clickAddWaypoint$ },
    GEO: { pos$ },
  } = o;
  const ol$1: Stream<OlCommand> = xs.combine(clickGps$, pos$).map(
    ([click, pos]): OlCommand => {
      return {
        type: "focus",
        longitude: pos.longtiude,
        latitude: pos.latitude,
      };
    }
  );
  const ol$2: Stream<OlCommand> = pos$.map(({
    longtiude,
    latitude,
    accuracy
  }): OlCommand => {
    return {
      type: "updateCurrentPosition",
      longtiude,
      latitude,
      accuracy
    };
  });
  const ol$3: Stream<OlCommand> = clickAddWaypoint$.map(({
    longitude,
    latitude
  }): OlCommand => {
    return {
      type: "addWaypoint",
      longitude,
      latitude,
    };
  });
  return {
    OL: xs.merge(ol$1, ol$2, ol$3),
    GEO: xs.never(),
  };
}



run(main, {
  GEO: makeGeoLocationDriver(),
  OL: makeOpenLayersDriver(),
});


