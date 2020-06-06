import React from "react";
import AppBar from "@material-ui/core/AppBar";
import { Toolbar } from "@material-ui/core";
import Typography from "@material-ui/core/Typography";
import makeStyles from "@material-ui/core/styles/makeStyles";
const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  title: {
    flexGrow: 1,
  },
}));
const Header = () => {
  const classes = useStyles();
  return (
    <div className={classes.root}>
      <Toolbar />
      <AppBar position="fixed" className={classes.appBar}>
        <Toolbar>
          <Typography variant="h5" className={classes.title}>
            NYPD Police Scanner
          </Typography>
        </Toolbar>
      </AppBar>
    </div>
  );
};
export default Header;
