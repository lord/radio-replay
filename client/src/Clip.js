import React, { useEffect, useRef } from "react";
import ListItem from "@material-ui/core/ListItem";
import ListItemText from "@material-ui/core/ListItemText";
import moment from "moment";
import ListItemAvatar from "@material-ui/core/ListItemAvatar";
import IconButton from "@material-ui/core/IconButton";
import GetAppIcon from "@material-ui/icons/GetApp";
import { getEastCoastTime } from "./helperFunctions";
import AudioPlayer from "./AudioPlayer";

const scrollToRef = (ref) => {
  window.scrollTo(0, ref.current.offsetTop);
};

const Clip = (props) => {
  const ref = useRef(null);
  const { timestamp, channel, url } = props.clip;
  const adjustedTimestamp = getEastCoastTime(timestamp);

  useEffect(() => {
    scrollToRef(ref);
  }, [props.clip]);

  return (
    <ListItem ref={ref}>
      <ListItemAvatar>
        <ListItemText
          style={{ marginRight: "30px", marginLeft: "20px" }}
          primary={moment(adjustedTimestamp).format("h:mm:ss [ET]")}
          secondary={channel}
        />
      </ListItemAvatar>
      <AudioPlayer
        url={url}
        handleCurrentlyPlaying={props.handleCurrentlyPlaying}
        currentlyPlaying={props.currentlyPlaying}
        autoPlay={props.autoPlay}
        autoPlayNew={props.autoPlayNew}
        shouldAutoPlayNext={props.shouldAutoPlayNext}
        queueNextClipToPlay={props.queueNextClipToPlay}
      />
      <IconButton href={"http://localhost:8080" + url}>
        <GetAppIcon />
      </IconButton>
    </ListItem>
  );
};

export default Clip;
