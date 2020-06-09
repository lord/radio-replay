import React, { useEffect, useRef } from "react";
const AudioPlayer = (props) => {
  const ref = useRef();
  useEffect(() => {
    if (!(props.currentlyPlaying === props.url)) {
      ref.current.pause();
    }
  }, [props.currentlyPlaying, props.url]);

  useEffect(() => {
    const audioPlayer = document.getElementById(props.url);

    audioPlayer.addEventListener("pause", () => {
      props.handleCurrentlyPlaying(null);
    });
  }, []);

  useEffect(() => {
    if (props.autoPlayNew) {
      const audioPlayer = document.getElementById(props.url);
      audioPlayer.play();
      audioPlayer.addEventListener("ended", () => {
        props.handleCurrentlyPlaying("waiting");
      });
    }
  }, [props.autoPlayNew]);

  useEffect(() => {
    const audioPlayer = document.getElementById(props.url);
    if (props.autoPlay) {
      audioPlayer.play();
    }
    if (props.shouldAutoPlayNext) {
      audioPlayer.addEventListener("ended", () => {
        props.queueNextClipToPlay();
      });
    }
  }, [props.autoPlay, props.shouldAutoPlayNext]);
  return (
    <audio
      ref={ref}
      onPlay={() => {
        props.handleCurrentlyPlaying(props.url);
      }}
      style={{ width: "100%" }}
      controls
      id={props.url}
    >
      <source src={"http://localhost:8080" + props.url} type="audio/mpeg" />
    </audio>
  );
};

export default AudioPlayer;
