import Button from '@material-ui/core/Button';
import PlayArrowIcon from '@material-ui/icons/PlayArrow';

import React, { useEffect, useRef } from "react";
const AudioPlayer = (props) => {
  const ref = useRef();
  useEffect(() => {
    if (props.currentlyPlaying === props.url) {
      // TODO check to see if this always makes it start from the beginning in weird cases?
      ref.current.play();
    }
    // if (props.currentlyPlaying !== props.url) {
    //   ref.current.pause();
    // }
  }, [props.currentlyPlaying]);

  if (props.currentlyPlaying !== props.url) {
    return (<Button
        variant="contained"
        color="primary"
        // style={{ width: "100%", outline: 0, textAlign: "left" }}
        startIcon={<PlayArrowIcon />}
        onClick={() => {
          props.handleCurrentlyPlaying(props.url);
        }}
      >Play</Button>
    )
  }

  return (
    <audio
      ref={ref}
      onPlay={() => {
        props.handleCurrentlyPlaying(props.url);
      }}
      style={{ width: "100%", outline: 0 }}
      controls
      onEnded={() => props.queueNextClipToPlay()}
      onPause={() => {
        // if (props.url === props.currentlyPlaying) {
        //   props.handleCurrentlyPlaying(null)
        // }
      }}
      id={props.url}
    >
      <source src={props.url} type="audio/mpeg" />
    </audio>
  );
};

export default AudioPlayer;
