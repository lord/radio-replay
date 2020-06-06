# radio-replay

interface to livestream and replay radio messages, making it slightly easier to listen to multiple mp3 radio channels at the same time or to replay hard-to-understand messages. thanks to <a href="https://github.com/syd-botz">@syd-botz</a> for making the frontend.

## installing

- install recent nodejs/npm
- [install rust](https://rustup.rs/), stable is fine
- install LAME (`sudo apt-get install libmp3lame-dev`)
- open `server/src/main.rs` and add your mp3 streams
- `cd client; npm install; npm run build`
- `cd server; cargo run --release`
- visit <http://localhost:8080/replay>

sometimes the rust server crashes and i still don't know why, so you may want to run it in a loop in production

## how to use

new non-silent audio chunks will be added to the `/replay` site as they come in. when an audio chunk is finished, the next one will automatically be played. you can press the play button next to 'livestream' at the bottom of the page to automatically play new chunks as they come in.

if the audio on a clip hangs for a second, it's just waiting for more mp3 frames to come in from the source
