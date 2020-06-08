import React, { useEffect, useRef } from "react";
const AudioPlayer = (props) => {
  const ref = useRef();
  useEffect(() => {
    if (!(props.currentlyPlaying === props.url)) {
      ref.current.pause();
    }
  }, [props.currentlyPlaying, props.url]);

  return (
    <audio
      ref={ref}
      onPlay={() => {
        props.handleCurrentlyPlaying(props.url);
      }}
      style={{ width: "100%" }}
      controls
    >
      <source
        id={props.url}
        src={"http://localhost:8080" + props.url}
        type="audio/mpeg"
      />
    </audio>
  );
};

export default AudioPlayer;
