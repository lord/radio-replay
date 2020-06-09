import React, { useEffect, useState } from "react";
import Toolbar from "@material-ui/core/Toolbar";
import AppBar from "@material-ui/core/AppBar";
import makeStyles from "@material-ui/core/styles/makeStyles";
import PlayCircleFilledIcon from "@material-ui/icons/PlayCircleFilled";
import PauseIcon from "@material-ui/icons/Pause";
import IconButton from "@material-ui/core/IconButton";
import Typography from "@material-ui/core/Typography";
import ListItemText from "@material-ui/core/ListItemText";
import moment from "moment";
import ListItemAvatar from "@material-ui/core/ListItemAvatar";
import { getEastCoastTime } from "./helperFunctions";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import Checkbox from "@material-ui/core/Checkbox";

const useStyles = makeStyles((theme) => ({
  appBar: {
    top: "auto",
    bottom: 0,
  },
  title: {
    display: "none",
    [theme.breakpoints.up("sm")]: {
      display: "block",
    },
    width: "100%",
  },
  toolbar: {
    paddingLeft: "16px !important",
  },
  menuButton: {
    paddingLeft: "22px",
    paddingRight: "25px",
  },
}));

const Footer = (props) => {
  const classes = useStyles();
  const [waiting, setWaiting] = useState(false);

  useEffect(() => {
    if (!(props.currentlyPlaying === "waiting")) {
      setWaiting(false);
    } else {
      setWaiting(true);
    }
  }, [props.currentlyPlaying]);

  const handleWaitingClick = () => {
    const isWaiting = waiting;
    setWaiting(!isWaiting);
    const currentlyPlaying = !isWaiting ? "waiting" : null;
    props.handleWaitingForClip(currentlyPlaying);
  };
  const [liveTime, setLiveTime] = useState(getEastCoastTime(new Date()));

  const updateTime = () => {
    setLiveTime(getEastCoastTime(new Date()));
  };

  useEffect(() => {
    setInterval(() => updateTime(), 1000);
  }, []);

  return (
    <div className={classes.root}>
      <Toolbar className={classes.toolbar} />
      <AppBar postion="sticky" className={classes.appBar}>
        <Toolbar className={classes.toolbar}>
          <ListItemAvatar>
            <ListItemText
              style={{ marginRight: "30px", marginLeft: "20px" }}
              primary={moment(liveTime).format("h:mm:ss [ET]")}
              secondary={moment(liveTime).format("MMM DD, YYYY")}
            />
          </ListItemAvatar>
          <IconButton
            edge={"start"}
            className={classes.menuButton}
            onClick={() => handleWaitingClick()}
          >
            {waiting ? <PauseIcon /> : <PlayCircleFilledIcon />}
          </IconButton>
          <Typography variant="h6" className={classes.title} noWrap>
            {waiting
              ? "Waiting for next audio clip..."
              : "Click play to wait for next audio clip to arrive."}
          </Typography>
          {/*<FormControlLabel*/}
          {/*  value="top"*/}
          {/*  control={*/}
          {/*    <Checkbox*/}
          {/*      color="secondary"*/}
          {/*      checked={props.shouldAutoPlayNext}*/}
          {/*      onChange={props.handleSetShouldAutoPlayNext}*/}
          {/*    />*/}
          {/*  }*/}
          {/*  label={"autoplay"}*/}
          {/*  labelPlacement="start"*/}
          {/*/>*/}
        </Toolbar>
      </AppBar>
    </div>
  );
};

export default Footer;
