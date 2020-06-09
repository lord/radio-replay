import React, { useEffect, useState } from "react";
import Clip from "./Clip";
import List from "@material-ui/core/List";

const Clips = (props) => {
  const [nextClipToPlay, setNextClipToPlay] = useState(null);

  const queueNextClipToPlay = (lastPlayedClipId) => {
    const nextClipToPlay = lastPlayedClipId + 1;
    if (nextClipToPlay + 1 > props.clips.length) {
      props.handleCurrentlyPlaying("waiting");
    } else {
      setNextClipToPlay(nextClipToPlay);
    }
  };

  useEffect(() => {
    if (!props.shouldAutoPlayNext) {
      setNextClipToPlay(null);
    }
  }, [props.shouldAutoPlayNext, props.clips]);

  return (
    <List>
      {props.clips.map((clip, i) => {
        let autoPlayNew = false;
        let autoPlay = false;
        if (props.newClipToPlay === clip.url) {
          autoPlayNew = true;
        }
        if ((nextClipToPlay === i) & props.shouldAutoPlayNext) {
          autoPlay = true;
        }
        return (
          <Clip
            key={i}
            clip={clip}
            handleCurrentlyPlaying={props.handleCurrentlyPlaying}
            currentlyPlaying={props.currentlyPlaying}
            autoPlay={autoPlay}
            autoPlayNew={autoPlayNew}
            shouldAutoPlayNext={props.shouldAutoPlayNext}
            queueNextClipToPlay={() => queueNextClipToPlay(i)}
          />
        );
      })}
    </List>
  );
};

export default Clips;
