//If you change it here - also change:
//1. settings/settings.ts (host change)
//2. build-utils/localcdn.js (port change)
const isLocal = process.env['release_target'] === "local"

console.log("IN CSS:", isLocal, process.env['release_target']);

const MEDIA_HOST = isLocal
	? 'http://localhost:4102'
  : "https://storage.googleapis.com/ji-cloud-eu";
    

const MEDIA_APP = MEDIA_HOST + "/app";
const MEDIA_UI = MEDIA_APP + "/ui";

module.exports = function loader(source) {
  return source
        .replace(/%MEDIA_HOST%/g, MEDIA_HOST)
        .replace(/%MEDIA_APP%/g, MEDIA_APP)
        .replace(/%MEDIA_UI%/g, MEDIA_UI);
}