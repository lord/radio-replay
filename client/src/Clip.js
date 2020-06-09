import React, { useEffect, useRef} from "react";
import ListItem from "@material-ui/core/ListItem";
import ListItemText from "@material-ui/core/ListItemText";
import moment from "moment";
import ListItemAvatar from "@material-ui/core/ListItemAvatar";
import IconButton from "@material-ui/core/IconButton";
import GetAppIcon from "@material-ui/icons/GetApp";
import { getEastCoastTime } from "./helperFunctions";
import AudioPlayer from "./AudioPlayer";
import makeStyles from "@material-ui/core/styles/makeStyles";

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

  const useStyles = makeStyles((theme) => ({
    listItem: {
      backgroundColor: props.currentlyPlaying === url ? "#cfd8dc" : "white",
      "&:hover": {
        backgroundColor: "#eceff1",
      },
    },
    listItemText: { marginRight: "30px", marginLeft: "20px" },
  }));

  const classes = useStyles();

  return (
    <ListItem ref={ref} className={classes.listItem}>
      <ListItemAvatar>
        <ListItemText
          className={classes.listItemText}
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
