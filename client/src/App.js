import React, { useEffect, useRef, useState } from "react";
import Clips from "./Clips";
import Header from "./Header";
import Footer from "./Footer";
import { ThemeProvider } from "@material-ui/styles";
import createMuiTheme from "@material-ui/core/styles/createMuiTheme";

const theme = createMuiTheme({
  palette: {
    primary: {
      light: "#b0bec5",
      main: "#607d8b",
      dark: "#37474f",
      contrastText: "#eceff1",
    },
    secondary: {
      main: "#b0bec5",
    },
  },
  typography: {
    fontFamily: "Tahoma",
  },
});

const App = () => {
  const [clips, _setClips] = useState([]);
  const clipsRef = useRef(clips);
  const [shouldAutoPlayNext, setShouldAutoPlayNext] = useState(true);

  const setClips = (data) => {
    clipsRef.current = data;
    _setClips(data);
  };

  // null, "waiting", or url of audio clip
  const [currentlyPlaying, _setCurrentlyPlaying] = useState(null);
  const currentlyPlayingRef = useRef(currentlyPlaying);
  const setCurrentlyPlaying = (data) => {
    currentlyPlayingRef.current = data;
    console.log("set currently playing:", data)
    _setCurrentlyPlaying(data);
  };

  const handleCurrentlyPlaying = (currentItem) => {
    setCurrentlyPlaying(currentItem);
  };

  const [playClipNext, setPlayClipNext] = useState("");

  useEffect(() => {
    // TO DO: set source url for environments to get correct host
    const eventSource = new EventSource("/stream");
    let queuedData = [];
    eventSource.addEventListener("audio", (event) => {
      const data = event.data;
      const parsedData = JSON.parse(data);
      parsedData.timestamp = new Date(parsedData.timestamp);

      if (queuedData.length === 0) {
        setTimeout(() => {
          const newClips = [...clipsRef.current];
          queuedData.forEach((item) => {
            newClips.push(item);
          });

          setClips(newClips);
          queuedData = [];
        }, 100);
      }
      queuedData.push(parsedData);
      if (currentlyPlayingRef.current === "waiting") {
        setCurrentlyPlaying(parsedData.url);
      }
    });

    eventSource.onerror = () => {
      console.log("Error retrieving Audio Clips");
    };
  }, []);

  return (
    <ThemeProvider theme={theme}>
      <Header />
      <Clips
        clips={clips}
        currentlyPlaying={currentlyPlaying}
        handleCurrentlyPlaying={(currentItem) =>
          handleCurrentlyPlaying(currentItem)
        }
        newClipToPlay={playClipNext}
        shouldAutoPlayNext={shouldAutoPlayNext}
      />
      <Footer
        handleWaitingForClip={(currentItem) =>
          handleCurrentlyPlaying(currentItem)
        }
        currentlyPlaying={currentlyPlaying}
        shouldAutoPlayNext={shouldAutoPlayNext}
        setShouldAutoPlayNext={(val)=>setShouldAutoPlayNext(val)}
      />
    </ThemeProvider>
  );
};

export default App;
