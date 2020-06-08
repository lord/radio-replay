import React from "react";
import Clip from "./Clip";
import List from "@material-ui/core/List";

const Clips = (props) => {
  return (
    <List>
      {props.clips.map((clip, i) => {
        return (
          <Clip
            key={i}
            clip={clip}
            handleCurrentlyPlaying={props.handleCurrentlyPlaying}
            currentlyPlaying={props.currentlyPlaying}
          />
        );
      })}
    </List>
  );
};

export default Clips;
