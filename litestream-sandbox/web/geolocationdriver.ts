import xs from 'xstream';
import type { Driver } from '@cycle/run'
import type { Stream } from 'xstream';

export interface GeoLocation {
  longtiude: number;
  latitude: number;
  accuracy: number;
}
export interface GeoSource {
  pos$: Stream<GeoLocation>
}
export type GeoCommand = never;

export function makeGeoLocationDriver(): Driver<Stream<GeoCommand>, GeoSource> {
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
