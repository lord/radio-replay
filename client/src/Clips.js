import React, { useEffect, useState } from "react";
import Clip from "./Clip";
import List from "@material-ui/core/List";

const Clips = (props) => {
  const queueNextClipToPlay = (lastPlayedClipId) => {
    if (props.shouldAutoPlayNext) {
      const nextClipToPlay = lastPlayedClipId + 1;
      if (nextClipToPlay + 1 > props.clips.length) {
        props.handleCurrentlyPlaying("waiting");
      } else {
        props.handleCurrentlyPlaying(props.clips[nextClipToPlay].url);
      }
    }
    if (!props.shouldAutoPlayNext) {
      props.handleCurrentlyPlaying(null);
    }
  };

  return (
    <List style={{paddingBottom:"0px"}}>
      {props.clips.map((clip, i) => {
        return (
          <Clip
            key={i}
            clip={clip}
            handleCurrentlyPlaying={props.handleCurrentlyPlaying}
            currentlyPlaying={props.currentlyPlaying}
            queueNextClipToPlay={() => queueNextClipToPlay(i)}
          />
        );
      })}
    </List>
  );
};

export default Clips;
